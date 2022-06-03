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
fn claim_rewards_should_work() {
    predefined_test_ext_with_deposits().execute_with(|| {
        const FAIL_ON_DOUBLE_CLAIM: bool = true;
        const REWARD_CURRENCY: AssetId = BSX;
        let global_farm_id = GC_FARM;
        let alice_bsx_balance = Tokens::free_balance(BSX, &ALICE);
        let bsx_tkn1_yield_farm_account = LiquidityMining::farm_account_id(GC_BSX_TKN1_YIELD_FARM_ID).unwrap();
        let bsx_tkn2_yield_farm_account = LiquidityMining::farm_account_id(GC_BSX_TKN2_YIELD_FARM_ID).unwrap();
        let bsx_tkn1_yield_farm_reward_balance = Tokens::free_balance(BSX, &bsx_tkn1_yield_farm_account);

        let expected_claimed_rewards = 79_906;
        let unclaimable_rewards = 70_094;

        //claim A1.1  (dep. A1 1-th time)
        assert_eq!(
            LiquidityMining::claim_rewards(
                ALICE,
                PREDEFINED_DEPOSIT_IDS[0],
                GC_BSX_TKN1_YIELD_FARM_ID,
                FAIL_ON_DOUBLE_CLAIM
            )
            .unwrap(),
            (
                global_farm_id,
                REWARD_CURRENCY,
                expected_claimed_rewards,
                unclaimable_rewards
            )
        );

        assert_eq!(
            LiquidityMining::deposit(PREDEFINED_DEPOSIT_IDS[0]).unwrap(),
            DepositData {
                shares: 50,
                amm_pool_id: BSX_TKN1_AMM,
                yield_farm_entries: vec![YieldFarmEntry {
                    global_farm_id,
                    yield_farm_id: GC_BSX_TKN1_YIELD_FARM_ID,
                    accumulated_rpvs: 0,
                    accumulated_claimed_rewards: expected_claimed_rewards,
                    entered_at: 18,
                    updated_at: 25,
                    valued_shares: 2_500
                }],
            },
        );

        //check if claimed rewards was transfered
        assert_eq!(
            Tokens::free_balance(BSX, &ALICE),
            alice_bsx_balance + expected_claimed_rewards
        );

        //check balance on yield farm account
        assert_eq!(
            Tokens::free_balance(BSX, &bsx_tkn1_yield_farm_account),
            bsx_tkn1_yield_farm_reward_balance - expected_claimed_rewards
        );

        // claim B3.1
        set_block_number(3_056);
        let bsx_tkn2_yield_farm_reward_balance = Tokens::free_balance(BSX, &bsx_tkn2_yield_farm_account);
        let alice_bsx_balance = Tokens::free_balance(BSX, &ALICE);

        let expected_claimed_rewards = 2_734;
        let unclaimable_rewards = 2_486;

        assert_eq!(
            LiquidityMining::claim_rewards(
                ALICE,
                PREDEFINED_DEPOSIT_IDS[4],
                GC_BSX_TKN2_YIELD_FARM_ID,
                FAIL_ON_DOUBLE_CLAIM
            )
            .unwrap(),
            (
                global_farm_id,
                REWARD_CURRENCY,
                expected_claimed_rewards,
                unclaimable_rewards
            )
        );

        assert_eq!(
            LiquidityMining::deposit(PREDEFINED_DEPOSIT_IDS[4]).unwrap(),
            DepositData {
                shares: 87,
                amm_pool_id: BSX_TKN2_AMM,
                yield_farm_entries: vec![YieldFarmEntry {
                    global_farm_id,
                    yield_farm_id: GC_BSX_TKN2_YIELD_FARM_ID,
                    valued_shares: 261,
                    accumulated_rpvs: 120,
                    accumulated_claimed_rewards: expected_claimed_rewards,
                    entered_at: 25,
                    updated_at: 30,
                }],
            },
        );

        assert_eq!(
            LiquidityMining::global_farm(GC_FARM).unwrap(),
            GlobalFarmData {
                updated_at: 30,
                accumulated_rpz: 14,
                total_shares_z: 703_990,
                accumulated_rewards: 1_039_045,
                paid_accumulated_rewards: 2_116_980,
                ..PREDEFINED_GLOBAL_FARMS[2].clone()
            }
        );

        assert_eq!(
            LiquidityMining::yield_farm((BSX_TKN2_AMM, global_farm_id, GC_BSX_TKN2_YIELD_FARM_ID)).unwrap(),
            YieldFarmData {
                updated_at: 30,
                accumulated_rpvs: 140,
                accumulated_rpz: 14,
                total_shares: 960,
                total_valued_shares: 47_629,
                entries_count: 4,
                ..PREDEFINED_YIELD_FARMS.with(|v| v[1].clone())
            },
        );

        //check if claimed rewards was transfered
        assert_eq!(
            Tokens::free_balance(BSX, &ALICE),
            alice_bsx_balance + expected_claimed_rewards
        );

        assert_eq!(
            Tokens::free_balance(BSX, &bsx_tkn2_yield_farm_account),
            bsx_tkn2_yield_farm_reward_balance + 952_580 - expected_claimed_rewards //952_580 liq. claim from global farm
        );

        //run for log time(longer than planned_yielding_periods) without interaction or claim.
        //planned_yielding_periods = 500; 100 blocks per period
        //claim A1.2
        set_block_number(125_879);
        let bst_tkn1_yield_farm_reward_balance = Tokens::free_balance(BSX, &bsx_tkn1_yield_farm_account);
        let alice_bsx_balance = Tokens::free_balance(BSX, &ALICE);

        let expected_claimed_rewards = 7_477_183;
        let unclaimable_rewards = 292_911;

        assert_eq!(
            LiquidityMining::claim_rewards(
                ALICE,
                PREDEFINED_DEPOSIT_IDS[0],
                GC_BSX_TKN1_YIELD_FARM_ID,
                FAIL_ON_DOUBLE_CLAIM
            )
            .unwrap(),
            (
                global_farm_id,
                REWARD_CURRENCY,
                expected_claimed_rewards,
                unclaimable_rewards
            )
        );

        assert_eq!(
            LiquidityMining::deposit(PREDEFINED_DEPOSIT_IDS[0]).unwrap(),
            DepositData {
                shares: 50,
                amm_pool_id: BSX_TKN1_AMM,
                yield_farm_entries: vec![YieldFarmEntry {
                    global_farm_id,
                    yield_farm_id: GC_BSX_TKN1_YIELD_FARM_ID,
                    valued_shares: 2_500,
                    accumulated_rpvs: 0,
                    accumulated_claimed_rewards: 7_557_089,
                    entered_at: 18,
                    updated_at: 1_258,
                }],
            },
        );

        assert_eq!(
            LiquidityMining::global_farm(GC_FARM).unwrap(),
            GlobalFarmData {
                updated_at: 1_258,
                max_reward_per_period: 60_000_000,
                accumulated_rpz: 628,
                total_shares_z: 703_990,
                accumulated_rewards: 293_025_705,
                paid_accumulated_rewards: 142_380_180,
                ..PREDEFINED_GLOBAL_FARMS[2].clone()
            }
        );

        assert_eq!(
            LiquidityMining::yield_farm((BSX_TKN1_AMM, global_farm_id, GC_BSX_TKN1_YIELD_FARM_ID)).unwrap(),
            YieldFarmData {
                updated_at: 1_258,
                accumulated_rpvs: 3_140,
                accumulated_rpz: 628,
                total_shares: 616,
                total_valued_shares: 45_540,
                entries_count: 3,
                ..PREDEFINED_YIELD_FARMS.with(|v| v[0].clone())
            },
        );

        assert_eq!(
            LiquidityMining::yield_farm((BSX_TKN2_AMM, global_farm_id, GC_BSX_TKN2_YIELD_FARM_ID)).unwrap(),
            YieldFarmData {
                updated_at: 30,
                accumulated_rpvs: 140,
                accumulated_rpz: 14,
                total_shares: 960,
                total_valued_shares: 47_629,
                entries_count: 4,
                ..PREDEFINED_YIELD_FARMS.with(|v| v[1].clone())
            },
        );

        //check if claimed rewards was transfered
        assert_eq!(
            Tokens::free_balance(BSX, &ALICE),
            alice_bsx_balance + expected_claimed_rewards
        );

        assert_eq!(
            Tokens::free_balance(BSX, &bsx_tkn1_yield_farm_account),
            bst_tkn1_yield_farm_reward_balance + 140_263_200 - expected_claimed_rewards //140_263_200 liq. claim from global farm
        );
    });

    //charlie's farm inncetivize KSM and reward currency is ACA
    //This test check if correct currency is tranfered if rewards and incetvized
    //assts are different, otherwise farm behaviour is the same as in tests above.
    predefined_test_ext().execute_with(|| {
        const FAIL_ON_DOUBLE_CLAIM: bool = true;
        let aca_ksm_assets = AssetPair {
            asset_in: ACA,
            asset_out: KSM,
        };

        let aca_ksm_amm_account = AMM_POOLS.with(|v| v.borrow().get(&asset_pair_to_map_key(aca_ksm_assets)).unwrap().0);

        let ksm_balance_in_amm = 50;
        //this is done because amount of incetivized token in AMM is used in calculations.
        Tokens::set_balance(Origin::root(), aca_ksm_amm_account, KSM, ksm_balance_in_amm, 0).unwrap();
        Tokens::set_balance(Origin::root(), aca_ksm_amm_account, ACA, 20, 0).unwrap();

        set_block_number(1_800); //period 18

        let global_farm_id = CHARLIE_FARM;
        let expected_claimed_rewards = 159_813; //ACA
        let unclaimable_rewards = 140_187;
        let deposited_amount = 50;
        let deposit_id = 1;
        assert_ok!(LiquidityMining::deposit_lp_shares(
            ALICE,
            CHARLIE_FARM,
            CHARLIE_ACA_KSM_YIELD_FARM_ID,
            ACA_KSM_AMM,
            deposited_amount,
        ));

        assert_eq!(
            LiquidityMining::deposit(deposit_id).unwrap(),
            DepositData {
                shares: 50,
                amm_pool_id: ACA_KSM_AMM,
                yield_farm_entries: vec![YieldFarmEntry {
                    global_farm_id,
                    yield_farm_id: CHARLIE_ACA_KSM_YIELD_FARM_ID,
                    accumulated_rpvs: 0,
                    accumulated_claimed_rewards: 0,
                    entered_at: 18,
                    updated_at: 18,
                    valued_shares: 2_500
                }],
            },
        );

        set_block_number(2_596); //period 25

        assert_eq!(
            LiquidityMining::claim_rewards(ALICE, deposit_id, CHARLIE_ACA_KSM_YIELD_FARM_ID, FAIL_ON_DOUBLE_CLAIM)
                .unwrap(),
            (CHARLIE_FARM, ACA, expected_claimed_rewards, unclaimable_rewards)
        );

        //alice had 0 ACA before claim
        assert_eq!(Tokens::free_balance(ACA, &ALICE), expected_claimed_rewards);
    });
}

#[test]
fn claim_rewards_deposit_with_multiple_entries_should_work() {
    predefined_test_ext_with_deposits().execute_with(|| {
        const FAIL_ON_DOUBLE_CLAIM: bool = true;
        //predefeinde_deposit[0] - GC_FARM, BSX_TKN1_AMM
        set_block_number(50_000);
        assert_ok!(LiquidityMining::redeposit_lp_shares(
            EVE_FARM,
            EVE_BSX_TKN1_YIELD_FARM_ID,
            PREDEFINED_DEPOSIT_IDS[0]
        ));

        set_block_number(800_000);
        //dave's farm incentivize TKN1 - some balance must be set so `valued_shares` will not be `0`.
        let bsx_tkn1_amm_account = AMM_POOLS.with(|v| {
            v.borrow()
                .get(&asset_pair_to_map_key(AssetPair {
                    asset_in: BSX,
                    asset_out: TKN1,
                }))
                .unwrap()
                .0
        });
        Tokens::set_balance(Origin::root(), bsx_tkn1_amm_account, TKN1, 100, 0).unwrap();
        assert_ok!(LiquidityMining::redeposit_lp_shares(
            DAVE_FARM,
            DAVE_BSX_TKN1_YIELD_FARM_ID,
            PREDEFINED_DEPOSIT_IDS[0]
        ));

        let deposit = LiquidityMining::deposit(PREDEFINED_DEPOSIT_IDS[0]).unwrap();

        assert_eq!(
            deposit.yield_farm_entries,
            vec![
                YieldFarmEntry {
                    global_farm_id: GC_FARM,
                    valued_shares: 2_500,
                    yield_farm_id: GC_BSX_TKN1_YIELD_FARM_ID,
                    accumulated_claimed_rewards: 0,
                    accumulated_rpvs: 0,
                    entered_at: 18,
                    updated_at: 18
                },
                YieldFarmEntry {
                    global_farm_id: DAVE_FARM,
                    valued_shares: 5_000,
                    yield_farm_id: DAVE_BSX_TKN1_YIELD_FARM_ID,
                    accumulated_claimed_rewards: 0,
                    accumulated_rpvs: 0,
                    entered_at: 800,
                    updated_at: 800
                },
                YieldFarmEntry {
                    global_farm_id: EVE_FARM,
                    valued_shares: 4_000,
                    yield_farm_id: EVE_BSX_TKN1_YIELD_FARM_ID,
                    accumulated_claimed_rewards: 0,
                    accumulated_rpvs: 0,
                    entered_at: 50,
                    updated_at: 50
                },
            ]
        );

        set_block_number(1_000_000);

        assert_eq!(
            LiquidityMining::claim_rewards(
                ALICE,
                PREDEFINED_DEPOSIT_IDS[0],
                EVE_BSX_TKN1_YIELD_FARM_ID,
                FAIL_ON_DOUBLE_CLAIM
            )
            .unwrap(),
            (EVE_FARM, KSM, 7_619_047, 380_953)
        );

        assert_noop!(
            LiquidityMining::claim_rewards(
                ALICE,
                PREDEFINED_DEPOSIT_IDS[0],
                EVE_BSX_TKN1_YIELD_FARM_ID,
                FAIL_ON_DOUBLE_CLAIM
            ),
            Error::<Test>::DoubleClaimInThePeriod
        );

        assert_eq!(
            LiquidityMining::claim_rewards(
                ALICE,
                PREDEFINED_DEPOSIT_IDS[0],
                GC_BSX_TKN1_YIELD_FARM_ID,
                FAIL_ON_DOUBLE_CLAIM
            )
            .unwrap(),
            (GC_FARM, BSX, 62_177_603, 309_897)
        );

        let deposit = LiquidityMining::deposit(PREDEFINED_DEPOSIT_IDS[0]).unwrap();
        assert_eq!(
            deposit.yield_farm_entries,
            vec![
                YieldFarmEntry {
                    global_farm_id: GC_FARM,
                    valued_shares: 2_500,
                    yield_farm_id: GC_BSX_TKN1_YIELD_FARM_ID,
                    accumulated_claimed_rewards: 62_177_603,
                    accumulated_rpvs: 0,
                    entered_at: 18,
                    updated_at: 10_000
                },
                YieldFarmEntry {
                    global_farm_id: DAVE_FARM,
                    valued_shares: 5_000,
                    yield_farm_id: DAVE_BSX_TKN1_YIELD_FARM_ID,
                    accumulated_claimed_rewards: 0,
                    accumulated_rpvs: 0,
                    entered_at: 800,
                    updated_at: 800
                },
                YieldFarmEntry {
                    global_farm_id: EVE_FARM,
                    valued_shares: 4_000,
                    yield_farm_id: EVE_BSX_TKN1_YIELD_FARM_ID,
                    accumulated_claimed_rewards: 7_619_047,
                    accumulated_rpvs: 0,
                    entered_at: 50,
                    updated_at: 1_000
                },
            ]
        );

        //same period different block
        set_block_number(1_000_050);
        assert_noop!(
            LiquidityMining::claim_rewards(
                ALICE,
                PREDEFINED_DEPOSIT_IDS[0],
                EVE_BSX_TKN1_YIELD_FARM_ID,
                FAIL_ON_DOUBLE_CLAIM
            ),
            Error::<Test>::DoubleClaimInThePeriod
        );

        assert_noop!(
            LiquidityMining::claim_rewards(
                ALICE,
                PREDEFINED_DEPOSIT_IDS[0],
                GC_BSX_TKN1_YIELD_FARM_ID,
                FAIL_ON_DOUBLE_CLAIM
            ),
            Error::<Test>::DoubleClaimInThePeriod
        );

        assert_eq!(
            LiquidityMining::claim_rewards(
                ALICE,
                PREDEFINED_DEPOSIT_IDS[0],
                DAVE_BSX_TKN1_YIELD_FARM_ID,
                FAIL_ON_DOUBLE_CLAIM
            )
            .unwrap(),
            (DAVE_FARM, ACA, 8_333_333, 1_666_667)
        );

        let deposit = LiquidityMining::deposit(PREDEFINED_DEPOSIT_IDS[0]).unwrap();
        assert_eq!(
            deposit.yield_farm_entries,
            vec![
                YieldFarmEntry {
                    global_farm_id: GC_FARM,
                    valued_shares: 2_500,
                    yield_farm_id: GC_BSX_TKN1_YIELD_FARM_ID,
                    accumulated_claimed_rewards: 62_177_603,
                    accumulated_rpvs: 0,
                    entered_at: 18,
                    updated_at: 10_000
                },
                YieldFarmEntry {
                    global_farm_id: DAVE_FARM,
                    valued_shares: 5_000,
                    yield_farm_id: DAVE_BSX_TKN1_YIELD_FARM_ID,
                    accumulated_claimed_rewards: 8_333_333,
                    accumulated_rpvs: 0,
                    entered_at: 800,
                    updated_at: 1_000
                },
                YieldFarmEntry {
                    global_farm_id: EVE_FARM,
                    valued_shares: 4_000,
                    yield_farm_id: EVE_BSX_TKN1_YIELD_FARM_ID,
                    accumulated_claimed_rewards: 7_619_047,
                    accumulated_rpvs: 0,
                    entered_at: 50,
                    updated_at: 1_000
                },
            ]
        );
    });
}

#[test]
fn claim_rewards_double_claim_in_the_same_period_should_not_work() {
    predefined_test_ext_with_deposits().execute_with(|| {
        const FAIL_ON_DOUBLE_CLAIM: bool = true;
        let global_farm_id = GC_FARM;
        let alice_bsx_balance = Tokens::free_balance(BSX, &ALICE);
        let bsx_tkn1_yield_farm_account = LiquidityMining::farm_account_id(GC_BSX_TKN1_YIELD_FARM_ID).unwrap();
        let bsx_tkn1_yield_farm_reward_balance = Tokens::free_balance(BSX, &bsx_tkn1_yield_farm_account);

        //1-th claim should work ok
        assert_ok!(LiquidityMining::claim_rewards(
            ALICE,
            PREDEFINED_DEPOSIT_IDS[0],
            GC_BSX_TKN1_YIELD_FARM_ID,
            FAIL_ON_DOUBLE_CLAIM
        ));

        assert_eq!(
            LiquidityMining::deposit(PREDEFINED_DEPOSIT_IDS[0]).unwrap(),
            DepositData {
                shares: 50,
                amm_pool_id: BSX_TKN1_AMM,
                yield_farm_entries: vec![YieldFarmEntry {
                    global_farm_id,
                    yield_farm_id: GC_BSX_TKN1_YIELD_FARM_ID,
                    valued_shares: 2_500,
                    accumulated_rpvs: 0,
                    accumulated_claimed_rewards: 79_906,
                    entered_at: 18,
                    updated_at: 25,
                }],
            },
        );

        assert_eq!(Tokens::free_balance(BSX, &ALICE), alice_bsx_balance + 79_906);
        assert_eq!(
            Tokens::free_balance(BSX, &bsx_tkn1_yield_farm_account),
            bsx_tkn1_yield_farm_reward_balance - 79_906
        );

        //second claim should fail
        assert_noop!(
            LiquidityMining::claim_rewards(
                ALICE,
                PREDEFINED_DEPOSIT_IDS[0],
                GC_BSX_TKN1_YIELD_FARM_ID,
                FAIL_ON_DOUBLE_CLAIM
            ),
            Error::<Test>::DoubleClaimInThePeriod
        );
    });
}

#[test]
fn claim_rewards_from_canceled_yield_farm_should_work() {
    predefined_test_ext_with_deposits().execute_with(|| {
        const FAIL_ON_DOUBLE_CLAIM: bool = true;
        let global_farm_id = GC_FARM;
        let alice_bsx_balance = Tokens::free_balance(BSX, &ALICE);
        let bsx_tkn1_yield_farm_account = LiquidityMining::farm_account_id(GC_BSX_TKN1_YIELD_FARM_ID).unwrap();
        let bsx_tkn1_yield_farm_reward_balance = Tokens::free_balance(BSX, &bsx_tkn1_yield_farm_account);

        //cancel yield farming before claim test
        assert_ok!(LiquidityMining::stop_yield_farm(GC, GC_FARM, BSX_TKN1_AMM));

        let expected_claimed_rewards = 79_906;
        let unclaimable_rewards = 70_094;

        //claim A1.1  (dep. A1 1-th time)
        assert_eq!(
            LiquidityMining::claim_rewards(
                ALICE,
                PREDEFINED_DEPOSIT_IDS[0],
                GC_BSX_TKN1_YIELD_FARM_ID,
                FAIL_ON_DOUBLE_CLAIM
            )
            .unwrap(),
            (global_farm_id, BSX, expected_claimed_rewards, unclaimable_rewards)
        );

        assert_eq!(
            LiquidityMining::deposit(PREDEFINED_DEPOSIT_IDS[0]).unwrap(),
            DepositData {
                shares: 50,
                amm_pool_id: BSX_TKN1_AMM,
                yield_farm_entries: vec![YieldFarmEntry {
                    global_farm_id,
                    yield_farm_id: GC_BSX_TKN1_YIELD_FARM_ID,
                    valued_shares: 2_500,
                    accumulated_rpvs: 0,
                    accumulated_claimed_rewards: expected_claimed_rewards,
                    entered_at: 18,
                    updated_at: 25,
                }],
            },
        );

        //check if claimed rewards was transfered
        assert_eq!(
            Tokens::free_balance(BSX, &ALICE),
            alice_bsx_balance + expected_claimed_rewards
        );

        //check balance on yield farm account
        assert_eq!(
            Tokens::free_balance(BSX, &bsx_tkn1_yield_farm_account),
            bsx_tkn1_yield_farm_reward_balance - expected_claimed_rewards
        );
    });
}

#[test]
fn claim_rewards_from_removed_yield_farm_should_not_work() {
    const FAIL_ON_DOUBLE_CLAIM: bool = true;
    predefined_test_ext_with_deposits().execute_with(|| {
        //cancel yield farming before removing
        assert_ok!(LiquidityMining::stop_yield_farm(GC, GC_FARM, BSX_TKN1_AMM,));

        //remove yield farm before claim test
        assert_ok!(LiquidityMining::destroy_yield_farm(
            GC,
            GC_FARM,
            GC_BSX_TKN1_YIELD_FARM_ID,
            BSX_TKN1_AMM
        ));

        assert_noop!(
            LiquidityMining::claim_rewards(
                ALICE,
                PREDEFINED_DEPOSIT_IDS[0],
                GC_BSX_TKN1_YIELD_FARM_ID,
                FAIL_ON_DOUBLE_CLAIM
            ),
            Error::<Test>::YieldFarmNotFound
        );
    });
}

#[test]
fn claim_rewards_double_claim_should_work() {
    const DONT_FAIL_ON_DOUBLE_CLAIM: bool = false;

    predefined_test_ext_with_deposits().execute_with(|| {
        let (_, _, claimable_rewards, unclaimable_rewards) = LiquidityMining::claim_rewards(
            ALICE,
            PREDEFINED_DEPOSIT_IDS[0],
            GC_BSX_TKN1_YIELD_FARM_ID,
            DONT_FAIL_ON_DOUBLE_CLAIM,
        )
        .unwrap();

        assert_eq!(claimable_rewards, 79_906);
        assert_eq!(unclaimable_rewards, 70_094);

        //second claim in the same period should renurn 0 for `claimable_rewards` and real value for
        //`unclaimable_rewards`
        let (_, _, claimable_rewards, unclaimable_rewards) = LiquidityMining::claim_rewards(
            ALICE,
            PREDEFINED_DEPOSIT_IDS[0],
            GC_BSX_TKN1_YIELD_FARM_ID,
            DONT_FAIL_ON_DOUBLE_CLAIM,
        )
        .unwrap();

        assert_eq!(claimable_rewards, 0);
        assert_eq!(unclaimable_rewards, 70_094);

        //check if double claim fails
        const FAIL_ON_DOUBLE_CLAIM: bool = true;
        assert_noop!(
            LiquidityMining::claim_rewards(
                ALICE,
                PREDEFINED_DEPOSIT_IDS[0],
                GC_BSX_TKN1_YIELD_FARM_ID,
                FAIL_ON_DOUBLE_CLAIM,
            ),
            Error::<Test>::DoubleClaimInThePeriod
        );
    });
}
