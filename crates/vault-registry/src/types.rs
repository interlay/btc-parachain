use crate::{ext, Config, Error, Pallet, PoolManager};
use codec::{Decode, Encode, HasCompact, MaxEncodedLen};
use currency::Amount;
use frame_support::{
    dispatch::{DispatchError, DispatchResult},
    ensure,
    traits::Get,
    Blake2_128Concat,
};
pub use primitives::{VaultCurrencyPair, VaultId};
use scale_info::TypeInfo;
use sp_core::H256;
use sp_runtime::{
    traits::{CheckedAdd, CheckedSub, Zero},
    ArithmeticError,
};

#[cfg(test)]
use mocktopus::macros::mockable;

pub use bitcoin::{Address as BtcAddress, PublicKey as BtcPublicKey};

/// Storage version.
#[derive(Debug, Encode, Decode, Eq, PartialEq, TypeInfo, MaxEncodedLen)]
pub enum Version {
    /// Initial version.
    V0,
    /// BtcAddress type with script format.
    V1,
    /// added replace_collateral to vault, changed vaultStatus enum
    V2,
    /// moved public_key out of the vault struct
    V3,
    /// Fixed liquidation vault
    V4,
    /// Added custom pervault secure collateral threshold
    V5,
    /// Removed wallet
    V6,
    /// Removed replace pallet fields
    /// Following fields have been removed from vault
    /// `to_be_replaced_tokens`, `replace_collateral` and `active_replace_collateral`
    V7,
}

#[derive(Debug, PartialEq)]
pub enum CurrencySource<T: frame_system::Config + orml_tokens::Config> {
    /// Used by vault to back issued tokens
    Collateral(DefaultVaultId<T>),
    /// User's issue griefing collateral
    UserGriefing(<T as frame_system::Config>::AccountId),
    /// Unlocked balance
    FreeBalance(<T as frame_system::Config>::AccountId),
    /// Locked balance (like collateral but doesn't slash)
    LiquidatedCollateral(DefaultVaultId<T>),
    /// Funds within the liquidation vault
    LiquidationVault(DefaultVaultCurrencyPair<T>),
}

#[cfg_attr(test, mockable)]
impl<T: Config> CurrencySource<T> {
    pub fn account_id(&self) -> <T as frame_system::Config>::AccountId {
        match self {
            CurrencySource::Collateral(DefaultVaultId::<T> { account_id: x, .. })
            | CurrencySource::UserGriefing(x)
            | CurrencySource::FreeBalance(x)
            | CurrencySource::LiquidatedCollateral(DefaultVaultId::<T> { account_id: x, .. }) => x.clone(),
            CurrencySource::LiquidationVault(_) => Pallet::<T>::liquidation_vault_account_id(),
        }
    }

    pub fn current_balance(&self, currency_id: CurrencyId<T>) -> Result<crate::Amount<T>, DispatchError> {
        let amount = match self {
            CurrencySource::Collateral(vault_id) => Pallet::<T>::get_backing_collateral(vault_id)?,
            CurrencySource::UserGriefing(x) => ext::currency::get_reserved_balance::<T>(currency_id, x),
            CurrencySource::FreeBalance(x) => ext::currency::get_free_balance::<T>(currency_id, x),
            CurrencySource::LiquidatedCollateral(vault_id) => {
                let vault = Pallet::<T>::get_vault_from_id(vault_id)?;
                Amount::new(vault.liquidated_collateral, vault_id.collateral_currency())
            }
            CurrencySource::LiquidationVault(currency_pair) => {
                let liquidation_vault = Pallet::<T>::get_liquidation_vault(&currency_pair);
                Amount::new(liquidation_vault.collateral, currency_pair.collateral)
            }
        };
        Ok(amount)
    }
}

pub(crate) type BalanceOf<T> = <T as currency::Config>::Balance;

pub(crate) type UnsignedFixedPoint<T> = <T as currency::Config>::UnsignedFixedPoint;

pub type CurrencyId<T> = <T as orml_tokens::Config>::CurrencyId;

pub type DefaultVaultId<T> = VaultId<<T as frame_system::Config>::AccountId, CurrencyId<T>>;

pub type DefaultVaultCurrencyPair<T> = VaultCurrencyPair<CurrencyId<T>>;

mod v6 {
    use super::*;
    pub type DefaultVault<T> = Vault<
        <T as frame_system::Config>::AccountId,
        <T as frame_system::Config>::BlockNumber,
        BalanceOf<T>,
        CurrencyId<T>,
        UnsignedFixedPoint<T>,
    >;

    #[derive(Encode, Decode, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
    #[cfg_attr(feature = "std", derive(Debug))]
    pub struct Vault<AccountId, BlockNumber, Balance, CurrencyId: Copy, UnsignedFixedPoint> {
        /// Account identifier of the Vault
        pub id: VaultId<AccountId, CurrencyId>,
        /// Current status of the vault
        pub status: VaultStatus,
        /// Block height until which this Vault is banned from being used for
        /// Issue, Redeem (except during automatic liquidation) and Replace.
        pub banned_until: Option<BlockNumber>,
        /// Custom secure collateral threshold
        pub secure_collateral_threshold: Option<UnsignedFixedPoint>,
        /// Number of tokens pending issue
        pub to_be_issued_tokens: Balance,
        /// Number of issued tokens
        pub issued_tokens: Balance,
        /// Number of tokens pending redeem
        pub to_be_redeemed_tokens: Balance,
        /// Number of tokens that have been requested for a replace through
        /// `request_replace`, but that have not been accepted yet by a new_vault.
        pub to_be_replaced_tokens: Balance,
        /// Amount of collateral that is available as griefing collateral to vaults accepting
        /// a replace request. It is to be payed out if the old_vault fails to call execute_replace.
        pub replace_collateral: Balance,
        /// Amount of collateral locked for accepted replace requests.
        pub active_replace_collateral: Balance,
        /// Amount of collateral that is locked for remaining to_be_redeemed
        /// tokens upon liquidation.
        pub liquidated_collateral: Balance,
    }

    #[frame_support::storage_alias]
    pub(super) type Vaults<T: Config> = StorageMap<Pallet<T>, Blake2_128Concat, DefaultVaultId<T>, DefaultVault<T>>;
}
pub mod v7 {
    use super::*;

    pub fn migrate_v6_to_v7<T: Config>() -> frame_support::weights::Weight {
        let mut weight = T::DbWeight::get().reads(1);

        // kintsugi is on V6 but interlay is also on V6
        if !matches!(crate::StorageVersion::<T>::get(), Version::V6) {
            log::info!("Not running vault storage migration");
            return weight; // already upgraded; don't run migration
        }

        // update vault struct to remove replace pallet fields
        crate::Vaults::<T>::translate(|_key, old: v6::DefaultVault<T>| {
            weight.saturating_accrue(T::DbWeight::get().reads_writes(1, 1));
            Some(crate::DefaultVault::<T> {
                id: old.id,
                status: old.status,
                banned_until: old.banned_until,
                secure_collateral_threshold: old.secure_collateral_threshold,
                to_be_issued_tokens: old.to_be_issued_tokens,
                issued_tokens: old.issued_tokens,
                to_be_redeemed_tokens: old.to_be_redeemed_tokens,
                liquidated_collateral: old.liquidated_collateral,
            })
        });

        // update version
        crate::StorageVersion::<T>::put(Version::V7);
        weight.saturating_add(T::DbWeight::get().reads_writes(0, 1))
    }
}

#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub enum VaultStatus {
    /// Vault is active - bool=true indicates that the vault accepts new issue requests
    Active(bool),

    /// Vault has been liquidated
    Liquidated,
}

impl Default for VaultStatus {
    fn default() -> Self {
        VaultStatus::Active(true)
    }
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Vault<AccountId, BlockNumber, Balance, CurrencyId: Copy, UnsignedFixedPoint> {
    /// Account identifier of the Vault
    pub id: VaultId<AccountId, CurrencyId>,
    /// Current status of the vault
    pub status: VaultStatus,
    /// Block height until which this Vault is banned from being used for
    /// Issue, Redeem (except during automatic liquidation) and Replace.
    pub banned_until: Option<BlockNumber>,
    /// Custom secure collateral threshold
    pub secure_collateral_threshold: Option<UnsignedFixedPoint>,
    /// Number of tokens pending issue
    pub to_be_issued_tokens: Balance,
    /// Number of issued tokens
    pub issued_tokens: Balance,
    /// Number of tokens pending redeem
    pub to_be_redeemed_tokens: Balance,
    /// Amount of collateral that is locked for remaining to_be_redeemed
    /// tokens upon liquidation.
    pub liquidated_collateral: Balance,
}

#[derive(Encode, Decode, Clone, PartialEq, TypeInfo, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(Debug, serde::Serialize, serde::Deserialize))]
pub struct SystemVault<Balance, CurrencyId: Copy> {
    // Number of tokens pending issue
    pub to_be_issued_tokens: Balance,
    // Number of issued tokens
    pub issued_tokens: Balance,
    // Number of tokens pending redeem
    pub to_be_redeemed_tokens: Balance,
    // amount of collateral stored
    pub collateral: Balance,
    /// the currency used for collateral
    pub currency_pair: VaultCurrencyPair<CurrencyId>,
}

impl<
        AccountId: Ord,
        BlockNumber: Default,
        Balance: HasCompact + Default,
        CurrencyId: Copy,
        UnsignedFixedPoint: Default,
    > Vault<AccountId, BlockNumber, Balance, CurrencyId, UnsignedFixedPoint>
{
    // note: public only for testing purposes
    pub fn new(
        id: VaultId<AccountId, CurrencyId>,
    ) -> Vault<AccountId, BlockNumber, Balance, CurrencyId, UnsignedFixedPoint> {
        Vault {
            id,
            banned_until: None,
            status: VaultStatus::Active(true),
            secure_collateral_threshold: Default::default(),
            issued_tokens: Default::default(),
            liquidated_collateral: Default::default(),
            to_be_issued_tokens: Default::default(),
            to_be_redeemed_tokens: Default::default(),
        }
    }

    pub fn is_liquidated(&self) -> bool {
        matches!(self.status, VaultStatus::Liquidated)
    }

    pub fn accepts_new_issues(&self) -> bool {
        matches!(self.status, VaultStatus::Active(true))
    }
}

pub type DefaultVault<T> = Vault<
    <T as frame_system::Config>::AccountId,
    <T as frame_system::Config>::BlockNumber,
    BalanceOf<T>,
    CurrencyId<T>,
    UnsignedFixedPoint<T>,
>;

pub type DefaultSystemVault<T> = SystemVault<BalanceOf<T>, CurrencyId<T>>;

#[cfg_attr(any(test, feature = "integration-tests"), visibility::make(pub))]
trait UpdatableVault<T: Config> {
    fn increase_issued(&mut self, tokens: &Amount<T>) -> DispatchResult;

    fn increase_to_be_issued(&mut self, tokens: &Amount<T>) -> DispatchResult;

    fn increase_to_be_redeemed(&mut self, tokens: &Amount<T>) -> DispatchResult;

    fn decrease_issued(&mut self, tokens: &Amount<T>) -> DispatchResult;

    fn decrease_to_be_issued(&mut self, tokens: &Amount<T>) -> DispatchResult;

    fn decrease_to_be_redeemed(&mut self, tokens: &Amount<T>) -> DispatchResult;
}

pub struct RichVault<T: Config> {
    pub(crate) data: DefaultVault<T>,
}

impl<T: Config> UpdatableVault<T> for RichVault<T> {
    fn increase_issued(&mut self, tokens: &Amount<T>) -> DispatchResult {
        if self.data.is_liquidated() {
            Pallet::<T>::get_rich_liquidation_vault(&self.data.id.currencies).increase_issued(tokens)
        } else {
            let new_value = self.issued_tokens().checked_add(&tokens)?.amount();
            self.update(|v| {
                v.issued_tokens = new_value;
                Ok(())
            })
        }
    }

    fn increase_to_be_issued(&mut self, tokens: &Amount<T>) -> DispatchResult {
        // this function should never be called on liquidated vaults
        ensure!(!self.data.is_liquidated(), Error::<T>::VaultNotFound);

        let new_value = self.to_be_issued_tokens().checked_add(&tokens)?.amount();
        self.update(|v| {
            v.to_be_issued_tokens = new_value;
            Ok(())
        })
    }

    fn increase_to_be_redeemed(&mut self, tokens: &Amount<T>) -> DispatchResult {
        // this function should never be called on liquidated vaults
        ensure!(!self.data.is_liquidated(), Error::<T>::VaultNotFound);

        let new_value = self.to_be_redeemed_tokens().checked_add(&tokens)?.amount();
        self.update(|v| {
            v.to_be_redeemed_tokens = new_value;
            Ok(())
        })
    }

    fn decrease_issued(&mut self, tokens: &Amount<T>) -> DispatchResult {
        if self.data.is_liquidated() {
            Pallet::<T>::get_rich_liquidation_vault(&self.data.id.currencies).decrease_issued(tokens)
        } else {
            let new_value = self.issued_tokens().checked_sub(&tokens)?.amount();
            self.update(|v| {
                v.issued_tokens = new_value;
                Ok(())
            })
        }
    }

    fn decrease_to_be_issued(&mut self, tokens: &Amount<T>) -> DispatchResult {
        if self.data.is_liquidated() {
            Pallet::<T>::get_rich_liquidation_vault(&self.data.id.currencies).decrease_to_be_issued(tokens)
        } else {
            let new_value = self.to_be_issued_tokens().checked_sub(&tokens)?.amount();
            self.update(|v| {
                v.to_be_issued_tokens = new_value;
                Ok(())
            })
        }
    }

    fn decrease_to_be_redeemed(&mut self, tokens: &Amount<T>) -> DispatchResult {
        // in addition to the change to this vault, _also_ change the liquidation vault
        if self.data.is_liquidated() {
            Pallet::<T>::get_rich_liquidation_vault(&self.data.id.currencies).decrease_to_be_redeemed(tokens)?;
        }
        let new_value = self.to_be_redeemed_tokens().checked_sub(&tokens)?.amount();
        self.update(|v| {
            v.to_be_redeemed_tokens = new_value;
            Ok(())
        })
    }
}

#[cfg_attr(test, mockable)]
impl<T: Config> RichVault<T> {
    pub(crate) fn id(&self) -> DefaultVaultId<T> {
        self.data.id.clone()
    }

    pub(crate) fn issued_tokens(&self) -> Amount<T> {
        Amount::new(self.data.issued_tokens, self.id().wrapped_currency())
    }

    pub(crate) fn to_be_issued_tokens(&self) -> Amount<T> {
        Amount::new(self.data.to_be_issued_tokens, self.id().wrapped_currency())
    }

    pub(crate) fn freely_redeemable_tokens(&self) -> Result<Amount<T>, DispatchError> {
        Ok(self.issued_tokens().checked_sub(&self.to_be_redeemed_tokens())?)
    }

    pub(crate) fn request_issue_tokens(&mut self, tokens: &Amount<T>) -> DispatchResult {
        self.increase_to_be_issued(tokens)
    }

    pub(crate) fn cancel_issue_tokens(&mut self, tokens: &Amount<T>) -> DispatchResult {
        self.decrease_to_be_issued(tokens)
    }

    pub(crate) fn execute_issue_tokens(&mut self, tokens: &Amount<T>) -> DispatchResult {
        self.decrease_to_be_issued(tokens)?;
        self.increase_issued(tokens)
    }

    pub(crate) fn request_redeem_tokens(&mut self, tokens: &Amount<T>) -> DispatchResult {
        self.increase_to_be_redeemed(tokens)
    }

    pub(crate) fn cancel_redeem_tokens(&mut self, tokens: &Amount<T>) -> DispatchResult {
        self.decrease_to_be_redeemed(tokens)
    }

    pub(crate) fn execute_redeem_tokens(&mut self, tokens: &Amount<T>) -> DispatchResult {
        // no need to update stake since these two token changes counteract the other's effect
        self.decrease_to_be_redeemed(tokens)?;
        self.decrease_issued(tokens)
    }

    pub(crate) fn wrapped_currency(&self) -> CurrencyId<T> {
        self.data.id.wrapped_currency()
    }

    pub(crate) fn backed_tokens(&self) -> Result<Amount<T>, DispatchError> {
        let amount = self
            .data
            .issued_tokens
            .checked_add(&self.data.to_be_issued_tokens)
            .ok_or(ArithmeticError::Overflow)?;
        Ok(Amount::new(amount, self.wrapped_currency()))
    }

    pub(crate) fn to_be_redeemed_tokens(&self) -> Amount<T> {
        Amount::new(self.data.to_be_redeemed_tokens, self.wrapped_currency())
    }

    pub(crate) fn liquidated_collateral(&self) -> Amount<T> {
        Amount::new(self.data.liquidated_collateral, self.data.id.currencies.collateral)
    }

    // todo: deduplicate these 2 funcs?
    pub fn get_vault_collateral(&self) -> Result<Amount<T>, DispatchError> {
        Pallet::<T>::compute_collateral(&self.id())
    }

    pub fn get_total_collateral(&self) -> Result<Amount<T>, DispatchError> {
        Pallet::<T>::get_backing_collateral(&self.id())
    }

    pub fn get_secure_threshold(&self) -> Result<UnsignedFixedPoint<T>, DispatchError> {
        let global_threshold =
            Pallet::<T>::secure_collateral_threshold(&self.id().currencies).ok_or(Error::<T>::ThresholdNotSet)?;
        Ok(self
            .data
            .secure_collateral_threshold
            .unwrap_or(UnsignedFixedPoint::<T>::zero())
            .max(global_threshold))
    }

    pub fn get_free_collateral(&self) -> Result<Amount<T>, DispatchError> {
        let used_collateral = self.get_used_collateral(self.get_secure_threshold()?)?;
        self.get_total_collateral()?.checked_sub(&used_collateral)
    }

    pub fn get_used_collateral(&self, threshold: UnsignedFixedPoint<T>) -> Result<Amount<T>, DispatchError> {
        let issued_tokens = self.backed_tokens()?;
        let issued_tokens_in_collateral = issued_tokens.convert_to(self.data.id.currencies.collateral)?;
        let used_collateral = issued_tokens_in_collateral.checked_mul(&threshold)?;
        self.get_total_collateral()?.min(&used_collateral)
    }

    pub fn issuable_tokens(&self) -> Result<Amount<T>, DispatchError> {
        // unable to issue additional tokens when banned
        if self.is_banned() {
            return Ok(Amount::new(0u32.into(), self.wrapped_currency()));
        }

        // used_collateral = (exchange_rate * (issued_tokens + to_be_issued_tokens)) * secure_collateral_threshold
        // free_collateral = collateral - used_collateral
        let free_collateral = self.get_free_collateral()?;

        let secure_threshold = self.get_secure_threshold()?;

        // issuable_tokens = (free_collateral / exchange_rate) / secure_collateral_threshold
        let issuable = Pallet::<T>::calculate_max_wrapped_from_collateral_for_threshold(
            &free_collateral,
            self.wrapped_currency(),
            secure_threshold,
        )?;

        Ok(issuable)
    }

    pub fn redeemable_tokens(&self) -> Result<Amount<T>, DispatchError> {
        // unable to redeem additional tokens when banned
        if self.is_banned() {
            return Ok(Amount::new(0u32.into(), self.wrapped_currency()));
        }

        self.issued_tokens().checked_sub(&self.to_be_redeemed_tokens())
    }

    pub(crate) fn set_custom_secure_threshold(&mut self, threshold: Option<UnsignedFixedPoint<T>>) -> DispatchResult {
        self.update(|v| {
            v.secure_collateral_threshold = threshold;
            Ok(())
        })
    }

    pub(crate) fn set_accept_new_issues(&mut self, accept_new_issues: bool) -> DispatchResult {
        self.update(|v| {
            v.status = VaultStatus::Active(accept_new_issues);
            Ok(())
        })
    }

    pub(crate) fn increase_liquidated_collateral(&mut self, amount: &Amount<T>) -> DispatchResult {
        self.update(|v| {
            v.liquidated_collateral = v
                .liquidated_collateral
                .checked_add(&amount.amount())
                .ok_or(ArithmeticError::Overflow)?;
            Ok(())
        })
    }

    pub(crate) fn decrease_liquidated_collateral(&mut self, amount: &Amount<T>) -> DispatchResult {
        self.update(|v| {
            v.liquidated_collateral = v
                .liquidated_collateral
                .checked_sub(&amount.amount())
                .ok_or(ArithmeticError::Underflow)?;
            Ok(())
        })
    }

    pub(crate) fn slash_for_to_be_redeemed(&mut self, amount: &Amount<T>) -> DispatchResult {
        let vault_id = self.id();
        let collateral = self.get_vault_collateral()?.min(amount)?;
        self.increase_liquidated_collateral(&collateral)?;
        PoolManager::<T>::withdraw_collateral(&vault_id, &vault_id.account_id, Some(collateral), None)?;
        Ok(())
    }

    pub(crate) fn slash_to_liquidation_vault(&mut self, amount: &Amount<T>) -> DispatchResult {
        let vault_id = self.id();

        // get the collateral supplied by the vault (i.e. excluding nomination)
        let collateral = self.get_vault_collateral()?;
        let (to_withdraw, to_slash) = amount
            .checked_sub(&collateral)
            .and_then(|leftover| Ok((collateral, Some(leftover))))
            .unwrap_or((amount.clone(), None));

        // "slash" vault first
        PoolManager::<T>::withdraw_collateral(&vault_id, &vault_id.account_id, Some(to_withdraw), None)?;
        // take remainder from nominators
        if let Some(to_slash) = to_slash {
            PoolManager::<T>::slash_collateral(&vault_id, &to_slash)?;
        }

        Pallet::<T>::transfer_funds(
            CurrencySource::LiquidatedCollateral(self.id()),
            CurrencySource::LiquidationVault(vault_id.currencies),
            amount,
        )?;
        Ok(())
    }

    pub(crate) fn liquidate(&mut self) -> Result<Amount<T>, DispatchError> {
        let vault_id = self.id();

        // we liquidate at most LIQUIDATION_THRESHOLD * collateral
        // this value is the amount of collateral held for the issued + to_be_issued
        let liquidated_collateral = self.get_used_collateral(
            Pallet::<T>::liquidation_collateral_threshold(&self.data.id.currencies)
                .ok_or(Error::<T>::ThresholdNotSet)?,
        )?;

        // the vault struct was modified in the call above - we need to re-fetch,
        // otherwise changes get overwritten below
        *self = Pallet::<T>::get_rich_vault_from_id(&vault_id)?;

        // amount of tokens being backed
        let collateral_tokens = self.backed_tokens()?;

        // (liquidated_collateral * (collateral_tokens - to_be_redeemed_tokens)) / collateral_tokens
        let liquidated_collateral_excluding_to_be_redeemed = Pallet::<T>::calculate_collateral(
            &liquidated_collateral,
            &collateral_tokens.checked_sub(&self.to_be_redeemed_tokens())?,
            &collateral_tokens,
        )?;

        let collateral_for_to_be_redeemed =
            liquidated_collateral.saturating_sub(&liquidated_collateral_excluding_to_be_redeemed)?;

        // slash collateral for the to_be_redeemed tokens
        // this is re-deposited once the tokens are burned
        self.slash_for_to_be_redeemed(&collateral_for_to_be_redeemed)?;

        // slash collateral used for issued + to_be_issued to the liquidation vault
        self.slash_to_liquidation_vault(&liquidated_collateral_excluding_to_be_redeemed)?;

        // Copy all tokens to the liquidation vault
        let mut liquidation_vault = Pallet::<T>::get_rich_liquidation_vault(&self.data.id.currencies);
        liquidation_vault.increase_issued(&self.issued_tokens())?;
        liquidation_vault.increase_to_be_issued(&self.to_be_issued_tokens())?;
        liquidation_vault.increase_to_be_redeemed(&self.to_be_redeemed_tokens())?;
        // todo: clear replace collateral?

        // withdraw stake from the reward pool
        ext::reward::set_stake::<T>(&vault_id, &Amount::zero(vault_id.wrapped_currency()))?;

        // Update vault: clear to_be_issued & issued_tokens, but don't touch to_be_redeemed
        let _ = self.update(|v| {
            v.to_be_issued_tokens = Zero::zero();
            v.issued_tokens = Zero::zero();
            v.status = VaultStatus::Liquidated;
            Ok(())
        });

        Ok(liquidated_collateral_excluding_to_be_redeemed)
    }

    pub fn ensure_not_banned(&self) -> DispatchResult {
        if self.is_banned() {
            Err(Error::<T>::VaultBanned.into())
        } else {
            Ok(())
        }
    }

    pub(crate) fn is_banned(&self) -> bool {
        match self.data.banned_until {
            None => false,
            Some(until) => ext::security::active_block_number::<T>() <= until,
        }
    }

    pub fn ban_until(&mut self, height: T::BlockNumber) {
        let _ = self.update(|v| {
            v.banned_until = Some(height);
            Ok(())
        });
    }

    fn new_deposit_public_key(&self, secure_id: H256) -> Result<BtcPublicKey, DispatchError> {
        let vault_public_key = Pallet::<T>::get_bitcoin_public_key(&self.data.id.account_id)?;
        let vault_public_key = vault_public_key
            .new_deposit_public_key(secure_id)
            .map_err(|_| Error::<T>::InvalidPublicKey)?;

        Ok(vault_public_key)
    }

    pub(crate) fn new_deposit_address(&mut self, secure_id: H256) -> Result<BtcAddress, DispatchError> {
        let public_key = self.new_deposit_public_key(secure_id)?;
        let btc_address = BtcAddress::P2WPKHv0(public_key.to_hash());
        Ok(btc_address)
    }

    fn update<F>(&mut self, func: F) -> DispatchResult
    where
        F: Fn(&mut DefaultVault<T>) -> DispatchResult,
    {
        func(&mut self.data)?;
        <crate::Vaults<T>>::insert(&self.id(), &self.data);
        Ok(())
    }
}

impl<T: Config> From<&RichVault<T>> for DefaultVault<T> {
    fn from(rv: &RichVault<T>) -> DefaultVault<T> {
        rv.data.clone()
    }
}

impl<T: Config> From<DefaultVault<T>> for RichVault<T> {
    fn from(vault: DefaultVault<T>) -> RichVault<T> {
        RichVault { data: vault }
    }
}

#[cfg_attr(feature = "integration-tests", visibility::make(pub))]
pub(crate) struct RichSystemVault<T: Config> {
    pub(crate) data: DefaultSystemVault<T>,
}

#[cfg_attr(test, mockable)]
impl<T: Config> RichSystemVault<T> {
    pub(crate) fn burn_issued(&mut self, tokens: &Amount<T>) -> DispatchResult {
        self.decrease_issued(tokens)
    }

    pub(crate) fn issued_tokens(&self) -> Amount<T> {
        Amount::new(self.data.issued_tokens, self.wrapped_currency())
    }

    pub(crate) fn to_be_issued_tokens(&self) -> Amount<T> {
        Amount::new(self.data.to_be_issued_tokens, self.wrapped_currency())
    }

    pub(crate) fn wrapped_currency(&self) -> CurrencyId<T> {
        self.data.currency_pair.wrapped
    }

    pub(crate) fn redeemable_tokens(&self) -> Result<Amount<T>, DispatchError> {
        self.issued_tokens().checked_sub(&self.to_be_redeemed_tokens())
    }

    pub(crate) fn to_be_backed_tokens(&self) -> Result<Amount<T>, DispatchError> {
        self.issued_tokens()
            .checked_add(&self.to_be_issued_tokens())?
            .checked_sub(&self.to_be_redeemed_tokens())
    }

    pub(crate) fn to_be_redeemed_tokens(&self) -> Amount<T> {
        Amount::new(self.data.to_be_redeemed_tokens, self.wrapped_currency())
    }

    #[cfg_attr(feature = "integration-tests", visibility::make(pub))]
    pub(crate) fn collateral(&self) -> Amount<T> {
        Amount::new(self.data.collateral, self.data.currency_pair.collateral)
    }

    pub(crate) fn increase_collateral(&mut self, amount: &Amount<T>) -> DispatchResult {
        let new_value = self.collateral().checked_add(&amount)?.amount();
        self.update(|v| {
            v.collateral = new_value;
            Ok(())
        })
    }

    pub(crate) fn decrease_collateral(&mut self, amount: &Amount<T>) -> DispatchResult {
        let new_value = self.collateral().checked_sub(&amount)?.amount();
        self.update(|v| {
            v.collateral = new_value;
            Ok(())
        })
    }
}

impl<T: Config> UpdatableVault<T> for RichSystemVault<T> {
    fn increase_issued(&mut self, tokens: &Amount<T>) -> DispatchResult {
        let new_value = self.issued_tokens().checked_add(&tokens)?.amount();
        self.update(|v| {
            v.issued_tokens = new_value;
            Ok(())
        })
    }

    fn increase_to_be_issued(&mut self, tokens: &Amount<T>) -> DispatchResult {
        let new_value = self.to_be_issued_tokens().checked_add(&tokens)?.amount();
        self.update(|v| {
            v.to_be_issued_tokens = new_value;
            Ok(())
        })
    }

    fn increase_to_be_redeemed(&mut self, tokens: &Amount<T>) -> DispatchResult {
        let new_value = self.to_be_redeemed_tokens().checked_add(&tokens)?.amount();
        self.update(|v| {
            v.to_be_redeemed_tokens = new_value;
            Ok(())
        })
    }

    fn decrease_issued(&mut self, tokens: &Amount<T>) -> DispatchResult {
        let new_value = self.issued_tokens().checked_sub(&tokens)?.amount();
        self.update(|v| {
            v.issued_tokens = new_value;
            Ok(())
        })
    }

    fn decrease_to_be_issued(&mut self, tokens: &Amount<T>) -> DispatchResult {
        let new_value = self.to_be_issued_tokens().checked_sub(&tokens)?.amount();
        self.update(|v| {
            v.to_be_issued_tokens = new_value;
            Ok(())
        })
    }

    fn decrease_to_be_redeemed(&mut self, tokens: &Amount<T>) -> DispatchResult {
        let new_value = self.to_be_redeemed_tokens().checked_sub(&tokens)?.amount();
        self.update(|v| {
            v.to_be_redeemed_tokens = new_value;
            Ok(())
        })
    }
}

#[cfg_attr(test, mockable)]
impl<T: Config> RichSystemVault<T> {
    fn update<F>(&mut self, func: F) -> Result<(), DispatchError>
    where
        F: Fn(&mut DefaultSystemVault<T>) -> Result<(), DispatchError>,
    {
        func(&mut self.data)?;
        <crate::LiquidationVault<T>>::insert(&self.data.currency_pair, self.data.clone());
        Ok(())
    }
}

impl<T: Config> From<&RichSystemVault<T>> for DefaultSystemVault<T> {
    fn from(rv: &RichSystemVault<T>) -> DefaultSystemVault<T> {
        rv.data.clone()
    }
}

impl<T: Config> From<DefaultSystemVault<T>> for RichSystemVault<T> {
    fn from(vault: DefaultSystemVault<T>) -> RichSystemVault<T> {
        RichSystemVault { data: vault }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::mock::{Test as T, *};

    #[test]
    fn migrating_from_v6_to_v7() {
        run_test(|| {
            crate::StorageVersion::<T>::put(Version::V6);

            let old = v6::DefaultVault::<T> {
                id: DefaultVaultId::<T>::new(123, Token(DOT), Token(IBTC)),
                status: VaultStatus::Active(true),
                banned_until: None,
                secure_collateral_threshold: Default::default(),
                to_be_issued_tokens: 0u32.into(),
                issued_tokens: 0u32.into(),
                to_be_redeemed_tokens: 0u32.into(),
                to_be_replaced_tokens: 0u32.into(),
                replace_collateral: 0u32.into(),
                active_replace_collateral: 0u32.into(),
                liquidated_collateral: 0u32.into(),
            };
            let key = DefaultVaultId::<T>::new(123, Token(DOT), Token(IBTC));

            v6::Vaults::<T>::insert(key.clone(), old.clone());

            v7::migrate_v6_to_v7::<T>();

            assert_eq!(crate::StorageVersion::<T>::get(), Version::V7);

            let new = crate::Vaults::<T>::get(key).unwrap();

            assert!(old.id == new.id);
            assert!(old.status == new.status);
            assert!(old.banned_until == new.banned_until);
            assert!(old.secure_collateral_threshold == new.secure_collateral_threshold);
            assert!(old.to_be_issued_tokens == new.to_be_issued_tokens);
            assert!(old.issued_tokens == new.issued_tokens);
            assert!(old.to_be_redeemed_tokens == new.to_be_redeemed_tokens);
            assert!(old.liquidated_collateral == new.liquidated_collateral);
        });
    }
}
