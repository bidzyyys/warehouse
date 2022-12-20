// This file is part of HydraDX.

// Copyright (C) 2020-2022  Intergalactic, Limited (GIB).
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::tests::mock::*;
use crate::{Error, Event, Order, PoolType, Recurrence, Schedule, Trade};
use frame_support::{assert_noop, assert_ok};
use frame_system::pallet_prelude::BlockNumberFor;
use pretty_assertions::assert_eq;
use sp_runtime::traits::ConstU32;
use sp_runtime::BoundedVec;
use sp_runtime::DispatchError;
use sp_runtime::DispatchError::BadOrigin;

#[test]
fn schedule_should_store_schedule_for_next_block_when_no_blocknumber_specified() {
    ExtBuilder::default().build().execute_with(|| {
        //Arrange
        let trades = create_bounded_vec(vec![Trade {
            asset_in: 3,
            asset_out: 4,
            pool: PoolType::XYK,
        }]);

        let schedule = Schedule {
            period: 10,
            order: Order {
                asset_in: 3,
                asset_out: 4,
                amount_in: 1000,
                amount_out: 2000,
                limit: 0,
                route: trades,
            },
            recurrence: Recurrence::Fixed,
        };

        //Act
        assert_ok!(Dca::schedule(Origin::signed(ALICE), schedule, Option::None));

        //Assert
        let stored_schedule = Dca::schedules(1).unwrap();
        assert_eq!(
            stored_schedule,
            Schedule {
                period: 10,
                order: Order {
                    asset_in: 3,
                    asset_out: 4,
                    amount_in: 1000,
                    amount_out: 2000,
                    limit: 0,
                    route: create_bounded_vec(vec![Trade {
                        asset_in: 3,
                        asset_out: 4,
                        pool: PoolType::XYK
                    }])
                },
                recurrence: Recurrence::Fixed
            }
        )
    });
}

#[test]
fn schedule_should_use_sequencer_when_storing_schedule_in_storage() {
    ExtBuilder::default().build().execute_with(|| {
        //Arrange
        let trades = create_bounded_vec(vec![Trade {
            asset_in: 3,
            asset_out: 4,
            pool: PoolType::XYK,
        }]);

        let schedule = Schedule {
            period: 10,
            order: Order {
                asset_in: 3,
                asset_out: 4,
                amount_in: 1000,
                amount_out: 2000,
                limit: 0,
                route: trades,
            },
            recurrence: Recurrence::Fixed,
        };

        //Act
        assert_ok!(Dca::schedule(Origin::signed(ALICE), schedule.clone(), Option::None));
        assert_ok!(Dca::schedule(Origin::signed(ALICE), schedule, Option::None));

        //Assert
        assert!(Dca::schedules(1).is_some());
        assert!(Dca::schedules(2).is_some());
    });
}

fn create_bounded_vec(trades: Vec<Trade>) -> BoundedVec<Trade, ConstU32<5>> {
    let bounded_vec: BoundedVec<Trade, sp_runtime::traits::ConstU32<5>> = trades.try_into().unwrap();
    bounded_vec
}

pub fn set_block_number(n: u64) {
    System::set_block_number(n);
}

//TODO: add negative case for validating block numbers
