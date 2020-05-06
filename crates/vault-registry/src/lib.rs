#![deny(warnings)]
#![cfg_attr(test, feature(proc_macro_hygiene))]
#![cfg_attr(not(feature = "std"), no_std)]

/// # Vault Registry implementation
/// This is the implementation of the Vault Registry following the spec at:
/// https://interlay.gitlab.io/polkabtc-spec/spec/vaultregistry.html
mod ext;
pub mod types;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod mock;

#[cfg(test)]
extern crate mocktopus;

#[cfg(test)]
use mocktopus::macros::mockable;

use codec::{Decode, Encode};
use frame_support::dispatch::DispatchResult;
use frame_support::{decl_event, decl_module, decl_storage, ensure};
use primitive_types::H256;
use sp_core::H160;
use sp_std::convert::TryInto;
use sp_std::vec::Vec;
use system::ensure_signed;

use security::ErrorCode;
use x_core::{Error, Result, UnitResult};

pub use crate::types::Vault;
use crate::types::{DefaultVault, PolkaBTC, RichVault, DOT};

/// Granularity of `SecureCollateralThreshold`, `AuctionCollateralThreshold`,
/// `LiquidationCollateralThreshold`, and `PunishmentFee`
pub const GRANULARITY: u32 = 5;

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct RegisterRequest<AccountId, DateTime> {
    registration_id: H256,
    vault: AccountId,
    timeout: DateTime,
}

/// ## Configuration and Constants
/// The pallet's configuration trait.
pub trait Trait:
    system::Trait + collateral::Trait + treasury::Trait + exchange_rate_oracle::Trait + security::Trait
{
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This pallet's storage items.
decl_storage! {
    trait Store for Module<T: Trait> as VaultRegistry {
    /// ## Storage
        /// The minimum collateral (DOT) a Vault needs to provide
        /// to participate in the issue process.
        MinimumCollateralVault: DOT<T>;

        /// If a Vault misbehaves in either the redeem or replace protocol by
        /// failing to prove that it sent the correct amount of BTC to the
        /// correct address within the time limit, a vault is punished.
        /// The punishment is the equivalent value of BTC in DOT
        /// (valued at the current exchange rate via `getExchangeRate`) plus a
        /// fixed `PunishmentFee` that is added as a percentage on top
        /// to compensate the damaged party for its loss.
        /// For example, if the `PunishmentFee` is set to 50000,
        /// it is equivalent to 50%.
        PunishmentFee: u128;

        /// If a Vault fails to execute a correct redeem or replace,
        /// it is temporarily banned from further issue, redeem or replace requests.
        PunishmentDelay: T::BlockNumber;

        /// If a Vault is running low on collateral and falls below
        /// `PremiumRedeemThreshold`, users are allocated a premium in DOT
        /// when redeeming with the Vault - as defined by this parameter.
        /// For example, if the RedeemPremiumFee is set to 5000, it is equivalent to 5%.
        RedeemPremiumFee: u128;

        /// Determines the over-collateralization rate for DOT collateral locked
        /// by Vaults, necessary for issuing PolkaBTC. Must to be strictly
        /// greater than 100000 and LiquidationCollateralThreshold.
        SecureCollateralThreshold: u128;

        /// Determines the rate for the collateral rate of Vaults, at which the
        /// BTC backed by the Vault are opened up for auction to other Vaults
        AuctionCollateralThreshold: u128;

        /// Determines the rate for the collateral rate of Vaults,
        /// at which users receive a premium in DOT, allocated from the
        /// Vault’s collateral, when performing a redeem with this Vault.
        /// Must to be strictly greater than 100000 and LiquidationCollateralThreshold.
        PremiumRedeemThreshold: u128;

        /// Determines the lower bound for the collateral rate in PolkaBTC.
        /// Must be strictly greater than 100000. If a Vault’s collateral rate
        /// drops below this, automatic liquidation (forced Redeem) is triggered.
        LiquidationCollateralThreshold: u128;

        /// Account identifier of an artificial Vault maintained by the VaultRegistry
        /// to handle polkaBTC balances and DOT collateral of liquidated Vaults.
        /// That is, when a Vault is liquidated, its balances are transferred to
        /// LiquidationVault and claims are later handled via the LiquidationVault.
        LiquidationVault: T::AccountId;

        /// Mapping of Vaults, using the respective Vault account identifier as key.
        Vaults: map hasher(blake2_128_concat) T::AccountId => Vault<T::AccountId, T::BlockNumber, PolkaBTC<T>>;
    }
}

// The pallet's dispatchable functions.
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Initializing events
        fn deposit_event() = default;

        /// Initiates the registration procedure for a new Vault.
        /// The Vault provides its BTC address and locks up DOT collateral,
        /// which is to be used to the issuing process.
        ///
        /// # Arguments
        /// * `collateral` - the amount of collateral to lock
        /// * `btc_address` - the BTC address of the vault to register
        ///
        /// # Errors
        /// * `InsuficientVaultCollateralAmount` - if the collateral is below the minimum threshold
        /// * `VaultAlreadyRegistered` - if a vault is already registered for the origin account
        /// * `InsufficientCollateralAvailable` - if the vault does not own enough collateral
        fn register_vault(origin, collateral: DOT<T>, btc_address: H160) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            ext::security::ensure_parachain_status_running::<T>()?;
            ext::security::ensure_parachain_status_running::<T>()?;

            ensure!(collateral >= Self::get_minimum_collateral_vault(),
                    Error::InsuficientVaultCollateralAmount);
            ensure!(!Self::vault_exists(&sender), Error::VaultAlreadyRegistered);

            ext::collateral::lock::<T>(&sender, collateral)?;
            let vault = RichVault::<T>::new(sender.clone(), btc_address);
            Self::_insert_vault(&sender, &vault);

            Self::deposit_event(Event::<T>::RegisterVault(vault.id(), collateral));

            Ok(())
        }

        /// Locks additional collateral as a security against stealing the
        /// Bitcoin locked with it.
        ///
        /// # Arguments
        /// * `amount` - the amount of extra collateral to lock
        ///
        /// # Errors
        /// * `VaultNotFound` - if no vault exists for the origin account
        /// * `InsufficientCollateralAvailable` - if the vault does not own enough collateral
        fn lock_additional_collateral(origin, amount: DOT<T>) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            Self::check_parachain_not_shutdown_or_has_errors([ErrorCode::OracleOffline].to_vec())?;

            let vault = Self::rich_vault_from_id(&sender)?;
            vault.increase_collateral(amount)?;
            Self::deposit_event(Event::<T>::LockAdditionalCollateral(
                vault.id(),
                amount,
                vault.get_collateral(),
                vault.get_free_collateral()?,
            ));
            Ok(())
        }

        /// Withdraws `amount` of the collateral from the amount locked by
        /// the vault corresponding to the origin account
        /// The collateral left after withdrawal must be more
        /// (free or used in backing issued PolkaBTC) than MinimumCollateralVault
        /// and above the SecureCollateralThreshold. Collateral that is currently
        /// being used to back issued PolkaBTC remains locked until the Vault
        /// is used for a redeem request (full release can take multiple redeem requests).
        ///
        /// # Arguments
        /// * `amount` - the amount of collateral to withdraw
        ///
        /// # Errors
        /// * `VaultNotFound` - if no vault exists for the origin account
        /// * `InsufficientCollateralAvailable` - if the vault does not own enough collateral
        fn withdraw_collateral(origin, amount: DOT<T>) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            ext::security::ensure_parachain_status_running::<T>()?;
            let vault = Self::rich_vault_from_id(&sender)?;
            vault.withdraw_collateral(amount)?;
            Self::deposit_event(Event::<T>::WithdrawCollateral(
                sender.clone(),
                amount,
                vault.get_collateral(),
            ));
            Ok(())
        }
    }
}

#[cfg_attr(test, mockable)]
impl<T: Trait> Module<T> {
    /// Public functions
    pub fn _punishment_fee() -> Result<DOT<T>> {
        Self::u128_to_dot(PunishmentFee::get())
    }

    pub fn _get_redeem_premium_fee() -> Result<DOT<T>> {
        Self::u128_to_dot(RedeemPremiumFee::get())
    }

    pub fn _get_vault_from_id(vault_id: &T::AccountId) -> Result<DefaultVault<T>> {
        ensure!(Self::vault_exists(&vault_id), Error::VaultNotFound);
        Ok(<Vaults<T>>::get(vault_id))
    }

    /// Increases the amount of tokens to be issued in the next issue request
    ///
    /// # Arguments
    /// * `vault_id` - the id of the vault from which to increase to-be-issued tokens
    /// * `tokens` - the amount of tokens to be reserved
    ///
    /// # Errors
    /// * `VaultNotFound` - if no vault exists for the given `vault_id`
    /// * `ExceedingVaultLimit` - if the amount of tokens to be issued is higher than the issuable amount by the vault
    pub fn _increase_to_be_issued_tokens(
        vault_id: &T::AccountId,
        tokens: PolkaBTC<T>,
    ) -> Result<H160> {
        ext::security::ensure_parachain_status_running::<T>()?;
        let mut vault = Self::rich_vault_from_id(&vault_id)?;
        vault.increase_to_be_issued(tokens)?;
        Self::deposit_event(Event::<T>::IncreaseToBeIssuedTokens(vault.id(), tokens));
        Ok(vault.data.btc_address)
    }

    /// Decreases the amount of tokens to be issued in the next issue request
    ///
    /// # Arguments
    /// * `vault_id` - the id of the vault from which to decrease to-be-issued tokens
    /// * `tokens` - the amount of tokens to be unreserved
    ///
    /// # Errors
    /// * `VaultNotFound` - if no vault exists for the given `vault_id`
    /// * `InsufficientTokensCommitted` - if the amount of tokens reserved is too low
    pub fn _decrease_to_be_issued_tokens(
        vault_id: &T::AccountId,
        tokens: PolkaBTC<T>,
    ) -> UnitResult {
        Self::check_parachain_not_shutdown_or_has_errors(
            [
                ErrorCode::InvalidBTCRelay,
                ErrorCode::OracleOffline,
                ErrorCode::Liquidation,
            ]
            .to_vec(),
        )?;

        let mut vault = Self::rich_vault_from_id(&vault_id)?;
        vault.decrease_to_be_issued(tokens)?;
        Self::deposit_event(Event::<T>::DecreaseToBeIssuedTokens(vault.id(), tokens));
        Ok(())
    }

    /// Issues an amount of `tokens` tokens for the given `vault_id`
    /// At this point, the to-be-issued tokens assigned to a vault are decreased
    /// and the issued tokens balance is increased by the amount of issued tokens.
    ///
    /// # Arguments
    /// * `vault_id` - the id of the vault from which to issue tokens
    /// * `tokens` - the amount of tokens to issue
    ///
    /// # Errors
    /// * `VaultNotFound` - if no vault exists for the given `vault_id`
    /// * `InsufficientTokensCommitted` - if the amount of tokens reserved is too low
    pub fn _issue_tokens(vault_id: &T::AccountId, tokens: PolkaBTC<T>) -> UnitResult {
        Self::check_parachain_not_shutdown_or_has_errors(
            [
                ErrorCode::InvalidBTCRelay,
                ErrorCode::OracleOffline,
                ErrorCode::Liquidation,
            ]
            .to_vec(),
        )?;
        let mut vault = Self::rich_vault_from_id(&vault_id)?;
        vault.issue_tokens(tokens)?;
        Self::deposit_event(Event::<T>::IssueTokens(vault.id(), tokens));
        Ok(())
    }

    /// Adds an amount tokens to the to-be-redeemed tokens balance of a vault.
    /// This function serves as a prevention against race conditions in the
    /// redeem and replace procedures. If, for example, a vault would receive
    /// two redeem requests at the same time that have a higher amount of tokens
    ///  to be issued than his issuedTokens balance, one of the two redeem
    /// requests should be rejected.
    ///
    /// # Arguments
    /// * `vault_id` - the id of the vault from which to increase to-be-redeemed tokens
    /// * `tokens` - the amount of tokens to be redeemed
    ///
    /// # Errors
    /// * `VaultNotFound` - if no vault exists for the given `vault_id`
    /// * `InsufficientTokensCommitted` - if the amount of redeemable tokens is too low
    pub fn _increase_to_be_redeemed_tokens(
        vault_id: &T::AccountId,
        tokens: PolkaBTC<T>,
    ) -> UnitResult {
        Self::check_parachain_not_shutdown_or_has_errors(
            [
                ErrorCode::InvalidBTCRelay,
                ErrorCode::OracleOffline,
                ErrorCode::Liquidation,
            ]
            .to_vec(),
        )?;
        let mut vault = Self::rich_vault_from_id(&vault_id)?;
        vault.increase_to_be_redeemed(tokens)?;
        Self::deposit_event(Event::<T>::IncreaseToBeRedeemedTokens(vault.id(), tokens));
        Ok(())
    }

    /// Subtracts an amount tokens from the to-be-redeemed tokens balance of a vault.
    ///
    /// # Arguments
    /// * `vault_id` - the id of the vault from which to decrease to-be-redeemed tokens
    /// * `tokens` - the amount of tokens to be redeemed
    ///
    /// # Errors
    /// * `VaultNotFound` - if no vault exists for the given `vault_id`
    /// * `InsufficientTokensCommitted` - if the amount of to-be-redeemed tokens is too low
    pub fn _decrease_to_be_redeemed_tokens(
        vault_id: &T::AccountId,
        tokens: PolkaBTC<T>,
    ) -> UnitResult {
        Self::check_parachain_not_shutdown_or_has_errors(
            [
                ErrorCode::InvalidBTCRelay,
                ErrorCode::OracleOffline,
                ErrorCode::Liquidation,
            ]
            .to_vec(),
        )?;
        let mut vault = Self::rich_vault_from_id(&vault_id)?;
        vault.decrease_to_be_redeemed(tokens)?;
        Self::deposit_event(Event::<T>::DecreaseToBeRedeemedTokens(vault.id(), tokens));
        Ok(())
    }

    /// Decreases the amount of tokens f a redeem request is not fulfilled
    /// Removes the amount of tokens assigned to the to-be-redeemed tokens.
    /// At this point, we consider the tokens lost and the issued tokens are
    /// removed from the vault
    ///
    /// # Arguments
    /// * `vault_id` - the id of the vault from which to decrease tokens
    /// * `tokens` - the amount of tokens to be decreased
    /// * `user_id` - the id of the user making the redeem request
    ///
    /// # Errors
    /// * `VaultNotFound` - if no vault exists for the given `vault_id`
    /// * `InsufficientTokensCommitted` - if the amount of to-be-redeemed tokens
    ///                                   or issued tokens is too low
    pub fn _decrease_tokens(
        vault_id: &T::AccountId,
        user_id: &T::AccountId,
        tokens: PolkaBTC<T>,
    ) -> UnitResult {
        Self::check_parachain_not_shutdown_or_has_errors(
            [
                ErrorCode::InvalidBTCRelay,
                ErrorCode::OracleOffline,
                ErrorCode::Liquidation,
            ]
            .to_vec(),
        )?;
        let mut vault = Self::rich_vault_from_id(&vault_id)?;
        vault.decrease_tokens(tokens)?;
        Self::deposit_event(Event::<T>::DecreaseTokens(
            vault.id(),
            user_id.clone(),
            tokens,
        ));
        Ok(())
    }

    /// Reduces the to-be-redeemed tokens when a redeem request successfully completes
    ///
    /// # Arguments
    /// * `vault_id` - the id of the vault from which to redeem tokens
    /// * `tokens` - the amount of tokens to be decreased
    ///
    /// # Errors
    /// * `VaultNotFound` - if no vault exists for the given `vault_id`
    /// * `InsufficientTokensCommitted` - if the amount of to-be-redeemed tokens
    ///                                   or issued tokens is too low
    pub fn _redeem_tokens(vault_id: &T::AccountId, tokens: PolkaBTC<T>) -> UnitResult {
        Self::check_parachain_not_shutdown_or_has_errors(
            [
                ErrorCode::InvalidBTCRelay,
                ErrorCode::OracleOffline,
                ErrorCode::Liquidation,
            ]
            .to_vec(),
        )?;
        let mut vault = Self::rich_vault_from_id(&vault_id)?;
        vault.redeem_tokens(tokens)?;
        Self::deposit_event(Event::<T>::RedeemTokens(vault.id(), tokens));
        Ok(())
    }

    /// Handles a redeem request, where a user is paid a premium in DOT.
    /// Calls redeem_tokens and then allocates the corresponding amount of DOT
    /// to the redeemer using the Vault’s free collateral
    ///
    /// # Arguments
    /// * `vault_id` - the id of the vault from which to redeem premiums
    /// * `tokens` - the amount of tokens redeemed
    /// * `premium` - the amount of DOT to be paid to the user as a premium
    ///               using the Vault’s released collateral.
    /// * `user` - the user redeeming at a premium
    ///
    /// # Errors
    /// * `VaultNotFound` - if no vault exists for the given `vault_id`
    /// * `InsufficientTokensCommitted` - if the amount of to-be-redeemed tokens
    ///                                   or issued tokens is too low
    /// * `InsufficientFunds` - if the vault does not have `premium` amount of collateral
    pub fn _redeem_tokens_premium(
        vault_id: &T::AccountId,
        tokens: PolkaBTC<T>,
        premium: DOT<T>,
        redeemer_id: &T::AccountId,
    ) -> UnitResult {
        Self::check_parachain_not_shutdown_or_has_errors(
            [
                ErrorCode::InvalidBTCRelay,
                ErrorCode::OracleOffline,
                ErrorCode::Liquidation,
            ]
            .to_vec(),
        )?;
        let mut vault = Self::rich_vault_from_id(&vault_id)?;
        vault.redeem_tokens(tokens)?;
        if premium > 0.into() {
            ext::collateral::slash::<T>(vault_id, redeemer_id, premium)?;
        }

        Self::deposit_event(Event::<T>::RedeemTokensPremium(
            vault_id.clone(),
            tokens,
            premium,
            redeemer_id.clone(),
        ));
        Ok(())
    }

    /// Handles redeem requests which are executed during a LIQUIDATION recover.
    /// Reduces the issued token of the LiquidationVault and slashes the
    /// corresponding amount of DOT collateral.
    /// Once LiquidationVault has not more issuedToken left,
    /// removes the LIQUIDATION error from the BTC Parachain status.
    ///
    /// # Arguments
    /// * `redeemer_id` - the account of the user redeeming PolkaBTC
    /// * `tokens` - the amount of PolkaBTC to be redeemed in DOT with the
    ///              LiquidationVault, denominated in BTC
    ///
    /// # Errors
    /// * `InsufficientTokensCommitted` - if the amount of tokens issued by the liquidation vault is too low
    /// * `InsufficientFunds` - if the liquidation vault does not have enough collateral to transfer
    pub fn _redeem_tokens_liquidation(
        redeemer_id: &T::AccountId,
        tokens: PolkaBTC<T>,
    ) -> UnitResult {
        Self::check_parachain_not_shutdown_or_has_errors(
            [ErrorCode::InvalidBTCRelay, ErrorCode::OracleOffline].to_vec(),
        )?;
        let vault_id = <LiquidationVault<T>>::get();
        let mut vault = Self::rich_vault_from_id(&vault_id)?;
        vault.decrease_issued(tokens)?;
        let to_slash = ext::oracle::btc_to_dots::<T>(tokens)?;
        ext::collateral::slash::<T>(&vault_id, &redeemer_id, to_slash)?;

        Self::deposit_event(Event::<T>::RedeemTokensLiquidation(
            redeemer_id.clone(),
            tokens,
        ));

        if vault.data.issued_tokens == 0.into() {
            ext::security::recover_from_liquidation::<T>()?;
        }

        Ok(())
    }

    /// Replaces the old vault by the new vault by transfering tokens
    /// from the old vault to the new one
    ///
    /// # Arguments
    /// * `old_vault_id` - the id of the old vault
    /// * `new_vault_id` - the id of the new vault
    /// * `tokens` - the amount of tokens to be transfered from the old to the new vault
    /// * `colalteral` - the collateral to be locked by the new vault
    ///
    /// # Errors
    /// * `VaultNotFound` - if either the old or new vault does not exist
    /// * `InsufficientTokensCommitted` - if the amount of tokens of the old vault is too low
    /// * `InsufficientFunds` - if the new vault does not have enough collateral to lock
    pub fn _replace_tokens(
        old_vault_id: &T::AccountId,
        new_vault_id: &T::AccountId,
        tokens: PolkaBTC<T>,
        collateral: DOT<T>,
    ) -> UnitResult {
        Self::check_parachain_not_shutdown_or_has_errors(
            [
                ErrorCode::InvalidBTCRelay,
                ErrorCode::OracleOffline,
                ErrorCode::Liquidation,
            ]
            .to_vec(),
        )?;

        let mut old_vault = Self::rich_vault_from_id(&old_vault_id)?;
        let mut new_vault = Self::rich_vault_from_id(&new_vault_id)?;
        old_vault.transfer(&mut new_vault, tokens)?;
        new_vault.increase_collateral(collateral)?;

        Self::deposit_event(Event::<T>::ReplaceTokens(
            old_vault_id.clone(),
            new_vault_id.clone(),
            tokens,
            collateral,
        ));
        Ok(())
    }

    /// Liquidates a vault, transferring all of its token balances to the
    /// `LiquidationVault`, as well as the DOT collateral
    ///
    /// # Arguments
    /// * `vault_id` - the id of the vault to liquidate
    ///
    /// # Errors
    /// * `VaultNotFound` - if the vault to liquidate does not exist
    pub fn _liquidate_vault(vault_id: &T::AccountId) -> UnitResult {
        // Parachain must not be shutdown
        ext::security::ensure_parachain_status_not_shutdown::<T>()?;

        let liquidation_vault_id = <LiquidationVault<T>>::get();
        let vault: RichVault<T> = Self::rich_vault_from_id(&vault_id)?;
        let mut liquidation_vault: RichVault<T> = Self::rich_vault_from_id(&liquidation_vault_id)?;

        vault.liquidate(&mut liquidation_vault)?;

        Self::deposit_event(Event::<T>::LiquidateVault(vault_id.clone()));
        Ok(())
    }

    pub fn _insert_vault<V: Into<DefaultVault<T>>>(id: &T::AccountId, rich_vault: V) {
        let vault: DefaultVault<T> = rich_vault.into();
        <Vaults<T>>::insert(id, vault)
    }

    pub fn _ban_vault(vault_id: T::AccountId, height: T::BlockNumber) -> UnitResult {
        let mut vault = Self::rich_vault_from_id(&vault_id)?;
        vault.ban_until(height);
        Ok(())
    }

    pub fn _ensure_not_banned(vault_id: &T::AccountId, height: T::BlockNumber) -> UnitResult {
        let vault = Self::rich_vault_from_id(&vault_id)?;
        vault.ensure_not_banned(height)
    }

    /// Threshold checks
    pub fn _is_vault_below_secure_threshold(vault_id: &T::AccountId) -> Result<bool> {
        Self::is_vault_below_threshold(&vault_id, <SecureCollateralThreshold>::get())
    }

    pub fn _is_vault_below_auction_threshold(vault_id: &T::AccountId) -> Result<bool> {
        Self::is_vault_below_threshold(&vault_id, <AuctionCollateralThreshold>::get())
    }

    pub fn _is_vault_below_premium_threshold(vault_id: &T::AccountId) -> Result<bool> {
        Self::is_vault_below_threshold(&vault_id, <PremiumRedeemThreshold>::get())
    }

    pub fn _is_vault_below_liquidation_threshold(vault_id: &T::AccountId) -> Result<bool> {
        Self::is_vault_below_threshold(&vault_id, <LiquidationCollateralThreshold>::get())
    }

    pub fn _is_collateral_below_secure_threshold(
        collateral: DOT<T>,
        btc_amount: PolkaBTC<T>,
    ) -> Result<bool> {
        let threshold = <SecureCollateralThreshold>::get();
        Self::is_collateral_below_threshold(collateral, btc_amount, threshold)
    }

    pub fn _get_secure_collateral_threshold() -> u128 {
        <SecureCollateralThreshold>::get()
    }

    pub fn _get_auction_collateral_threshold() -> u128 {
        <AuctionCollateralThreshold>::get()
    }

    pub fn _get_premium_redeem_threshold() -> u128 {
        <PremiumRedeemThreshold>::get()
    }

    pub fn _get_liquidation_collateral_threshold() -> u128 {
        <LiquidationCollateralThreshold>::get()
    }

    pub fn _set_secure_collateral_threshold(threshold: u128) {
        <SecureCollateralThreshold>::set(threshold);
    }

    pub fn _set_auction_collateral_threshold(threshold: u128) {
        <AuctionCollateralThreshold>::set(threshold);
    }

    pub fn _set_premium_redeem_threshold(threshold: u128) {
        <PremiumRedeemThreshold>::set(threshold);
    }

    pub fn _set_liquidation_collateral_threshold(threshold: u128) {
        <LiquidationCollateralThreshold>::set(threshold);
    }

    pub fn _is_over_minimum_collateral(amount: DOT<T>) -> bool {
        amount > Self::get_minimum_collateral_vault()
    }

    pub fn _get_total_liquidation_value() -> Result<u128> {
        let liquidation_vault_id = <LiquidationVault<T>>::get();

        let liquidation_vault = Self::rich_vault_from_id(&liquidation_vault_id)?;

        let liquidated_polka_btc_in_dot =
            ext::oracle::btc_to_dots::<T>(liquidation_vault.data.issued_tokens)?;

        let raw_collateral =
            Self::dot_to_u128(ext::collateral::for_account::<T>(&liquidation_vault_id))?;

        let raw_liquidated_polka_btc_in_dot = Self::dot_to_u128(liquidated_polka_btc_in_dot)?;

        let total_liquidation_value = raw_liquidated_polka_btc_in_dot - raw_collateral;
        Ok(total_liquidation_value)
    }

    /// Private getters and setters

    fn rich_vault_from_id(vault_id: &T::AccountId) -> Result<RichVault<T>> {
        let vault = Self::_get_vault_from_id(vault_id)?;
        Ok(vault.into())
    }

    fn vault_exists(id: &T::AccountId) -> bool {
        <Vaults<T>>::contains_key(id)
    }

    fn get_minimum_collateral_vault() -> DOT<T> {
        <MinimumCollateralVault<T>>::get()
    }

    /// Other helpers
    /// Returns an error if the parachain is not in running state
    fn ensure_parachain_running() -> UnitResult {
        // TODO: integrate security module
        // ensure!(
        //     !<security::Module<T>>::check_parachain_status(
        //         StatusCode::Shutdown),
        //     Error::Shutdown
        // );
        Ok(())

    // Other helpers
    /// Ensure that the parachain is NOT shutdown and DOES NOT have the given errors
    ///
    /// # Arguments
    ///
    ///   * `error_codes` - list of `ErrorCode` to be checked
    ///
    fn check_parachain_not_shutdown_or_has_errors(error_codes: Vec<ErrorCode>) -> UnitResult {
        // Parachain must not be shutdown
        ext::security::ensure_parachain_status_not_shutdown::<T>()?;
        // There must not be in InvalidBTCRelay, OracleOffline or Liquidation error state
        ext::security::ensure_parachain_status_has_not_specific_errors::<T>(error_codes)
    }

    fn is_vault_below_threshold(vault_id: &T::AccountId, threshold: u128) -> Result<bool> {
        let vault = Self::rich_vault_from_id(&vault_id)?;

        // the currently issued tokens in PolkaBTC
        let issued_tokens = vault.data.issued_tokens;

        // the current locked collateral by the vault
        let collateral = ext::collateral::for_account::<T>(&vault_id);

        Self::is_collateral_below_threshold(collateral, issued_tokens, threshold)
    }

    fn is_collateral_below_threshold(
        collateral: DOT<T>,
        btc_amount: PolkaBTC<T>,
        threshold: u128,
    ) -> Result<bool> {
        let max_tokens =
            Self::calculate_max_polkabtc_from_collateral_for_threshold(collateral, threshold)?;

        // check if the max_tokens are below the issued tokens
        Ok(max_tokens < btc_amount)
    }

    fn calculate_max_polkabtc_from_collateral_for_threshold(
        collateral: DOT<T>,
        threshold: u128,
    ) -> Result<PolkaBTC<T>> {
        // convert the collateral to polkabtc
        let collateral_in_polka_btc = ext::oracle::dots_to_btc::<T>(collateral)?;
        let raw_collateral_in_polka_btc = Self::polkabtc_to_u128(collateral_in_polka_btc)?;

        // calculate how many tokens should be maximally issued given the threshold
        let raw_scaled_collateral_in_polka_btc = raw_collateral_in_polka_btc
            .checked_mul(10u32.pow(GRANULARITY) as u128)
            .ok_or(Error::RuntimeError)?;
        let raw_max_tokens = raw_scaled_collateral_in_polka_btc
            .checked_div(threshold)
            .unwrap_or(0);

        let max_tokens = Self::u128_to_polkabtc(raw_max_tokens)?;
        Ok(max_tokens)
    }

    fn polkabtc_to_u128(x: PolkaBTC<T>) -> Result<u128> {
        TryInto::<u128>::try_into(x).map_err(|_| Error::RuntimeError)
    }

    fn dot_to_u128(x: DOT<T>) -> Result<u128> {
        TryInto::<u128>::try_into(x).map_err(|_| Error::RuntimeError)
    }

    fn u128_to_dot(x: u128) -> Result<DOT<T>> {
        TryInto::<DOT<T>>::try_into(x).map_err(|_| Error::RuntimeError)
    }

    fn u128_to_polkabtc(x: u128) -> Result<PolkaBTC<T>> {
        TryInto::<PolkaBTC<T>>::try_into(x).map_err(|_| Error::RuntimeError)
    }
}

decl_event! {
    /// ## Events
    pub enum Event<T> where
            AccountId = <T as system::Trait>::AccountId,
            DOT = DOT<T>,
            BTCBalance = PolkaBTC<T> {
        RegisterVault(AccountId, DOT),
        /// id, new collateral, total collateral, free collateral
        LockAdditionalCollateral(AccountId, DOT, DOT, DOT),
        /// id, withdrawn collateral, total collateral
        WithdrawCollateral(AccountId, DOT, DOT),

        IncreaseToBeIssuedTokens(AccountId, BTCBalance),
        DecreaseToBeIssuedTokens(AccountId, BTCBalance),
        IssueTokens(AccountId, BTCBalance),
        IncreaseToBeRedeemedTokens(AccountId, BTCBalance),
        DecreaseToBeRedeemedTokens(AccountId, BTCBalance),
        DecreaseTokens(AccountId, AccountId, BTCBalance),
        RedeemTokens(AccountId, BTCBalance),
        RedeemTokensPremium(AccountId, BTCBalance, DOT, AccountId),
        RedeemTokensLiquidation(AccountId, BTCBalance),
        ReplaceTokens(AccountId, AccountId, BTCBalance, DOT),
        LiquidateVault(AccountId),
    }
}
