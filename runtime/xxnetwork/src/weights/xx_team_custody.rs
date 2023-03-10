
//! Autogenerated weights for `xx_team_custody`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-11-15, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("xxnetwork-dev"), DB CACHE: 128

// Executed Command:
// ./xxnetwork-chain
// benchmark
// --chain=xxnetwork-dev
// --steps=50
// --repeat=20
// --pallet=xx_team_custody
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./weights/xx_team_custody.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `xx_team_custody`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> xx_team_custody::WeightInfo for WeightInfo<T> {
	// Storage: XXCustody TeamAccounts (r:1 w:1)
	// Storage: System Account (r:3 w:3)
	// Storage: XXCustody TotalCustody (r:1 w:1)
	fn payout() -> Weight {
		Weight::from_ref_time(104_056_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: XXCustody Custodians (r:1 w:0)
	// Storage: XXCustody CustodyAccounts (r:1 w:0)
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Staking CurrentEra (r:1 w:0)
	// Storage: Staking HistoryDepth (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	fn custody_bond() -> Weight {
		Weight::from_ref_time(73_017_000 as u64)
			.saturating_add(T::DbWeight::get().reads(8 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: XXCustody Custodians (r:1 w:0)
	// Storage: XXCustody CustodyAccounts (r:1 w:0)
	// Storage: Staking Bonded (r:1 w:0)
	// Storage: Staking Ledger (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Staking Nominators (r:1 w:0)
	fn custody_bond_extra() -> Weight {
		Weight::from_ref_time(64_452_000 as u64)
			.saturating_add(T::DbWeight::get().reads(7 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: XXCustody Custodians (r:1 w:0)
	// Storage: XXCustody CustodyAccounts (r:1 w:0)
	// Storage: Staking Bonded (r:1 w:1)
	// Storage: Staking Ledger (r:2 w:2)
	fn custody_set_controller() -> Weight {
		Weight::from_ref_time(35_016_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: XXCustody Custodians (r:1 w:0)
	// Storage: XXCustody CustodyAccounts (r:1 w:0)
	// Storage: Proxy Proxies (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn custody_set_proxy() -> Weight {
		Weight::from_ref_time(48_942_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: XXCustody TeamAccounts (r:1 w:0)
	// Storage: Proxy Proxies (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn team_custody_set_proxy() -> Weight {
		Weight::from_ref_time(48_291_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: XXCustody Custodians (r:0 w:1)
	fn add_custodian() -> Weight {
		Weight::from_ref_time(15_880_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: XXCustody Custodians (r:0 w:1)
	fn remove_custodian() -> Weight {
		Weight::from_ref_time(16_071_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: XXCustody TeamAccounts (r:2 w:2)
	fn replace_team_member() -> Weight {
		Weight::from_ref_time(27_903_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}
