// Copyright 2020-2023 Manta Network.
// This file is part of Manta.
//
// Manta is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Manta is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Manta.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for manta_collator_selection
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-23, STEPS: `50`, REPEAT: 40, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("calamari-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/manta
// benchmark
// pallet
// --chain=calamari-dev
// --steps=50
// --repeat=40
// --pallet=manta_collator_selection
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./scripts/benchmarking/frame-weights-output/manta_collator_selection.rs
// --template=.github/resources/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;
use manta_primitives::constants::RocksDbWeight;

/// Weight functions needed for manta_collator_selection.
pub trait WeightInfo {
    fn set_invulnerables(b: u32, ) -> Weight;
    fn set_desired_candidates() -> Weight;
    fn set_candidacy_bond() -> Weight;
    fn set_eviction_baseline() -> Weight;
    fn set_eviction_tolerance() -> Weight;
    fn register_as_candidate(c: u32, ) -> Weight;
    fn leave_intent(c: u32, ) -> Weight;
    fn remove_collator(c: u32, ) -> Weight;
    fn register_candidate(c: u32, ) -> Weight;
    fn note_author() -> Weight;
    fn new_session(c: u32, ) -> Weight;
}

/// Weights for manta_collator_selection using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> manta_collator_selection::WeightInfo for SubstrateWeight<T> {
	// Storage: CollatorSelection Invulnerables (r:0 w:1)
	/// The range of component `b` is `[1, 5]`.
	fn set_invulnerables(b: u32, ) -> Weight {
		// Minimum execution time: 13_105 nanoseconds.
		Weight::from_ref_time(14_626_712)
			// Standard Error: 4_537
			.saturating_add(Weight::from_ref_time(47_669).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: CollatorSelection DesiredCandidates (r:0 w:1)
	fn set_desired_candidates() -> Weight {
		// Minimum execution time: 46_857 nanoseconds.
		Weight::from_ref_time(47_700_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: CollatorSelection CandidacyBond (r:0 w:1)
	fn set_candidacy_bond() -> Weight {
		// Minimum execution time: 17_664 nanoseconds.
		Weight::from_ref_time(18_157_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: CollatorSelection EvictionBaseline (r:0 w:1)
	fn set_eviction_baseline() -> Weight {
		// Minimum execution time: 13_324 nanoseconds.
		Weight::from_ref_time(13_679_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: CollatorSelection EvictionTolerance (r:0 w:1)
	fn set_eviction_tolerance() -> Weight {
		// Minimum execution time: 12_716 nanoseconds.
		Weight::from_ref_time(13_609_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: CollatorSelection Candidates (r:1 w:1)
	// Storage: CollatorSelection DesiredCandidates (r:1 w:0)
	// Storage: CollatorSelection Invulnerables (r:1 w:0)
	// Storage: Session NextKeys (r:1 w:0)
	// Storage: CollatorSelection CandidacyBond (r:1 w:0)
	/// The range of component `c` is `[1, 50]`.
	fn register_as_candidate(c: u32, ) -> Weight {
		// Minimum execution time: 45_562 nanoseconds.
		Weight::from_ref_time(50_290_799)
			// Standard Error: 2_858
			.saturating_add(Weight::from_ref_time(245_007).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: CollatorSelection Candidates (r:1 w:1)
	/// The range of component `c` is `[1, 50]`.
	fn leave_intent(c: u32, ) -> Weight {
		// Minimum execution time: 31_018 nanoseconds.
		Weight::from_ref_time(34_441_449)
			// Standard Error: 2_413
			.saturating_add(Weight::from_ref_time(200_694).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: CollatorSelection Invulnerables (r:1 w:0)
	// Storage: CollatorSelection Candidates (r:1 w:1)
	/// The range of component `c` is `[1, 50]`.
	fn remove_collator(c: u32, ) -> Weight {
		// Minimum execution time: 33_220 nanoseconds.
		Weight::from_ref_time(37_414_399)
			// Standard Error: 2_878
			.saturating_add(Weight::from_ref_time(205_481).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: CollatorSelection Candidates (r:1 w:1)
	// Storage: CollatorSelection DesiredCandidates (r:1 w:0)
	// Storage: CollatorSelection Invulnerables (r:1 w:0)
	// Storage: Session NextKeys (r:1 w:0)
	// Storage: CollatorSelection CandidacyBond (r:1 w:0)
	/// The range of component `c` is `[1, 50]`.
	fn register_candidate(c: u32, ) -> Weight {
		// Minimum execution time: 45_756 nanoseconds.
		Weight::from_ref_time(50_548_470)
			// Standard Error: 3_033
			.saturating_add(Weight::from_ref_time(228_898).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: System Account (r:2 w:2)
	// Storage: CollatorSelection BlocksPerCollatorThisSession (r:1 w:1)
	// Storage: System BlockWeight (r:1 w:1)
	fn note_author() -> Weight {
		// Minimum execution time: 34_166 nanoseconds.
		Weight::from_ref_time(34_795_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: CollatorSelection Candidates (r:1 w:0)
	// Storage: CollatorSelection EvictionBaseline (r:1 w:0)
	// Storage: CollatorSelection EvictionTolerance (r:1 w:0)
	// Storage: CollatorSelection BlocksPerCollatorThisSession (r:2 w:2)
	// Storage: CollatorSelection Invulnerables (r:1 w:0)
	// Storage: System BlockWeight (r:1 w:1)
	// Storage: Session Validators (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	/// The range of component `c` is `[1, 50]`.
	fn new_session(c: u32, ) -> Weight {
		// Minimum execution time: 37_790 nanoseconds.
		Weight::from_ref_time(25_322_154)
			// Standard Error: 49_419
			.saturating_add(Weight::from_ref_time(22_066_782).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: CollatorSelection Invulnerables (r:0 w:1)
	/// The range of component `b` is `[1, 5]`.
	fn set_invulnerables(b: u32, ) -> Weight {
		// Minimum execution time: 13_105 nanoseconds.
		Weight::from_ref_time(14_626_712)
			// Standard Error: 4_537
			.saturating_add(Weight::from_ref_time(47_669).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: CollatorSelection DesiredCandidates (r:0 w:1)
	fn set_desired_candidates() -> Weight {
		// Minimum execution time: 46_857 nanoseconds.
		Weight::from_ref_time(47_700_000)
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: CollatorSelection CandidacyBond (r:0 w:1)
	fn set_candidacy_bond() -> Weight {
		// Minimum execution time: 17_664 nanoseconds.
		Weight::from_ref_time(18_157_000)
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: CollatorSelection EvictionBaseline (r:0 w:1)
	fn set_eviction_baseline() -> Weight {
		// Minimum execution time: 13_324 nanoseconds.
		Weight::from_ref_time(13_679_000)
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: CollatorSelection EvictionTolerance (r:0 w:1)
	fn set_eviction_tolerance() -> Weight {
		// Minimum execution time: 12_716 nanoseconds.
		Weight::from_ref_time(13_609_000)
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: CollatorSelection Candidates (r:1 w:1)
	// Storage: CollatorSelection DesiredCandidates (r:1 w:0)
	// Storage: CollatorSelection Invulnerables (r:1 w:0)
	// Storage: Session NextKeys (r:1 w:0)
	// Storage: CollatorSelection CandidacyBond (r:1 w:0)
	/// The range of component `c` is `[1, 50]`.
	fn register_as_candidate(c: u32, ) -> Weight {
		// Minimum execution time: 45_562 nanoseconds.
		Weight::from_ref_time(50_290_799)
			// Standard Error: 2_858
			.saturating_add(Weight::from_ref_time(245_007).saturating_mul(c.into()))
			.saturating_add(RocksDbWeight::get().reads(5))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: CollatorSelection Candidates (r:1 w:1)
	/// The range of component `c` is `[1, 50]`.
	fn leave_intent(c: u32, ) -> Weight {
		// Minimum execution time: 31_018 nanoseconds.
		Weight::from_ref_time(34_441_449)
			// Standard Error: 2_413
			.saturating_add(Weight::from_ref_time(200_694).saturating_mul(c.into()))
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: CollatorSelection Invulnerables (r:1 w:0)
	// Storage: CollatorSelection Candidates (r:1 w:1)
	/// The range of component `c` is `[1, 50]`.
	fn remove_collator(c: u32, ) -> Weight {
		// Minimum execution time: 33_220 nanoseconds.
		Weight::from_ref_time(37_414_399)
			// Standard Error: 2_878
			.saturating_add(Weight::from_ref_time(205_481).saturating_mul(c.into()))
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: CollatorSelection Candidates (r:1 w:1)
	// Storage: CollatorSelection DesiredCandidates (r:1 w:0)
	// Storage: CollatorSelection Invulnerables (r:1 w:0)
	// Storage: Session NextKeys (r:1 w:0)
	// Storage: CollatorSelection CandidacyBond (r:1 w:0)
	/// The range of component `c` is `[1, 50]`.
	fn register_candidate(c: u32, ) -> Weight {
		// Minimum execution time: 45_756 nanoseconds.
		Weight::from_ref_time(50_548_470)
			// Standard Error: 3_033
			.saturating_add(Weight::from_ref_time(228_898).saturating_mul(c.into()))
			.saturating_add(RocksDbWeight::get().reads(5))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: System Account (r:2 w:2)
	// Storage: CollatorSelection BlocksPerCollatorThisSession (r:1 w:1)
	// Storage: System BlockWeight (r:1 w:1)
	fn note_author() -> Weight {
		// Minimum execution time: 34_166 nanoseconds.
		Weight::from_ref_time(34_795_000)
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(4))
	}
	// Storage: CollatorSelection Candidates (r:1 w:0)
	// Storage: CollatorSelection EvictionBaseline (r:1 w:0)
	// Storage: CollatorSelection EvictionTolerance (r:1 w:0)
	// Storage: CollatorSelection BlocksPerCollatorThisSession (r:2 w:2)
	// Storage: CollatorSelection Invulnerables (r:1 w:0)
	// Storage: System BlockWeight (r:1 w:1)
	// Storage: Session Validators (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	/// The range of component `c` is `[1, 50]`.
	fn new_session(c: u32, ) -> Weight {
		// Minimum execution time: 37_790 nanoseconds.
		Weight::from_ref_time(25_322_154)
			// Standard Error: 49_419
			.saturating_add(Weight::from_ref_time(22_066_782).saturating_mul(c.into()))
			.saturating_add(RocksDbWeight::get().reads(6))
			.saturating_add(RocksDbWeight::get().reads((2_u64).saturating_mul(c.into())))
			.saturating_add(RocksDbWeight::get().writes(3))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(c.into())))
	}
}
