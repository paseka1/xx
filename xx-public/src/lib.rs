#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;

pub mod weights;

use frame_support::traits::{
    Currency, VestingSchedule, Get,
    EnsureOrigin, ExistenceRequirement::AllowDeath,
};
use frame_support::{
    decl_event, decl_module, decl_error, decl_storage, ensure,
    PalletId, dispatch::{DispatchResult, DispatchClass, Pays},
};
use sp_runtime::{
    traits::{Zero, AccountIdConversion}, RuntimeDebug
};
use frame_system::{ensure_root, ensure_signed};
use codec::{Encode, Decode, HasCompact};
use sp_std::prelude::*;
pub use weights::WeightInfo;

pub type CurrencyOf<T> = <<T as Config>::VestingSchedule as VestingSchedule<<T as frame_system::Config>::AccountId>>::Currency;
pub type BalanceOf<T> = <CurrencyOf<T> as Currency<<T as frame_system::Config>::AccountId>>::Balance;

/// Transfer data contains information about a single transfer
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, scale_info::TypeInfo)]
pub struct TransferData<AccountId, Balance: HasCompact, Block> {
    /// Destination account
    pub destination: AccountId,
    /// Amount to transfer
    #[codec(compact)]
    pub amount: Balance,
    /// Vesting schedules info
    pub schedules: Option<Vec<(Balance, Balance, Block)>>,
}

pub trait PublicAccountsHandler<AccountId> {
    fn accounts() -> Vec<AccountId>;
}

pub trait Config: frame_system::Config {
    /// The Event type.
    type RuntimeEvent: From<Event>
        + Into<<Self as frame_system::Config>::RuntimeEvent>;

    /// The Vesting mechanism.
    type VestingSchedule: VestingSchedule<Self::AccountId, Moment=Self::BlockNumber>;

    /// An ID used to derive the Testnet account
    type TestnetId: Get<PalletId>;

    /// An ID used to derive the Sale account
    type SaleId: Get<PalletId>;

    /// The admin origin for the pallet
    type AdminOrigin: EnsureOrigin<Self::RuntimeOrigin>;

    /// Weight information for extrinsics in this pallet.
    type WeightInfo: WeightInfo;
}

decl_storage! {
    trait Store for Module<T: Config> as XXSale {
        /// Testnet Manager account
        pub TestnetManager get(fn testnet_manager): Option<T::AccountId>;
        /// Sale Manager account
        pub SaleManager get(fn sale_manager): Option<T::AccountId>;
    }
	add_extra_genesis {
        config(testnet_manager): Option<T::AccountId>;
	    config(testnet_balance): BalanceOf<T>;
        config(sale_manager): Option<T::AccountId>;
	    config(sale_balance): BalanceOf<T>;
		build(|config| {
            // Set managers
            if let Some(manager) = &config.testnet_manager {
                <TestnetManager<T>>::put(manager);
            }
            if let Some(manager) = &config.sale_manager {
                <SaleManager<T>>::put(manager);
            }
		    // Create Testnet account and set balance from genesis
		    let testnet_account_id = <Module<T>>::testnet_account_id();
            let _ = <CurrencyOf<T>>::make_free_balance_be(&testnet_account_id, config.testnet_balance);
			// Create Sale account and set the balance from genesis
			let sale_account_id = <Module<T>>::sale_account_id();
            let _ = <CurrencyOf<T>>::make_free_balance_be(&sale_account_id, config.sale_balance);
		});
	}
}

decl_event! {
    pub enum Event
    {
        /// Testnet Manager updated
        TestnetManagerUpdated,
        /// Sale Manager updated
        SaleManagerUpdated,
    }
}

decl_error! {
	pub enum Error for Module<T: Config> {
	    /// Must be the testnet manager to call this function
        MustBeTestnetManager,
		/// Must be the sale manager to call this function
        MustBeSaleManager,
        /// Not enough funds to do distribution
        NotEnoughFunds,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::RuntimeOrigin {
	    const TestnetAccount: T::AccountId = T::TestnetId::get().into_account_truncating();
	    const SaleAccount: T::AccountId = T::SaleId::get().into_account_truncating();

        type Error = Error<T>;

	    fn deposit_event() = default;

        //----------------     ADMIN     ----------------//

        /// Set the Testnet manager account
        ///
        /// The dispatch origin must be AdminOrigin.
        ///
        #[weight = <T as Config>::WeightInfo::set_testnet_manager_account()]
        pub fn set_testnet_manager_account(origin, who: T::AccountId) {
            Self::ensure_admin(origin)?;
            <TestnetManager<T>>::put(who);
            Self::deposit_event(Event::TestnetManagerUpdated);
        }

        /// Set the Sale manager account
        ///
        /// The dispatch origin must be AdminOrigin.
        ///
        #[weight = <T as Config>::WeightInfo::set_sale_manager_account()]
        pub fn set_sale_manager_account(origin, who: T::AccountId) {
            Self::ensure_admin(origin)?;
            <SaleManager<T>>::put(who);
            Self::deposit_event(Event::SaleManagerUpdated);
        }

        //----------------    MANAGERS    ----------------//
        /// Do a tesnet distribution
        ///
        /// `data` is a vector of TransferData
        /// The dispatch origin must be `TestnetManager`
        ///
        #[weight = (
			<T as Config>::WeightInfo::testnet_distribute(data.len() as u32),
			DispatchClass::Operational,
			Pays::No
		)]
        pub fn testnet_distribute(origin,
            data: Vec<TransferData<T::AccountId, BalanceOf<T>, T::BlockNumber>>,
        ) {
            let who = ensure_signed(origin)?;
            ensure!(Self::is_testnet_manager(who), Error::<T>::MustBeTestnetManager);
            Self::do_testnet_distribution(data)?
        }

        /// Do a sale distribution
        ///
        /// `data` is a vector of TransferData
        /// The dispatch origin must be `SaleManager`
        ///
        #[weight = (
			<T as Config>::WeightInfo::sale_distribute(data.len() as u32),
			DispatchClass::Operational,
			Pays::No
		)]
        pub fn sale_distribute(origin,
            data: Vec<TransferData<T::AccountId, BalanceOf<T>, T::BlockNumber>>,
        ) {
            let who = ensure_signed(origin)?;
            ensure!(Self::is_sale_manager(who), Error::<T>::MustBeSaleManager);
            Self::do_sale_distribution(data)?
        }
	}
}

impl<T: Config> Module<T> {
    /// Get the Testnet AccountId
    pub fn testnet_account_id() -> T::AccountId {
        T::TestnetId::get().into_account_truncating()
    }

    /// Get the Sale AccountId
    pub fn sale_account_id() -> T::AccountId {
        T::SaleId::get().into_account_truncating()
    }

    /// Check if origin is admin
    fn ensure_admin(o: T::RuntimeOrigin) -> DispatchResult {
        <T as Config>::AdminOrigin::try_origin(o)
            .map(|_| ())
            .or_else(ensure_root)?;
        Ok(())
    }

    /// Check if given account is the testnet manager
    fn is_testnet_manager(who: T::AccountId) -> bool {
        if let Some(manager) = <TestnetManager<T>>::get() {
            who == manager
        } else {
            false
        }
    }

    /// Check if given account is the sale manager
    fn is_sale_manager(who: T::AccountId) -> bool {
        if let Some(manager) = <SaleManager<T>>::get() {
            who == manager
        } else {
            false
        }
    }

    /// Do a testnet distribution
    fn do_testnet_distribution(
        data: Vec<TransferData<T::AccountId, BalanceOf<T>, T::BlockNumber>>,
    ) -> DispatchResult {
        Self::do_distribution(Self::testnet_account_id(), data)
    }

    /// Do a sale distribution
    fn do_sale_distribution(
        data: Vec<TransferData<T::AccountId, BalanceOf<T>, T::BlockNumber>>,
    ) -> DispatchResult {
        Self::do_distribution(Self::sale_account_id(), data)
    }

    /// Do a distribution
    fn do_distribution(
        account: T::AccountId,
        data: Vec<TransferData<T::AccountId, BalanceOf<T>, T::BlockNumber>>,
    ) -> DispatchResult {
        // Exit early if not enough funds to do distribution
        let available = <CurrencyOf<T>>::free_balance(&account);
        let total = data.iter().fold(Zero::zero(), |acc, x| {
            acc + x.amount
        });
        ensure!(available >= total, Error::<T>::NotEnoughFunds);
        // Do distribution
        data.iter().try_for_each(|d| -> DispatchResult {
            <CurrencyOf<T>>::transfer(
                &account,
                &d.destination,
                d.amount,
                AllowDeath,
            )?;
            if let Some(vs) = &d.schedules {
                vs.iter().for_each( |v| {
                    // This can fail if we try to add more vesting schedules
                    // than the Vesting pallet limit.
                    // The caller is responsible for ensuring the limit is respected.
                    // This function can only be called by the privileged manager accounts,
                    // so this is fine.
                    // Regardless, in the case that too many schedules are used,
                    // by ignoring the return value, we ensure the function never
                    // fails, but the extra vesting schedules are ignored.
                    let _ = T::VestingSchedule::add_vesting_schedule(
                        &d.destination,
                        v.0,
                        v.1,
                        v.2,
                    );
                });
            }
            Ok(().into())
        })
    }
}

// Implement PublicAccountsHandler
impl<T: Config> PublicAccountsHandler<T::AccountId> for Module<T> {
    fn accounts() -> Vec<T::AccountId> {
        let testnet_account = Self::testnet_account_id();
        let sale_account = Self::sale_account_id();
        vec![testnet_account, sale_account]
    }
}

// Manual implementation of WhitelistedStorageKeys for runtime benchmarks
#[cfg(feature = "runtime-benchmarks")]
impl<T: Config> frame_support::traits::WhitelistedStorageKeys for Module<T> {
    fn whitelisted_storage_keys() -> frame_support::sp_std::vec::Vec<frame_benchmarking::TrackedStorageKey> {
        use frame_support::sp_std::vec;
        vec![]
    }
}
