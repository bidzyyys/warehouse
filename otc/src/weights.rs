// This file is part of galacticcouncil/warehouse.

// Copyright (C) 2020-2023  Intergalactic, Limited (GIB).
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

//! Autogenerated weights for pallet_otc
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-27, STEPS: 1, REPEAT: 1, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --extrinsic
// *
// --pallet
// pallet-otc
// --output
// ./pallets/otc/src/weights.rs
// --template=.maintain/pallet-weight-template.hbs
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_otc.
pub trait WeightInfo {
    fn place_order() -> Weight;
    fn partial_fill_order() -> Weight;
    fn fill_order() -> Weight;
    fn cancel_order() -> Weight;
}

/// Weights for pallet_otc using the hydraDX node and recommended hardware.
pub struct HydraWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
    fn place_order() -> Weight {
        Weight::from_ref_time(31_000_000 as u64)
            .saturating_add(T::DbWeight::get().reads(5 as u64))
            .saturating_add(T::DbWeight::get().writes(4 as u64))
    }
    fn partial_fill_order() -> Weight {
        Weight::from_ref_time(52_000_000 as u64)
            .saturating_add(T::DbWeight::get().reads(8 as u64))
            .saturating_add(T::DbWeight::get().writes(6 as u64))
    }
    fn fill_order() -> Weight {
        Weight::from_ref_time(49_000_000 as u64)
            .saturating_add(T::DbWeight::get().reads(7 as u64))
            .saturating_add(T::DbWeight::get().writes(6 as u64))
    }
    fn cancel_order() -> Weight {
        Weight::from_ref_time(23_000_000 as u64)
            .saturating_add(T::DbWeight::get().reads(3 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn place_order() -> Weight {
        Weight::from_ref_time(31_000_000 as u64)
            .saturating_add(RocksDbWeight::get().reads(5 as u64))
            .saturating_add(RocksDbWeight::get().writes(4 as u64))
    }
    fn partial_fill_order() -> Weight {
        Weight::from_ref_time(52_000_000 as u64)
            .saturating_add(RocksDbWeight::get().reads(8 as u64))
            .saturating_add(RocksDbWeight::get().writes(6 as u64))
    }
    fn fill_order() -> Weight {
        Weight::from_ref_time(49_000_000 as u64)
            .saturating_add(RocksDbWeight::get().reads(7 as u64))
            .saturating_add(RocksDbWeight::get().writes(6 as u64))
    }
    fn cancel_order() -> Weight {
        Weight::from_ref_time(23_000_000 as u64)
            .saturating_add(RocksDbWeight::get().reads(3 as u64))
            .saturating_add(RocksDbWeight::get().writes(3 as u64))
    }
}
