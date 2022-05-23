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
fn resume_liquidity_pool_should_work() {
    predefined_test_ext_with_deposits().execute_with(|| {
        //cancel liq. pool before resuming
        assert_ok!(LiquidityMining::cancel_liquidity_pool(GC, GC_FARM, BSX_TKN1_AMM));

        let liq_pool = LiquidityMining::liquidity_pool(GC_FARM, BSX_TKN1_AMM).unwrap();
        let global_pool = LiquidityMining::global_pool(GC_FARM).unwrap();

        let new_multiplier = FixedU128::from(7_490_000);

        assert!(liq_pool.canceled);
        assert!(liq_pool.stake_in_global_pool.is_zero());
        assert!(liq_pool.multiplier.is_zero());

        set_block_number(13_420_000);

        assert_ok!(LiquidityMining::resume_liquidity_pool(
            GC,
            GC_FARM,
            new_multiplier,
            BSX_TKN1_AMM,
        ));

        let liq_pool_stake_in_global_pool = new_multiplier.checked_mul_int(45_540).unwrap();

        assert_eq!(
            LiquidityMining::liquidity_pool(GC_FARM, BSX_TKN1_AMM).unwrap(),
            LiquidityPoolYieldFarm {
                canceled: false,
                stake_in_global_pool: liq_pool_stake_in_global_pool,
                accumulated_rpz: 62_996,
                multiplier: new_multiplier,
                updated_at: 134_200,
                ..liq_pool
            }
        );

        assert_eq!(
            LiquidityMining::global_pool(GC_FARM).unwrap(),
            GlobalPool {
                total_shares_z: global_pool.total_shares_z + liq_pool_stake_in_global_pool,
                updated_at: 134_200,
                accumulated_rpz: 62_996,
                accumulated_rewards: 29_999_067_250,
                ..global_pool
            }
        );
    });
}

#[test]
fn resume_liquidity_pool_non_existing_pool_should_not_work() {
    predefined_test_ext_with_deposits().execute_with(|| {
        let new_multiplier = FixedU128::from(7_490_000);

        assert_noop!(
            LiquidityMining::resume_liquidity_pool(GC, GC_FARM, new_multiplier, BSX_KSM_AMM),
            Error::<Test>::LiquidityPoolNotFound
        );
    });
}

#[test]
fn resume_liquidity_pool_non_canceled_pool_should_not_work() {
    predefined_test_ext_with_deposits().execute_with(|| {
        let new_multiplier = FixedU128::from(7_490_000);

        assert_noop!(
            LiquidityMining::resume_liquidity_pool(GC, GC_FARM, new_multiplier, BSX_TKN1_AMM),
            Error::<Test>::LiquidityMiningIsNotCanceled
        );
    });
}

#[test]
fn resume_liquidity_pool_not_owner_should_not_work() {
    predefined_test_ext_with_deposits().execute_with(|| {
        let new_multiplier = FixedU128::from(7_490_000);

        assert_noop!(
            LiquidityMining::resume_liquidity_pool(ALICE, GC_FARM, new_multiplier, BSX_TKN1_AMM),
            Error::<Test>::LiquidityMiningIsNotCanceled
        );
    });
}
