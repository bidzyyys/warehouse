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

pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut ext = ExtBuilder::default().build();
    ext.execute_with(|| set_block_number(1));
    ext
}

pub fn predefined_test_ext() -> sp_io::TestExternalities {
    let mut ext = new_test_ext();
    ext.execute_with(|| {
        assert_ok!(LiquidityMining::create_farm(
            100_000_000_000,
            PREDEFINED_GLOBAL_POOLS[0].planned_yielding_periods,
            PREDEFINED_GLOBAL_POOLS[0].blocks_per_period,
            PREDEFINED_GLOBAL_POOLS[0].incentivized_asset,
            PREDEFINED_GLOBAL_POOLS[0].reward_currency,
            ALICE,
            PREDEFINED_GLOBAL_POOLS[0].yield_per_period,
        ));

        assert_ok!(LiquidityMining::create_farm(
            1_000_000_000,
            PREDEFINED_GLOBAL_POOLS[1].planned_yielding_periods,
            PREDEFINED_GLOBAL_POOLS[1].blocks_per_period,
            PREDEFINED_GLOBAL_POOLS[1].incentivized_asset,
            PREDEFINED_GLOBAL_POOLS[1].reward_currency,
            BOB,
            PREDEFINED_GLOBAL_POOLS[1].yield_per_period,
        ));

        assert_ok!(LiquidityMining::create_farm(
            30_000_000_000,
            PREDEFINED_GLOBAL_POOLS[2].planned_yielding_periods,
            PREDEFINED_GLOBAL_POOLS[2].blocks_per_period,
            PREDEFINED_GLOBAL_POOLS[2].incentivized_asset,
            PREDEFINED_GLOBAL_POOLS[2].reward_currency,
            GC,
            PREDEFINED_GLOBAL_POOLS[2].yield_per_period,
        ));

        assert_ok!(LiquidityMining::create_farm(
            30_000_000_000,
            PREDEFINED_GLOBAL_POOLS[3].planned_yielding_periods,
            PREDEFINED_GLOBAL_POOLS[3].blocks_per_period,
            PREDEFINED_GLOBAL_POOLS[3].incentivized_asset,
            PREDEFINED_GLOBAL_POOLS[3].reward_currency,
            CHARLIE,
            PREDEFINED_GLOBAL_POOLS[3].yield_per_period,
        ));

        let amm_mock_data = vec![
            (
                AssetPair {
                    asset_in: BSX,
                    asset_out: ACA,
                },
                (BSX_ACA_AMM, BSX_ACA_SHARE_ID),
            ),
            (
                AssetPair {
                    asset_in: KSM,
                    asset_out: BSX,
                },
                (BSX_KSM_AMM, BSX_KSM_SHARE_ID),
            ),
            (
                AssetPair {
                    asset_in: BSX,
                    asset_out: DOT,
                },
                (BSX_DOT_AMM, BSX_DOT_SHARE_ID),
            ),
            (
                AssetPair {
                    asset_in: BSX,
                    asset_out: ETH,
                },
                (BSX_ETH_AMM, BSX_ETH_SHARE_ID),
            ),
            (
                AssetPair {
                    asset_in: BSX,
                    asset_out: HDX,
                },
                (BSX_HDX_AMM, BSX_HDX_SHARE_ID),
            ),
            (
                AssetPair {
                    asset_in: BSX,
                    asset_out: TKN1,
                },
                (BSX_TKN1_AMM, BSX_TKN1_SHARE_ID),
            ),
            (
                AssetPair {
                    asset_in: BSX,
                    asset_out: TKN2,
                },
                (BSX_TKN2_AMM, BSX_TKN2_SHARE_ID),
            ),
            (
                AssetPair {
                    asset_in: KSM,
                    asset_out: DOT,
                },
                (KSM_DOT_AMM, KSM_DOT_SHARE_ID),
            ),
            (
                AssetPair {
                    asset_in: ACA,
                    asset_out: KSM,
                },
                (ACA_KSM_AMM, ACA_KSM_SHARE_ID),
            ),
        ];

        AMM_POOLS.with(|h| {
            let mut hm = h.borrow_mut();
            for (k, v) in amm_mock_data {
                hm.insert(asset_pair_to_map_key(k), v);
            }
        });

        assert_ok!(LiquidityMining::add_liquidity_pool(
            GC,
            GC_FARM,
            PREDEFINED_LIQ_POOLS.with(|v| v[0].multiplier),
            PREDEFINED_LIQ_POOLS.with(|v| v[0].loyalty_curve.clone()),
            BSX_TKN1_AMM,
            BSX,
            TKN1,
        ));

        assert_eq!(
            LiquidityMining::liquidity_pool(GC_FARM, BSX_TKN1_AMM).unwrap(),
            PREDEFINED_LIQ_POOLS.with(|v| v[0].clone())
        );

        assert_ok!(LiquidityMining::add_liquidity_pool(
            GC,
            GC_FARM,
            PREDEFINED_LIQ_POOLS.with(|v| v[1].multiplier),
            PREDEFINED_LIQ_POOLS.with(|v| v[1].loyalty_curve.clone()),
            BSX_TKN2_AMM,
            BSX,
            TKN2,
        ));

        assert_eq!(
            LiquidityMining::liquidity_pool(GC_FARM, BSX_TKN2_AMM).unwrap(),
            PREDEFINED_LIQ_POOLS.with(|v| v[1].clone())
        );

        assert_ok!(LiquidityMining::add_liquidity_pool(
            CHARLIE,
            CHARLIE_FARM,
            PREDEFINED_LIQ_POOLS.with(|v| v[2].multiplier),
            PREDEFINED_LIQ_POOLS.with(|v| v[2].loyalty_curve.clone()),
            ACA_KSM_AMM,
            ACA,
            KSM,
        ));

        assert_eq!(
            LiquidityMining::liquidity_pool(CHARLIE_FARM, ACA_KSM_AMM).unwrap(),
            PREDEFINED_LIQ_POOLS.with(|v| v[2].clone())
        );

        reset_rpvs_updated();
        reset_rpz_updated();
    });

    ext
}

pub fn predefined_test_ext_with_deposits() -> sp_io::TestExternalities {
    let mut ext = predefined_test_ext();

    ext.execute_with(|| {
        let farm_id = GC_FARM; //global pool

        let bsx_tkn1_assets = AssetPair {
            asset_in: BSX,
            asset_out: TKN1,
        };

        let bsx_tkn2_assets = AssetPair {
            asset_in: BSX,
            asset_out: TKN2,
        };

        let global_pool_account = LiquidityMining::pool_account_id(GC_FARM).unwrap();
        let bsx_tkn1_liq_pool_account = LiquidityMining::pool_account_id(BSX_TKN1_LIQ_POOL_ID).unwrap();
        let bsx_tkn2_liq_pool_account = LiquidityMining::pool_account_id(BSX_TKN2_LIQ_POOL_ID).unwrap();
        let bsx_tkn1_amm_account =
            AMM_POOLS.with(|v| v.borrow().get(&asset_pair_to_map_key(bsx_tkn1_assets)).unwrap().0);
        let bsx_tkn2_amm_account =
            AMM_POOLS.with(|v| v.borrow().get(&asset_pair_to_map_key(bsx_tkn2_assets)).unwrap().0);

        //DEPOSIT 1:
        set_block_number(1_800); //18-th period

        //this is done because amount of incetivized token in AMM is used in calculations.
        Tokens::set_balance(Origin::root(), bsx_tkn1_amm_account, BSX, 50, 0).unwrap();

        let deposited_amount = 50;
        assert_ok!(LiquidityMining::deposit_shares(
            ALICE,
            farm_id,
            deposited_amount,
            BSX_TKN1_AMM
        ));

        assert!(LiquidityMining::deposit(PREDEFINED_DEPOSIT_IDS[0]).is_some());

        // DEPOSIT 2 (deposit in same period):

        //this is done because amount of incetivized token in AMM is used in calculations.
        Tokens::set_balance(Origin::root(), bsx_tkn1_amm_account, BSX, 52, 0).unwrap();

        let deposited_amount = 80;
        assert_ok!(LiquidityMining::deposit_shares(
            BOB,
            farm_id,
            deposited_amount,
            BSX_TKN1_AMM
        ));

        assert!(LiquidityMining::deposit(PREDEFINED_DEPOSIT_IDS[1]).is_some());

        // DEPOSIT 3 (same period, second liq pool yield farm):

        //this is done because amount of incetivized token in AMM is used in calculations.
        Tokens::set_balance(Origin::root(), bsx_tkn2_amm_account, BSX, 8, 0).unwrap();

        let deposited_amount = 25;
        assert_ok!(LiquidityMining::deposit_shares(
            BOB,
            farm_id,
            deposited_amount,
            BSX_TKN2_AMM,
        ));

        assert!(LiquidityMining::deposit(PREDEFINED_DEPOSIT_IDS[2]).is_some());

        // DEPOSIT 4 (new period):
        set_block_number(2051); //period 20

        //this is done because amount of incetivized token in AMM is used in calculations.
        Tokens::set_balance(Origin::root(), bsx_tkn2_amm_account, BSX, 58, 0).unwrap();

        let deposited_amount = 800;
        assert_ok!(LiquidityMining::deposit_shares(
            BOB,
            farm_id,
            deposited_amount,
            BSX_TKN2_AMM
        ));

        assert!(LiquidityMining::deposit(PREDEFINED_DEPOSIT_IDS[3]).is_some());

        // DEPOSIT 5 (same period, second liq pool yield farm):
        set_block_number(2_586); //period 25

        //this is done because amount of incetivized token in AMM is used in calculations.
        Tokens::set_balance(Origin::root(), bsx_tkn2_amm_account, BSX, 3, 0).unwrap();

        let deposited_amount = 87;
        assert_ok!(LiquidityMining::deposit_shares(
            ALICE,
            farm_id,
            deposited_amount,
            BSX_TKN2_AMM
        ));

        assert!(LiquidityMining::deposit(PREDEFINED_DEPOSIT_IDS[4]).is_some());

        // DEPOSIT 6 (same period):
        set_block_number(2_596); //period 25

        //this is done because amount of incetivized token in AMM is used in calculations.
        Tokens::set_balance(Origin::root(), bsx_tkn2_amm_account, BSX, 16, 0).unwrap();

        let deposited_amount = 48;
        assert_ok!(LiquidityMining::deposit_shares(
            ALICE,
            farm_id,
            deposited_amount,
            BSX_TKN2_AMM
        ));

        assert!(LiquidityMining::deposit(PREDEFINED_DEPOSIT_IDS[5]).is_some());

        // DEPOSIT 7 : (same period differen liq poll farm)
        set_block_number(2_596); //period 25

        //this is done because amount of incetivized token in AMM is used in calculations.
        Tokens::set_balance(Origin::root(), bsx_tkn1_amm_account, BSX, 80, 0).unwrap();

        let deposited_amount = 486;
        assert_ok!(LiquidityMining::deposit_shares(
            ALICE,
            farm_id,
            deposited_amount,
            BSX_TKN1_AMM
        ));

        assert!(LiquidityMining::deposit(PREDEFINED_DEPOSIT_IDS[6]).is_some());

        assert_eq!(
            LiquidityMining::global_pool(GC_FARM).unwrap(),
            GlobalPool {
                id: GC_FARM,
                updated_at: 25,
                reward_currency: BSX,
                yield_per_period: Permill::from_percent(50),
                planned_yielding_periods: 500_u64,
                blocks_per_period: 100_u64,
                owner: GC,
                incentivized_asset: BSX,
                max_reward_per_period: 60_000_000,
                accumulated_rpz: 12,
                liq_pools_count: 2,
                total_shares_z: 703_990,
                accumulated_rewards: 231_650,
                paid_accumulated_rewards: 1_164_400,
            }
        );

        assert_eq!(
            LiquidityMining::liquidity_pool(GC_FARM, BSX_TKN1_AMM).unwrap(),
            LiquidityPoolYieldFarm {
                updated_at: 25,
                accumulated_rpvs: 60,
                accumulated_rpz: 12,
                total_shares: 616,
                total_valued_shares: 45_540,
                stake_in_global_pool: 227_700,
                ..PREDEFINED_LIQ_POOLS.with(|v| v[0].clone())
            },
        );

        assert_eq!(
            LiquidityMining::liquidity_pool(GC_FARM, BSX_TKN2_AMM).unwrap(),
            LiquidityPoolYieldFarm {
                updated_at: 25,
                accumulated_rpvs: 120,
                accumulated_rpz: 12,
                total_shares: 960,
                total_valued_shares: 47_629,
                stake_in_global_pool: 476_290,
                ..PREDEFINED_LIQ_POOLS.with(|v| v[1].clone())
            },
        );

        //liq. pool meta check (deposits count)
        assert_eq!(
            LiquidityMining::liq_pool_meta(BSX_TKN1_LIQ_POOL_ID).unwrap(),
            (3, GC_FARM)
        );

        //liq. pool meta check (deposits count)
        assert_eq!(
            LiquidityMining::liq_pool_meta(BSX_TKN2_LIQ_POOL_ID).unwrap(),
            (4, GC_FARM)
        );

        //reward currency balance check. total_rewards - sum(claimes from global pool)
        assert_eq!(
            Tokens::free_balance(BSX, &global_pool_account),
            (30_000_000_000 - 1_164_400)
        );

        //check of claimed amount from global pool (sum of all claims)
        assert_eq!(Tokens::free_balance(BSX, &bsx_tkn1_liq_pool_account), 212_400);
        assert_eq!(Tokens::free_balance(BSX, &bsx_tkn2_liq_pool_account), 952_000);

        //balance check after transfer amm shares
        assert_eq!(Tokens::free_balance(BSX_TKN1_SHARE_ID, &ALICE), 3_000_000 - 536);
        assert_eq!(Tokens::free_balance(BSX_TKN2_SHARE_ID, &ALICE), 3_000_000 - 135);

        //balance check after transfer amm shares
        assert_eq!(Tokens::free_balance(BSX_TKN1_SHARE_ID, &BOB), 2_000_000 - 80);
        assert_eq!(Tokens::free_balance(BSX_TKN2_SHARE_ID, &BOB), 2_000_000 - 825);

        reset_rpvs_updated();
        reset_rpz_updated();
    });

    ext
}
