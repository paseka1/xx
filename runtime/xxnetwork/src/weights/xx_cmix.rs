
//! Autogenerated weights for `xx_cmix`
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
// --pallet=xx_cmix
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./weights/xx_cmix.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `xx_cmix`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> xx_cmix::WeightInfo for WeightInfo<T> {
	// Storage: XXCmix AdminPermission (r:1 w:0)
	// Storage: XXCmix CmixHashes (r:0 w:1)
	fn set_cmix_hashes() -> Weight {
		(21_992_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: XXCmix SchedulingAccount (r:0 w:1)
	fn set_scheduling_account() -> Weight {
		(14_126_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: XXCmix NextCmixVariables (r:0 w:1)
	fn set_next_cmix_variables() -> Weight {
		(3_527_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: XXCmix SchedulingAccount (r:1 w:0)
	// Storage: Staking ActiveEra (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:1)
	fn submit_cmix_points(n: u32, ) -> Weight {
		(25_583_000 as Weight)
			// Standard Error: 0
			.saturating_add((234_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: XXCmix SchedulingAccount (r:1 w:0)
	// Storage: Staking ActiveEra (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:1)
	fn submit_cmix_deductions(n: u32, ) -> Weight {
		(25_284_000 as Weight)
			// Standard Error: 0
			.saturating_add((241_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: XXCmix SchedulingAccount (r:1 w:0)
	// Storage: XXCmix CmixAddressSpace (r:0 w:1)
	fn set_cmix_address_space() -> Weight {
		(17_163_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: XXCmix AdminPermission (r:0 w:1)
	fn set_admin_permission() -> Weight {
		(14_026_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}