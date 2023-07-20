//! # Nomination Module

#![deny(warnings)]
#![cfg_attr(test, feature(proc_macro_hygiene))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
extern crate mocktopus;

#[cfg(test)]
use mocktopus::macros::mockable;

#[cfg(test)]
mod tests;

mod ext;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod default_weights;
pub use default_weights::WeightInfo;

use currency::Amount;
use frame_support::{
    dispatch::{DispatchError, DispatchResult},
    ensure, transactional,
};
use frame_system::{ensure_root, ensure_signed};
pub use pallet::*;
use primitives::VaultId;

pub(crate) type BalanceOf<T> = <T as currency::Config>::Balance;

pub(crate) type DefaultVaultId<T> = VaultId<<T as frame_system::Config>::AccountId, currency::CurrencyId<T>>;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use vault_registry::types::DefaultVaultCurrencyPair;

    /// ## Configuration
    /// The pallet's configuration trait.
    #[pallet::config]
    pub trait Config: frame_system::Config + security::Config + vault_registry::Config + fee::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Weight information for the extrinsics in this module.
        type WeightInfo: WeightInfo;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        NominationOptIn {
            vault_id: DefaultVaultId<T>,
        },
        NominationOptOut {
            vault_id: DefaultVaultId<T>,
        },
        DepositCollateral {
            vault_id: DefaultVaultId<T>,
            nominator_id: T::AccountId,
            amount: BalanceOf<T>,
        },
        WithdrawCollateral {
            vault_id: DefaultVaultId<T>,
            nominator_id: T::AccountId,
            amount: BalanceOf<T>,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Vault has already enabled nomination.
        VaultAlreadyOptedInToNomination,
        /// Vault has not enabled nomination.
        VaultNotOptedInToNomination,
        /// Vault not found.
        VaultNotFound,
        /// Account cannot withdraw.
        CannotWithdrawCollateral,
        /// Nomination is not enabled.
        VaultNominationDisabled,
        /// Nomination would overburden Vault.
        NominationExceedsLimit,
        /// Vault cannot withdraw.
        CollateralizationTooLow,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

    /// Flag indicating whether this feature is enabled
    #[pallet::storage]
    #[pallet::getter(fn is_nomination_enabled)]
    pub type NominationEnabled<T: Config> = StorageValue<_, bool, ValueQuery>;

    /// Map of Vaults who have enabled nomination
    #[pallet::storage]
    pub(super) type Vaults<T: Config> = StorageMap<_, Blake2_128Concat, DefaultVaultId<T>, bool, ValueQuery>;

    /// The maximum amount of collateral to be nominated for a given vault.
    #[pallet::storage]
    pub(super) type NominationLimit<T: Config> =
        StorageMap<_, Blake2_128Concat, DefaultVaultId<T>, BalanceOf<T>, ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig {
        pub is_nomination_enabled: bool,
    }

    #[cfg(feature = "std")]
    impl Default for GenesisConfig {
        fn default() -> Self {
            Self {
                is_nomination_enabled: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig {
        fn build(&self) {
            {
                NominationEnabled::<T>::put(self.is_nomination_enabled);
            }
        }
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // The pallet's dispatchable functions.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(<T as Config>::WeightInfo::set_nomination_enabled())]
        #[transactional]
        pub fn set_nomination_enabled(origin: OriginFor<T>, enabled: bool) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            <NominationEnabled<T>>::set(enabled);
            Ok(().into())
        }

        /// Allow nomination for this vault
        #[pallet::call_index(1)]
        #[pallet::weight(<T as Config>::WeightInfo::opt_in_to_nomination())]
        #[transactional]
        pub fn opt_in_to_nomination(
            origin: OriginFor<T>,
            currency_pair: DefaultVaultCurrencyPair<T>,
        ) -> DispatchResultWithPostInfo {
            let account_id = ensure_signed(origin)?;
            let vault_id = VaultId::new(account_id, currency_pair.collateral, currency_pair.wrapped);

            Self::_opt_in_to_nomination(&vault_id)?;
            Ok(().into())
        }

        /// Disallow nomination for this vault
        #[pallet::call_index(2)]
        #[pallet::weight(<T as Config>::WeightInfo::opt_out_of_nomination())]
        #[transactional]
        pub fn opt_out_of_nomination(
            origin: OriginFor<T>,
            currency_pair: DefaultVaultCurrencyPair<T>,
        ) -> DispatchResultWithPostInfo {
            let account_id = ensure_signed(origin)?;
            let vault_id = VaultId::new(account_id, currency_pair.collateral, currency_pair.wrapped);
            Self::_opt_out_of_nomination(&vault_id)?;
            Ok(().into())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(<T as Config>::WeightInfo::deposit_collateral())]
        #[transactional]
        pub fn deposit_collateral(
            origin: OriginFor<T>,
            vault_id: DefaultVaultId<T>,
            amount: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let nominator_id = ensure_signed(origin)?;
            Self::_deposit_collateral(&vault_id, &nominator_id, amount)?;
            Ok(().into())
        }

        #[pallet::call_index(4)]
        #[pallet::weight(<T as Config>::WeightInfo::withdraw_collateral())]
        #[transactional]
        pub fn withdraw_collateral(
            origin: OriginFor<T>,
            vault_id: DefaultVaultId<T>,
            amount: Option<BalanceOf<T>>,
            index: Option<T::Index>,
        ) -> DispatchResultWithPostInfo {
            let nominator_id = ensure_signed(origin)?;
            Self::_withdraw_collateral(&vault_id, &nominator_id, amount, index.unwrap_or_default())?;
            Ok(().into())
        }

        #[pallet::call_index(5)]
        #[pallet::weight(<T as Config>::WeightInfo::set_nomination_limit())]
        #[transactional]
        pub fn set_nomination_limit(
            origin: OriginFor<T>,
            currency_pair: DefaultVaultCurrencyPair<T>,
            limit: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let account_id = ensure_signed(origin)?;
            let vault_id = VaultId::new(account_id, currency_pair.collateral, currency_pair.wrapped);

            NominationLimit::<T>::insert(vault_id, limit);
            Ok(().into())
        }
    }
}

// "Internal" functions, callable by code.
#[cfg_attr(test, mockable)]
impl<T: Config> Pallet<T> {
    pub fn _withdraw_collateral(
        vault_id: &DefaultVaultId<T>,
        nominator_id: &T::AccountId,
        maybe_amount: Option<BalanceOf<T>>,
        index: T::Index,
    ) -> DispatchResult {
        let nonce = ext::staking::nonce::<T>(vault_id);
        let index = sp_std::cmp::min(index, nonce);

        let maybe_amount = maybe_amount.map(|x| Amount::<T>::new(x, vault_id.collateral_currency()));

        // nominators are always allowed to withdraw from stale staking pools
        if index == nonce {
            // we can only withdraw nominated collateral if the vault is still
            // above the secure threshold for issued + to_be_issued tokens
            ensure!(
                ext::vault_registry::is_allowed_to_withdraw_collateral::<T>(vault_id, maybe_amount.clone())?,
                Error::<T>::CannotWithdrawCollateral
            );

            if &vault_id.account_id != nominator_id {
                ensure!(Self::is_nomination_enabled(), Error::<T>::VaultNominationDisabled);
                ensure!(Self::is_opted_in(vault_id), Error::<T>::VaultNotOptedInToNomination);
            }
        }

        // withdraw `amount` of stake from the vault staking pool
        let amount = ext::vault_registry::pool_manager::withdraw_collateral::<T>(
            vault_id,
            nominator_id,
            maybe_amount,
            Some(index),
        )?;
        amount.unlock_on(&vault_id.account_id)?;
        amount.transfer(&vault_id.account_id, &nominator_id)?;

        ext::vault_registry::decrease_total_backing_collateral(&vault_id.currencies, &amount)?;

        Self::deposit_event(Event::<T>::WithdrawCollateral {
            vault_id: vault_id.clone(),
            nominator_id: nominator_id.clone(),
            amount: amount.amount(),
        });
        Ok(())
    }

    pub fn _deposit_collateral(
        vault_id: &DefaultVaultId<T>,
        nominator_id: &T::AccountId,
        amount: BalanceOf<T>,
    ) -> DispatchResult {
        let amount = Amount::new(amount, vault_id.collateral_currency());

        if &vault_id.account_id != nominator_id {
            let total_nominated_collateral = Self::get_total_nominated_collateral(vault_id)?;
            let new_nominated_collateral = total_nominated_collateral.checked_add(&amount)?;
            let max_nominatable_collateral = Self::get_nomination_limit(vault_id);

            ensure!(Self::is_nomination_enabled(), Error::<T>::VaultNominationDisabled);
            ensure!(Self::is_opted_in(vault_id), Error::<T>::VaultNotOptedInToNomination);

            ensure!(
                new_nominated_collateral.le(&max_nominatable_collateral)?,
                Error::<T>::NominationExceedsLimit
            );
            amount.transfer(&nominator_id, &vault_id.account_id)?;
        }

        // Deposit `amount` of stake into the vault staking pool
        ext::vault_registry::pool_manager::deposit_collateral::<T>(vault_id, nominator_id, &amount)?;
        amount.lock_on(&vault_id.account_id)?;
        ext::vault_registry::try_increase_total_backing_collateral(&vault_id.currencies, &amount)?;

        Self::deposit_event(Event::<T>::DepositCollateral {
            vault_id: vault_id.clone(),
            nominator_id: nominator_id.clone(),
            amount: amount.amount(),
        });
        Ok(())
    }

    /// Vault is to allow nominated collateral
    ///
    /// # Arguments
    /// * `vault_id` - the id of the vault to allow nomination for
    fn _opt_in_to_nomination(vault_id: &DefaultVaultId<T>) -> DispatchResult {
        ensure!(Self::is_nomination_enabled(), Error::<T>::VaultNominationDisabled);
        ensure!(
            ext::vault_registry::vault_exists::<T>(&vault_id),
            Error::<T>::VaultNotFound
        );
        ensure!(
            !<Vaults<T>>::contains_key(vault_id),
            Error::<T>::VaultAlreadyOptedInToNomination
        );
        <Vaults<T>>::insert(vault_id, true);
        Self::deposit_event(Event::<T>::NominationOptIn {
            vault_id: vault_id.clone(),
        });
        Ok(())
    }

    fn _opt_out_of_nomination(vault_id: &DefaultVaultId<T>) -> DispatchResult {
        ensure!(Self::is_opted_in(&vault_id), Error::<T>::VaultNotOptedInToNomination);
        let total_nominated_collateral = Self::get_total_nominated_collateral(&vault_id)?;
        ensure!(
            ext::vault_registry::is_allowed_to_withdraw_collateral::<T>(&vault_id, Some(total_nominated_collateral))?,
            Error::<T>::CollateralizationTooLow
        );

        let refunded_collateral = ext::vault_registry::pool_manager::kick_nominators::<T>(vault_id)?;

        // Update the system-wide total backing collateral
        ext::vault_registry::decrease_total_backing_collateral(&vault_id.currencies, &refunded_collateral)?;

        <Vaults<T>>::remove(vault_id);
        Self::deposit_event(Event::<T>::NominationOptOut {
            vault_id: vault_id.clone(),
        });
        Ok(())
    }

    pub fn is_opted_in(vault_id: &DefaultVaultId<T>) -> bool {
        <Vaults<T>>::contains_key(&vault_id)
    }

    pub fn get_total_nominated_collateral(vault_id: &DefaultVaultId<T>) -> Result<Amount<T>, DispatchError> {
        let vault_backing_collateral = ext::vault_registry::get_backing_collateral::<T>(vault_id)?;
        let vault_actual_collateral = ext::vault_registry::compute_collateral::<T>(vault_id)?;
        vault_backing_collateral.checked_sub(&vault_actual_collateral)
    }

    pub fn get_nomination_limit(vault_id: &DefaultVaultId<T>) -> Amount<T> {
        let limit = NominationLimit::<T>::get(vault_id);
        Amount::new(limit, vault_id.collateral_currency())
    }

    pub fn get_nominator_collateral(
        vault_id: &DefaultVaultId<T>,
        nominator_id: &T::AccountId,
    ) -> Result<Amount<T>, DispatchError> {
        let amount = ext::staking::compute_stake::<T>(vault_id, nominator_id)?;
        Ok(Amount::new(amount, vault_id.collateral_currency()))
    }
}

impl<T: Config> traits::NominationApi<DefaultVaultId<T>, Amount<T>> for Pallet<T> {
    fn deposit_vault_collateral(vault_id: &DefaultVaultId<T>, amount: &Amount<T>) -> Result<(), DispatchError> {
        Pallet::<T>::_deposit_collateral(vault_id, &vault_id.account_id, amount.amount())
    }

    fn ensure_opted_in_to_nomination(vault_id: &DefaultVaultId<T>) -> DispatchResult {
        ensure!(Self::is_opted_in(vault_id), Error::<T>::VaultNotOptedInToNomination);
        Ok(())
    }

    #[cfg(any(feature = "runtime-benchmarks", test))]
    fn opt_in_to_nomination(vault_id: &DefaultVaultId<T>) {
        Vaults::<T>::insert(vault_id, true);
    }
}
