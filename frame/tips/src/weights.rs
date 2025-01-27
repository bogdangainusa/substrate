// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_tips
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-05-24, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/substrate
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_tips
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --template=./.maintain/frame-weight-template.hbs
// --output=./frame/tips/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{RefTimeWeight, Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_tips.
pub trait WeightInfo {
	fn report_awesome(r: u32, ) -> Weight;
	fn retract_tip() -> Weight;
	fn tip_new(r: u32, t: u32, ) -> Weight;
	fn tip(t: u32, ) -> Weight;
	fn close_tip(t: u32, ) -> Weight;
	fn slash_tip(t: u32, ) -> Weight;
}

/// Weights for pallet_tips using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Tips Reasons (r:1 w:1)
	// Storage: Tips Tips (r:1 w:1)
	fn report_awesome(r: u32, ) -> Weight {
		Weight::from_ref_time(30_669_000 as RefTimeWeight)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(4_000 as RefTimeWeight).saturating_mul(r as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Tips Tips (r:1 w:1)
	// Storage: Tips Reasons (r:0 w:1)
	fn retract_tip() -> Weight {
		Weight::from_ref_time(28_768_000 as RefTimeWeight)
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Elections Members (r:1 w:0)
	// Storage: Tips Reasons (r:1 w:1)
	// Storage: Tips Tips (r:0 w:1)
	fn tip_new(r: u32, t: u32, ) -> Weight {
		Weight::from_ref_time(20_385_000 as RefTimeWeight)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(2_000 as RefTimeWeight).saturating_mul(r as RefTimeWeight))
			// Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(166_000 as RefTimeWeight).saturating_mul(t as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Elections Members (r:1 w:0)
	// Storage: Tips Tips (r:1 w:1)
	fn tip(t: u32, ) -> Weight {
		Weight::from_ref_time(12_287_000 as RefTimeWeight)
			// Standard Error: 6_000
			.saturating_add(Weight::from_ref_time(363_000 as RefTimeWeight).saturating_mul(t as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Tips Tips (r:1 w:1)
	// Storage: Elections Members (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Tips Reasons (r:0 w:1)
	fn close_tip(t: u32, ) -> Weight {
		Weight::from_ref_time(45_656_000 as RefTimeWeight)
			// Standard Error: 14_000
			.saturating_add(Weight::from_ref_time(276_000 as RefTimeWeight).saturating_mul(t as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(3 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(3 as RefTimeWeight))
	}
	// Storage: Tips Tips (r:1 w:1)
	// Storage: Tips Reasons (r:0 w:1)
	fn slash_tip(t: u32, ) -> Weight {
		Weight::from_ref_time(18_525_000 as RefTimeWeight)
			// Standard Error: 5_000
			.saturating_add(Weight::from_ref_time(37_000 as RefTimeWeight).saturating_mul(t as RefTimeWeight))
			.saturating_add(T::DbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(T::DbWeight::get().writes(2 as RefTimeWeight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Tips Reasons (r:1 w:1)
	// Storage: Tips Tips (r:1 w:1)
	fn report_awesome(r: u32, ) -> Weight {
		Weight::from_ref_time(30_669_000 as RefTimeWeight)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(4_000 as RefTimeWeight).saturating_mul(r as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Tips Tips (r:1 w:1)
	// Storage: Tips Reasons (r:0 w:1)
	fn retract_tip() -> Weight {
		Weight::from_ref_time(28_768_000 as RefTimeWeight)
			.saturating_add(RocksDbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Elections Members (r:1 w:0)
	// Storage: Tips Reasons (r:1 w:1)
	// Storage: Tips Tips (r:0 w:1)
	fn tip_new(r: u32, t: u32, ) -> Weight {
		Weight::from_ref_time(20_385_000 as RefTimeWeight)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(2_000 as RefTimeWeight).saturating_mul(r as RefTimeWeight))
			// Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(166_000 as RefTimeWeight).saturating_mul(t as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(2 as RefTimeWeight))
	}
	// Storage: Elections Members (r:1 w:0)
	// Storage: Tips Tips (r:1 w:1)
	fn tip(t: u32, ) -> Weight {
		Weight::from_ref_time(12_287_000 as RefTimeWeight)
			// Standard Error: 6_000
			.saturating_add(Weight::from_ref_time(363_000 as RefTimeWeight).saturating_mul(t as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().reads(2 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(1 as RefTimeWeight))
	}
	// Storage: Tips Tips (r:1 w:1)
	// Storage: Elections Members (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Tips Reasons (r:0 w:1)
	fn close_tip(t: u32, ) -> Weight {
		Weight::from_ref_time(45_656_000 as RefTimeWeight)
			// Standard Error: 14_000
			.saturating_add(Weight::from_ref_time(276_000 as RefTimeWeight).saturating_mul(t as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().reads(3 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(3 as RefTimeWeight))
	}
	// Storage: Tips Tips (r:1 w:1)
	// Storage: Tips Reasons (r:0 w:1)
	fn slash_tip(t: u32, ) -> Weight {
		Weight::from_ref_time(18_525_000 as RefTimeWeight)
			// Standard Error: 5_000
			.saturating_add(Weight::from_ref_time(37_000 as RefTimeWeight).saturating_mul(t as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().reads(1 as RefTimeWeight))
			.saturating_add(RocksDbWeight::get().writes(2 as RefTimeWeight))
	}
}
