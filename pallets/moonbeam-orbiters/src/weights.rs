// Copyright 2019-2022 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_moonbeam_orbiters
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-01-19, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/release/moonbeam
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// *
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --template=./benchmarking/frame-weight-template.hbs
// --json-file
// raw.json
// --output
// ./tmp/

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_moonbeam_orbiters.
pub trait WeightInfo {
	#[rustfmt::skip]
	fn collator_add_orbiter() -> Weight;
	#[rustfmt::skip]
	fn collator_remove_orbiter() -> Weight;
	#[rustfmt::skip]
	fn orbiter_leave_collator_pool() -> Weight;
	#[rustfmt::skip]
	fn orbiter_register() -> Weight;
	#[rustfmt::skip]
	fn orbiter_unregister(n: u32, ) -> Weight;
	#[rustfmt::skip]
	fn add_collator() -> Weight;
	#[rustfmt::skip]
	fn remove_collator() -> Weight;
	#[rustfmt::skip]
	fn on_initialize(x: u32, ) -> Weight;
	#[rustfmt::skip]
	fn distribute_rewards() -> Weight;
	#[rustfmt::skip]
	fn on_new_round() -> Weight;
}

/// Weights for pallet_moonbeam_orbiters using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: MoonbeamOrbiters CollatorsPool (r:1 w:1)
	// Storage: Balances Reserves (r:1 w:0)
	#[rustfmt::skip]
	fn collator_add_orbiter() -> Weight {
		Weight::from_ref_time(47_129_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: MoonbeamOrbiters CollatorsPool (r:1 w:1)
	#[rustfmt::skip]
	fn collator_remove_orbiter() -> Weight {
		Weight::from_ref_time(40_774_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: MoonbeamOrbiters CollatorsPool (r:1 w:1)
	#[rustfmt::skip]
	fn orbiter_leave_collator_pool() -> Weight {
		Weight::from_ref_time(39_782_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: MoonbeamOrbiters MinOrbiterDeposit (r:1 w:0)
	// Storage: Balances Reserves (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: MoonbeamOrbiters RegisteredOrbiter (r:0 w:1)
	#[rustfmt::skip]
	fn orbiter_register() -> Weight {
		Weight::from_ref_time(57_964_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: MoonbeamOrbiters CounterForCollatorsPool (r:1 w:0)
	// Storage: MoonbeamOrbiters CollatorsPool (r:1 w:0)
	// Storage: Balances Reserves (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: MoonbeamOrbiters RegisteredOrbiter (r:0 w:1)
	#[rustfmt::skip]
	fn orbiter_unregister(n: u32, ) -> Weight {
		Weight::from_ref_time(62_482_870 as u64)
			// Standard Error: 8_132
			.saturating_add(Weight::from_ref_time(6_912_586 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: MoonbeamOrbiters CollatorsPool (r:1 w:1)
	// Storage: MoonbeamOrbiters CounterForCollatorsPool (r:1 w:1)
	#[rustfmt::skip]
	fn add_collator() -> Weight {
		Weight::from_ref_time(24_099_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: MoonbeamOrbiters CollatorsPool (r:1 w:1)
	// Storage: MoonbeamOrbiters CounterForCollatorsPool (r:1 w:1)
	// Storage: MoonbeamOrbiters AccountLookupOverride (r:0 w:9)
	#[rustfmt::skip]
	fn remove_collator() -> Weight {
		Weight::from_ref_time(39_193_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(11 as u64))
	}
	// Storage: MoonbeamOrbiters CurrentRound (r:1 w:0)
	// Storage: MoonbeamOrbiters OrbiterPerRound (r:0 w:2)
	#[rustfmt::skip]
	fn on_initialize(x: u32, ) -> Weight {
		Weight::from_ref_time(20_040_331 as u64)
			// Standard Error: 1_234
			.saturating_add(Weight::from_ref_time(1_018_554 as u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(x as u64)))
	}
	// Storage: MoonbeamOrbiters OrbiterPerRound (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	#[rustfmt::skip]
	fn distribute_rewards() -> Weight {
		Weight::from_ref_time(43_995_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: MoonbeamOrbiters ForceRotation (r:1 w:1)
	// Storage: MoonbeamOrbiters CollatorsPool (r:2 w:1)
	// Storage: MoonbeamOrbiters OrbiterPerRound (r:0 w:3)
	// Storage: MoonbeamOrbiters AccountLookupOverride (r:0 w:3)
	// Storage: MoonbeamOrbiters CurrentRound (r:0 w:1)
	#[rustfmt::skip]
	fn on_new_round() -> Weight {
		Weight::from_ref_time(52_427_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(9 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: MoonbeamOrbiters CollatorsPool (r:1 w:1)
	// Storage: Balances Reserves (r:1 w:0)
	#[rustfmt::skip]
	fn collator_add_orbiter() -> Weight {
		Weight::from_ref_time(47_129_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: MoonbeamOrbiters CollatorsPool (r:1 w:1)
	#[rustfmt::skip]
	fn collator_remove_orbiter() -> Weight {
		Weight::from_ref_time(40_774_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: MoonbeamOrbiters CollatorsPool (r:1 w:1)
	#[rustfmt::skip]
	fn orbiter_leave_collator_pool() -> Weight {
		Weight::from_ref_time(39_782_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: MoonbeamOrbiters MinOrbiterDeposit (r:1 w:0)
	// Storage: Balances Reserves (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: MoonbeamOrbiters RegisteredOrbiter (r:0 w:1)
	#[rustfmt::skip]
	fn orbiter_register() -> Weight {
		Weight::from_ref_time(57_964_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: MoonbeamOrbiters CounterForCollatorsPool (r:1 w:0)
	// Storage: MoonbeamOrbiters CollatorsPool (r:1 w:0)
	// Storage: Balances Reserves (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: MoonbeamOrbiters RegisteredOrbiter (r:0 w:1)
	#[rustfmt::skip]
	fn orbiter_unregister(n: u32, ) -> Weight {
		Weight::from_ref_time(62_482_870 as u64)
			// Standard Error: 8_132
			.saturating_add(Weight::from_ref_time(6_912_586 as u64).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().reads((1 as u64).saturating_mul(n as u64)))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: MoonbeamOrbiters CollatorsPool (r:1 w:1)
	// Storage: MoonbeamOrbiters CounterForCollatorsPool (r:1 w:1)
	#[rustfmt::skip]
	fn add_collator() -> Weight {
		Weight::from_ref_time(24_099_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: MoonbeamOrbiters CollatorsPool (r:1 w:1)
	// Storage: MoonbeamOrbiters CounterForCollatorsPool (r:1 w:1)
	// Storage: MoonbeamOrbiters AccountLookupOverride (r:0 w:9)
	#[rustfmt::skip]
	fn remove_collator() -> Weight {
		Weight::from_ref_time(39_193_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(11 as u64))
	}
	// Storage: MoonbeamOrbiters CurrentRound (r:1 w:0)
	// Storage: MoonbeamOrbiters OrbiterPerRound (r:0 w:2)
	#[rustfmt::skip]
	fn on_initialize(x: u32, ) -> Weight {
		Weight::from_ref_time(20_040_331 as u64)
			// Standard Error: 1_234
			.saturating_add(Weight::from_ref_time(1_018_554 as u64).saturating_mul(x as u64))
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(x as u64)))
	}
	// Storage: MoonbeamOrbiters OrbiterPerRound (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	#[rustfmt::skip]
	fn distribute_rewards() -> Weight {
		Weight::from_ref_time(43_995_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: MoonbeamOrbiters ForceRotation (r:1 w:1)
	// Storage: MoonbeamOrbiters CollatorsPool (r:2 w:1)
	// Storage: MoonbeamOrbiters OrbiterPerRound (r:0 w:3)
	// Storage: MoonbeamOrbiters AccountLookupOverride (r:0 w:3)
	// Storage: MoonbeamOrbiters CurrentRound (r:0 w:1)
	#[rustfmt::skip]
	fn on_new_round() -> Weight {
		Weight::from_ref_time(52_427_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(9 as u64))
	}
}
