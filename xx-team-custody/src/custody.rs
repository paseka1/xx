use super::{
    Config, BalanceOf, Module, Error, RawEvent,
    TeamAccounts, CustodyAccounts, TotalCustody
};
use sp_runtime::{Perbill, RuntimeDebug};
use sp_std::prelude::*;
use sp_std::convert::TryFrom;
use codec::{Encode, Decode, HasCompact};

use frame_support::traits::{
    Currency, ReservableCurrency, Get, ExistenceRequirement::AllowDeath,
    fungible::Inspect,
};
use sp_runtime::traits::{
    Zero, Saturating, AtLeast32BitUnsigned,
    Convert, StaticLookup, SaturatedConversion
};
use frame_support::{StorageValue, StorageMap, dispatch::DispatchResult};

/// Custody Info
#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, scale_info::TypeInfo)]
pub struct CustodyInfo<AccountId, Balance: HasCompact> {
    /// Allocation
    #[codec(compact)]
    pub allocation: Balance,
    /// Current vested amount (increases as payouts are given)
    #[codec(compact)]
    pub vested: Balance,
    /// Custody account
    pub custody: AccountId,
    /// Reserve account
    pub reserve: AccountId,
}

impl<AccountId, Balance> CustodyInfo<AccountId, Balance> where
    Balance: AtLeast32BitUnsigned + Saturating + Copy,
{
    /// Increase vested amount in self
    fn increase_vested(&mut self, amount: Balance) {
        self.vested += amount;
    }
}

/// Implement Custody sub module functions
impl<T: Config> Module<T> {
    /// Get payout frequency
    pub fn payout_frequency() -> T::BlockNumber {
        T::PayoutFrequency::get()
    }

    /// Get the custody duration
    pub fn custody_duration() -> T::BlockNumber {
        T::CustodyDuration::get()
    }

    /// Get the governance custody duration
    pub fn governance_custody_duration() -> T::BlockNumber {
        T::GovernanceCustodyDuration::get()
    }

    /// Check if custody is done
    pub fn is_custody_done(block: T::BlockNumber) -> bool {
        block > Self::custody_duration()
    }

    /// Check if governance custody is done
    pub fn is_governance_custody_done(block: T::BlockNumber) -> bool {
        block > Self::governance_custody_duration()
    }

    /// Initialize custody for a given account and allocation
    pub fn initialize_custody(who: &T::AccountId, amount: BalanceOf<T>) {
        // 1. Split allocation: 5.55555556% -> reserve, 94.4444444% -> custody
        // NOTE: allocation here is 90% of actual team member allocation, so the reserve
        // has 5.55555556% * 0.9 = 5% of total funds
        let reserve_ratio = Perbill::from_rational(55_555_556u32, 1_000_000_000u32);
        let reserve_balance = reserve_ratio * (amount);
        let custody_balance = amount - reserve_balance;

        // 2. Create custody account
        // Generate custody account for team member using
        // the anonymous account function from proxy pallet
        let custody_account =
            <pallet_proxy::Pallet<T>>::pure_account(
                who,
                &T::CustodyProxy::get(),
                0,
                None
            );

        // Deposit amount in custody account
        let _ = <T as Config>::Currency::deposit_creating(
            &custody_account,
            custody_balance
        );

        // 3. Create reserve account
        // Generate reserve account for team member using
        // the anonymous account function from proxy pallet
        let reserve_account =
            <pallet_proxy::Pallet<T>>::pure_account(
                who,
                &T::CustodyProxy::get(),
                1,
                None
            );

        // Deposit amount in reserve account
        let _ = <T as Config>::Currency::deposit_creating(
            &reserve_account.clone(),
            reserve_balance
        );

        // 4. Create custody info
        let custody_info = CustodyInfo {
            allocation: amount,
            vested:  Zero::zero(),
            custody: custody_account.clone(),
            reserve: reserve_account,
        };

        // 5. Store custody info and custody account
        <TeamAccounts<T>>::insert(who, custody_info);
        <CustodyAccounts<T>>::insert(&custody_account, ());

        // 6. Update total amount under custody
        <TotalCustody<T>>::mutate(|n| *n += custody_balance);
        <TotalCustody<T>>::mutate(|n| *n += reserve_balance);
    }

    /// Set the governance proxy of given custody account
    fn set_custody_governance_proxy(custody: &T::AccountId, proxy: T::AccountId) -> DispatchResult {
        // 1. Remove any proxies
        Self::remove_custody_proxies(custody);
        // 2. Set new proxy
        <pallet_proxy::Pallet<T>>::add_proxy_delegate(
            custody,
            proxy,
            T::CustodyProxy::get(),
            Zero::zero()
        )
    }

    /// Remove any proxies of given custody account, refunding team member's deposit if existing
    /// NOTE: Any proxies are guaranteed to be only of Governance type
    fn remove_custody_proxies(custody: &T::AccountId) {
        // Can't call remove_proxies directly, so need to replicate code here
        let (_, old_deposit) = <pallet_proxy::Proxies::<T>>::take(custody);
        <T as pallet_proxy::Config>::Currency::unreserve(custody, old_deposit.clone());
    }

    /// Compute payout
    fn compute_payout(allocation: BalanceOf<T>) -> BalanceOf<T> {
        let payout_ratio = Perbill::from_rational(
            Self::payout_frequency(),
            Self::custody_duration()
        );
        payout_ratio * allocation
    }

    /// Attempt a payout to the given team member account
    pub fn try_payout(who: T::AccountId) -> DispatchResult {
        // 1. Get the block number from the FRAME System module.
        let block = <frame_system::Pallet<T>>::block_number();

        // 2. Get custody info for team member
        // (can't fail because team member existing is checked before)
        let info = <TeamAccounts<T>>::get(&who).unwrap();

        // 3. If custody is over, payout full remaining amount
        if Self::is_custody_done(block) {
            // 3.1. If any leftover custody amount is still bonded, force unstake
            let custody = info.custody.clone();
            if <pallet_staking::Bonded<T>>::contains_key(&custody) {
                <pallet_staking::Pallet<T>>::force_unstake(
                    frame_system::RawOrigin::Root.into(),
                    custody.clone(),
                    Zero::zero(),
                )?;
            }
            // 3.2. Remove any proxies on the custody account
            Self::remove_custody_proxies(&custody);
            // 3.3. Payout remaining amount
            Self::do_payout(who.clone(), Zero::zero(), info, false)?;
            // 3.4. Emmit custody done event
            Self::deposit_event(RawEvent::CustodyDone(who));
            return Ok(())
        }

        // 4. Compute payout according to block
        let payout = Self::compute_payout(info.allocation);
        let chunks = block / Self::payout_frequency();
        let chunks = <T as Config>::BlockNumberToBalance::convert(chunks);
        let amount = payout * chunks - info.vested;

        if !amount.is_zero() {
            // 4.1. Do payout
            // NOTE: function does nothing if custody account has only the existential deposit
            // transferable balance and reserve funds are exhausted
            Self::do_payout(who.clone(), amount, info, true)?;
        } else {
            // 4.2. Payout not available
            Err(Error::<T>::PayoutNotAvailable)?
        }
        Ok(())
    }

    /// Do a payout
    fn do_payout(
        who: T::AccountId, amount: BalanceOf<T>,
        mut info: CustodyInfo<T::AccountId, BalanceOf<T>>,
        keep_alive: bool,
    ) -> DispatchResult {
        // 1. Get custody transferable balance
        // Use Inspect trait here to get transferable balance of Custody account, and use keep
        // alive to limit transfers down to existential deposit until end of the custody period
        let custody = info.custody.clone();
        let custody_transferable_balance =
            <T as Config>::Currency::reducible_balance(&custody, keep_alive.clone());
        // T::Currency and T::Inspect are both implemented by Balances pallet, so the
        // balance type is the same. However, explicit conversion is needed here.
        let custody_balance = <BalanceOf<T>>::try_from(
                custody_transferable_balance.saturated_into::<u128>()
            ).ok().unwrap_or(Zero::zero());

        // 2. Get reserve balance
        // Reserve account is never used in any Reservable or Lockable Currency operations
        // However, if funds are taken from the Reserve, we could have a payout that
        // leaves dust in the account. This will lead to loss of custody funds, meaning the
        // team member never fully vests.
        // In order to prevent this, use the Inspect trait here, and use keep alive to
        // limit transfers down to existential deposit until end of the custody period
        let reserve = info.reserve.clone();
        let reserve_transferable_balance =
            <T as Config>::Currency::reducible_balance(&reserve, keep_alive.clone());
        // T::Currency and T::Inspect are both implemented by Balances pallet, so the
        // balance type is the same. However, explicit conversion is needed here.
        let reserve_balance = <BalanceOf<T>>::try_from(
                reserve_transferable_balance.saturated_into::<u128>()
            ).ok().unwrap_or(Zero::zero());

        // 3. Calculate amounts to withdraw from custody and reserve
        // If custody period is done, transfer full amount from both accounts
        // in order to not leave any inaccessible funds around
        let (withdraw_custody, withdraw_reserve) = if keep_alive {
            let from_custody = amount.min(custody_balance);
            let from_reserve = amount - from_custody;
            (from_custody, from_reserve.min(reserve_balance))
        } else {
            (custody_balance, reserve_balance)
        };
        let withdraw = withdraw_custody + withdraw_reserve;

        // 4. Make transfer from custody, if possible
        if !withdraw_custody.is_zero() {
            // Transfer from custody to team member account
            <T as Config>::Currency::transfer(
                &custody,
                &who,
                withdraw_custody.into(),
                AllowDeath
            )?;
            // Emmit TeamPayoutCustody event
            Self::deposit_event(RawEvent::PayoutFromCustody(who.clone(), withdraw_custody));
        }

        // 5. Make transfer from reserve, if possible
        if !withdraw_reserve.is_zero() {
            // Transfer from reserve to team member account
            <T as Config>::Currency::transfer(
                &reserve,
                &who,
                withdraw_reserve.into(),
                AllowDeath
            )?;
            // Emmit TeamPayoutReserve event
            Self::deposit_event(RawEvent::PayoutFromReserve(who.clone(), withdraw_reserve));
        }

        // 6. Error if no payout is possible (only before custody ends)
        if keep_alive && withdraw.is_zero() {
            Err(Error::<T>::PayoutFailedInsufficientFunds)?
        }

        // 7. Update custody info
        // Increase vested amount
        info.increase_vested(withdraw);

        // Adjust total custody
        let custody_deduct = if keep_alive {
            // If its not the last payout, then use the total withdraw
            withdraw
        } else {
            // At the last payout, make sure to square total custody correctly based
            // on difference between vested and allocation
            if info.allocation < info.vested {
                // If more was paid out than was allocated, make sure to deduct the difference
                let diff = info.vested - info.allocation;
                withdraw.saturating_sub(diff)
            } else if info.allocation > info.vested {
                // If not enough was paid out compared to allocation, make sure to add the difference
                let diff = info.allocation - info.vested;
                withdraw.saturating_add(diff)
            } else {
                // Total paid equals allocation, all good
                withdraw
            }
        };

        // Note: The logic above has been tested and results in exact amounts being subtracted
        // from TotalCustody. Still, use saturating sub here just to be safe and not have panics
        <TotalCustody<T>>::mutate(|n| *n = n.clone().saturating_sub(custody_deduct));

        // Update or delete info in storage
        if keep_alive {
            // Insert updated custody info
            <TeamAccounts<T>>::insert(&who, info);
        } else {
            // Delete team member custody info
            <TeamAccounts<T>>::remove(&who);
            // Delete custody account
            <CustodyAccounts<T>>::remove(&info.custody);
        }

        Ok(())
    }

    /// Check if the custody period is active, return error if not
    fn check_custody() -> DispatchResult {
        // Get the block number from the FRAME System module.
        let block = <frame_system::Pallet<T>>::block_number();
        // If custody should be active but is done, return error
        if Self::is_custody_done(block) {
            Err(Error::<T>::CustodyPeriodEnded)?
        }
        Ok(())
    }

    /// Check if the governance custody period is active/done, return appropriate error
    fn check_governance_custody(active: bool) -> DispatchResult {
        // Get the block number from the FRAME System module.
        let block = <frame_system::Pallet<T>>::block_number();
        if active {
            // If governance custody should be active but is done, return error
            if Self::is_governance_custody_done(block) {
                Err(Error::<T>::GovernanceCustodyPeriodEnded)?
            }
        } else {
            // If governance custody should be done but is active, return error
            if !Self::is_governance_custody_done(block) {
                Err(Error::<T>::GovernanceCustodyActive)?
            }
        }
        Ok(())
    }

    /// Attempt to bond funds from a custody account
    pub fn try_custody_bond(
        custody: T::AccountId,
        controller: T::AccountId,
        value: pallet_staking::BalanceOf<T>,
    ) -> DispatchResult {
        // 1. Return error if custody done
        Self::check_custody()?;

        // 2. Call bond function
        <pallet_staking::Pallet<T>>::bond(
            T::RuntimeOrigin::from(Some(custody).into()),
            T::Lookup::unlookup(controller),
            value.into(),
            None,
        )
    }

    /// Attempt to bond extra funds from a custody account
    pub fn try_custody_bond_extra(
        custody: T::AccountId,
        value: pallet_staking::BalanceOf<T>,
    ) -> DispatchResult {
        // 1. Return error if custody done
        Self::check_custody()?;

        // 2. Call bond extra function
        <pallet_staking::Pallet<T>>::bond_extra(
            T::RuntimeOrigin::from(Some(custody).into()),
            value.into()
        )
    }

    /// Attempt to set the staking controller of a custody account
    pub fn try_custody_set_controller(
        custody: T::AccountId,
        controller: T::AccountId,
    ) -> DispatchResult {
        // 1. Return error if custody done
        Self::check_custody()?;

        // 2. Call set controller function
        <pallet_staking::Pallet<T>>::set_controller(
            T::RuntimeOrigin::from(Some(custody).into()),
            T::Lookup::unlookup(controller)
        )
    }

    /// Attempt to set a governance proxy of a given custody account
    pub fn try_custody_set_proxy(
        custody: T::AccountId,
        proxy: T::AccountId,
    ) -> DispatchResult {
        // 1. Return error if governance custody done
        Self::check_governance_custody(true)?;

        // 2. Set new governance proxy (removes any previous existing ones)
        Self::set_custody_governance_proxy(&custody, proxy)
    }

    /// Attempt to set a governance proxy of a team member's own custody account
    pub fn try_team_custody_set_proxy(who: T::AccountId, proxy: T::AccountId) -> DispatchResult {
        // 1. Return error if governance custody is not done
        Self::check_governance_custody(false)?;

        // 2. Get team member custody account
        // (can't fail because team member existing is checked before)
        let info = <TeamAccounts<T>>::get(&who).unwrap();

        // 3. Set new governance proxy (removes any previous existing ones)
        Self::set_custody_governance_proxy(&info.custody, proxy)
    }

    /// Update a team member account
    pub fn update_team_member(who: T::AccountId, new: T::AccountId) {
        // 1. Take info from team accounts
        let info = <TeamAccounts<T>>::take(&who).unwrap();
        // 2. Insert info in new account
        <TeamAccounts<T>>::insert(&new, info);
    }
}
