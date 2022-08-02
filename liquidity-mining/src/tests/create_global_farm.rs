// This file is part of galacticcouncil/warehouse.

// Copyright (C) 2020-2022  Intergalactic, Limited (GIB).
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

use super::*;
use test_ext::*;

#[test]
fn create_global_farm_should_work() {
    new_test_ext().execute_with(|| {
        let global_farm_id = 1;
        let total_rewards: Balance = 50_000_000_000;
        let reward_currency = BSX;
        let planned_yielding_periods: BlockNumber = 1_000_000_000_u64;
        let blocks_per_period = 20_000;
        let incentivized_token = BSX;
        let owner = ALICE;
        let yield_per_period = Perquintill::from_percent(20);
        let max_reward_per_period: Balance = total_rewards.checked_div(planned_yielding_periods.into()).unwrap();

        let created_at_block = 15_896;

        set_block_number(created_at_block);

        let global_farm_account = LiquidityMining::farm_account_id(global_farm_id).unwrap();

        assert_eq!(Tokens::free_balance(reward_currency, &global_farm_account), 0);

        assert_eq!(
            LiquidityMining::create_global_farm(
                total_rewards,
                planned_yielding_periods,
                blocks_per_period,
                incentivized_token,
                reward_currency,
                owner,
                yield_per_period,
                10,
                One::one(),
            )
            .unwrap(),
            (global_farm_id, max_reward_per_period)
        );

        //Check if total_rewards are transferred to farm's account.
        assert_eq!(
            Tokens::free_balance(reward_currency, &global_farm_account),
            total_rewards
        );
        assert_eq!(
            Tokens::free_balance(reward_currency, &ALICE),
            (INITIAL_BALANCE - total_rewards)
        );

        let updated_at = created_at_block / blocks_per_period;

        let global_farm = GlobalFarmData::new(
            global_farm_id,
            updated_at,
            reward_currency,
            yield_per_period,
            planned_yielding_periods,
            blocks_per_period,
            owner,
            incentivized_token,
            max_reward_per_period,
            10,
            One::one(),
        );

        assert_eq!(LiquidityMining::global_farm(global_farm_id).unwrap(), global_farm);
    });
}

#[test]
fn create_global_farm_invalid_data_should_not_work() {
    new_test_ext().execute_with(|| {
        let created_at_block = 15_896;

        set_block_number(created_at_block);

        //total_rewards bellow mini. limit.
        assert_noop!(
            LiquidityMining::create_global_farm(
                100,
                1_000,
                300,
                BSX,
                BSX,
                ALICE,
                Perquintill::from_percent(20),
                10,
                One::one()
            ),
            Error::<Test, Instance1>::InvalidTotalRewards
        );

        //planned_yielding_periods bellow min. limit.
        assert_noop!(
            LiquidityMining::create_global_farm(
                1_000_000,
                10,
                300,
                BSX,
                BSX,
                ALICE,
                Perquintill::from_percent(20),
                10,
                One::one()
            ),
            Error::<Test, Instance1>::InvalidPlannedYieldingPeriods
        );

        //blocks_per_period is 0.
        assert_noop!(
            LiquidityMining::create_global_farm(
                1_000_000,
                1_000,
                0,
                BSX,
                BSX,
                ALICE,
                Perquintill::from_percent(20),
                10,
                One::one()
            ),
            Error::<Test, Instance1>::InvalidBlocksPerPeriod
        );

        //yield_per_period is 0.
        assert_noop!(
            LiquidityMining::create_global_farm(
                1_000_000,
                1_000,
                1,
                BSX,
                BSX,
                ALICE,
                Perquintill::from_percent(0),
                10,
                One::one()
            ),
            Error::<Test, Instance1>::InvalidYieldPerPeriod
        );

        //min. deposit is 0.
        assert_noop!(
            LiquidityMining::create_global_farm(
                1_000_000,
                1_000,
                1,
                BSX,
                BSX,
                ALICE,
                Perquintill::from_percent(10),
                0,
                One::one()
            ),
            Error::<Test, Instance1>::InvalidMinDeposit
        );

        //price adjustment is 0.
        assert_noop!(
            LiquidityMining::create_global_farm(
                1_000_000,
                1_000,
                1,
                BSX,
                BSX,
                ALICE,
                Perquintill::from_percent(10),
                10,
                FixedU128::from(0_u128)
            ),
            Error::<Test, Instance1>::InvalidPriceAdjustment
        );
    });
}

#[test]
fn create_global_farm_with_inssufficient_balance_should_not_work() {
    //Owner's account balance is 1M BSX.
    new_test_ext().execute_with(|| {
        assert_noop!(
            LiquidityMining::create_global_farm(
                1_000_001,
                1_000,
                1,
                BSX,
                BSX,
                ACCOUNT_WITH_1M,
                Perquintill::from_percent(20),
                10,
                One::one(),
            ),
            Error::<Test, Instance1>::InsufficientRewardCurrencyBalance
        );
    });
}
