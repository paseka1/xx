
//! Autogenerated weights for `pallet_utility`
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
// --pallet=pallet_utility
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./weights/pallet_utility.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_utility`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_utility::WeightInfo for WeightInfo<T> {
	fn batch(c: u32, ) -> Weight {
		Weight::from_ref_time(23_469_000 as u64)
			// Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(6_395_000 as u64).saturating_mul(c as u64))
	}
	fn as_derivative() -> Weight {
		Weight::from_ref_time(3_156_000 as u64)
	}
	fn batch_all(c: u32, ) -> Weight {
		Weight::from_ref_time(4_614_000 as u64)
			// Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(6_867_000 as u64).saturating_mul(c as u64))
	}
	fn dispatch_as() -> Weight {
		Weight::from_ref_time(13_886_000 as u64)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn force_batch(c: u32, ) -> Weight {
		Weight::from_ref_time(19_136_000 as u64)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(2_697_000 as u64).saturating_mul(c as u64))
	}
}
