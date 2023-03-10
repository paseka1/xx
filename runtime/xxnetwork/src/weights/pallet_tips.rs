
//! Autogenerated weights for `pallet_tips`
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
// --pallet=pallet_tips
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./weights/pallet_tips.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_tips`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_tips::WeightInfo for WeightInfo<T> {
	// Storage: Tips Reasons (r:1 w:1)
	// Storage: Tips Tips (r:1 w:1)
	fn report_awesome(r: u32, ) -> Weight {
		Weight::from_ref_time(46_165_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(2_000 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Tips Tips (r:1 w:1)
	// Storage: Tips Reasons (r:0 w:1)
	fn retract_tip() -> Weight {
		Weight::from_ref_time(42_069_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Elections Members (r:1 w:0)
	// Storage: Tips Reasons (r:1 w:1)
	// Storage: Tips Tips (r:0 w:1)
	fn tip_new(r: u32, t: u32, ) -> Weight {
		Weight::from_ref_time(30_970_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(2_000 as u64).saturating_mul(r as u64))
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(86_000 as u64).saturating_mul(t as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Elections Members (r:1 w:0)
	// Storage: Tips Tips (r:1 w:1)
	fn tip(t: u32, ) -> Weight {
		Weight::from_ref_time(19_539_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(495_000 as u64).saturating_mul(t as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Tips Tips (r:1 w:1)
	// Storage: Elections Members (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Tips Reasons (r:0 w:1)
	fn close_tip(t: u32, ) -> Weight {
		Weight::from_ref_time(77_378_000 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(285_000 as u64).saturating_mul(t as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Tips Tips (r:1 w:1)
	// Storage: Tips Reasons (r:0 w:1)
	fn slash_tip(_t: u32, ) -> Weight {
		Weight::from_ref_time(22_638_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}
