// This file is part of Basilisk-node.

// Copyright (C) 2020-2021  Intergalactic, Limited (GIB).
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

use super::*;
use test_ext::*;

#[test]
fn destroy_farm_should_work() {
    predefined_test_ext().execute_with(|| {
        //transfer all rewards from farm account
        let farm_account = LiquidityMining::farm_account_id(BOB_FARM).unwrap();
        let _ = Tokens::transfer_all(
            Origin::signed(farm_account),
            TREASURY,
            PREDEFINED_GLOBAL_FARMS[1].reward_currency,
            false,
        );
        assert_eq!(
            Tokens::free_balance(PREDEFINED_GLOBAL_FARMS[1].reward_currency, &farm_account),
            0
        );

        assert_ok!(LiquidityMining::destroy_global_farm(BOB, BOB_FARM));

        assert!(LiquidityMining::global_farm(BOB_FARM).is_none());
    });
}

#[test]
fn destroy_farm_not_owner_should_not_work() {
    predefined_test_ext().execute_with(|| {
        //transfer all rewards from farm account
        let farm_account = LiquidityMining::farm_account_id(BOB_FARM).unwrap();
        let _ = Tokens::transfer_all(
            Origin::signed(farm_account),
            TREASURY,
            PREDEFINED_GLOBAL_FARMS[1].reward_currency,
            false,
        );
        assert_eq!(
            Tokens::free_balance(PREDEFINED_GLOBAL_FARMS[1].reward_currency, &farm_account),
            0
        );

        assert_noop!(LiquidityMining::destroy_global_farm(ALICE, BOB_FARM), Error::<Test>::Forbidden);

        assert_eq!(
            LiquidityMining::global_farm(BOB_FARM).unwrap(),
            PREDEFINED_GLOBAL_FARMS[1]
        );
    });
}

#[test]
fn destroy_farm_farm_not_exists_should_not_work() {
    predefined_test_ext().execute_with(|| {
        const NON_EXISTING_FARM: u32 = 999_999_999;
        assert_noop!(
            LiquidityMining::destroy_global_farm(ALICE, NON_EXISTING_FARM),
            Error::<Test>::GlobalFarmNotFound
        );
    });
}

#[test]
fn destroy_farm_with_yield_farms_should_not_work() {
    //all rewards was distributed but yield farms still exist in the global farm
    predefined_test_ext().execute_with(|| {
        //transfer all rewards from farm account
        let farm_account = LiquidityMining::farm_account_id(GC_FARM).unwrap();
        let _ = Tokens::transfer_all(
            Origin::signed(farm_account),
            TREASURY,
            PREDEFINED_GLOBAL_FARMS[2].reward_currency,
            false,
        );
        assert_eq!(
            Tokens::free_balance(PREDEFINED_GLOBAL_FARMS[2].reward_currency, &farm_account),
            0
        );

        assert_noop!(
            LiquidityMining::destroy_global_farm(GC, GC_FARM),
            Error::<Test>::GlobalFarmIsNotEmpty
        );

        assert_eq!(
            LiquidityMining::global_farm(GC_FARM).unwrap(),
            PREDEFINED_GLOBAL_FARMS[2]
        );
    });
}

#[test]
fn destroy_farm_with_undistributed_rewards_and_no_yield_farms_should_not_work() {
    //all yield farms was removed from the farm but there are undistributed rewards on farm account
    predefined_test_ext().execute_with(|| {
        let farm_account = LiquidityMining::farm_account_id(BOB_FARM).unwrap();
        assert!(!Tokens::free_balance(PREDEFINED_GLOBAL_FARMS[1].reward_currency, &farm_account).is_zero());

        assert_noop!(
            LiquidityMining::destroy_global_farm(BOB, BOB_FARM),
            Error::<Test>::RewardBalanceIsNotZero
        );

        assert_eq!(
            LiquidityMining::global_farm(BOB_FARM).unwrap(),
            PREDEFINED_GLOBAL_FARMS[1]
        );
    });
}

#[test]
fn destroy_farm_healthy_farm_should_not_work() {
    //farm with undistributed rewards and yield farms
    predefined_test_ext().execute_with(|| {
        let farm_account = LiquidityMining::farm_account_id(GC_FARM).unwrap();
        assert!(!Tokens::free_balance(PREDEFINED_GLOBAL_FARMS[2].reward_currency, &farm_account).is_zero());

        assert_noop!(
            LiquidityMining::destroy_global_farm(GC, GC_FARM),
            Error::<Test>::GlobalFarmIsNotEmpty
        );

        assert_eq!(
            LiquidityMining::global_farm(GC_FARM).unwrap(),
            PREDEFINED_GLOBAL_FARMS[2]
        );
    });
}
