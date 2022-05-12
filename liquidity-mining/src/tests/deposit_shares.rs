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
fn deposit_shares_should_work() {
	//NOTE: farm incentivize BSX token
	predefined_test_ext().execute_with(|| {
		let farm_id = GC_FARM;
		let bsx_tkn1_assets = AssetPair {
			asset_in: BSX,
			asset_out: TKN1,
		};

		let bsx_tkn2_assets = AssetPair {
			asset_in: BSX,
			asset_out: TKN2,
		};

		let pallet_account = LiquidityMining::account_id();
		let global_pool_account = LiquidityMining::pool_account_id(GC_FARM).unwrap();
		let bsx_tkn1_liq_pool_account = LiquidityMining::pool_account_id(BSX_TKN1_LIQ_POOL_ID).unwrap();
		let bsx_tkn2_liq_pool_account = LiquidityMining::pool_account_id(BSX_TKN2_LIQ_POOL_ID).unwrap();
		let bsx_tkn1_amm_account =
			AMM_POOLS.with(|v| v.borrow().get(&asset_pair_to_map_key(bsx_tkn1_assets)).unwrap().0);
		let bsx_tkn2_amm_account =
			AMM_POOLS.with(|v| v.borrow().get(&asset_pair_to_map_key(bsx_tkn2_assets)).unwrap().0);
		//DEPOSIT 1:
		set_block_number(1_800); //18-th period

		let bsx_tkn1_alice_shares = Tokens::free_balance(BSX_TKN1_SHARE_ID, &ALICE);

		//this is done because amount of incetivized token in AMM is used in calculations.
		Tokens::set_balance(Origin::root(), bsx_tkn1_amm_account, BSX, 50, 0).unwrap();
		assert_eq!(Tokens::free_balance(BSX_TKN1_SHARE_ID, &pallet_account), 0);

		let deposited_amount = 50;
		assert_ok!(LiquidityMining::deposit_shares(
			Origin::signed(ALICE),
			farm_id,
			bsx_tkn1_assets,
			deposited_amount,
		));

		expect_events(vec![
			mock::Event::NFT(pallet_nft::Event::InstanceMinted {
				owner: ALICE,
				class_id: LIQ_MINING_NFT_CLASS,
				instance_id: PREDEFINED_NFT_IDS[0],
				metadata: Default::default(),
			}),
			mock::Event::LiquidityMining(Event::SharesDeposited {
				farm_id: GC_FARM,
				liq_pool_farm_id: BSX_TKN1_LIQ_POOL_ID,
				who: ALICE,
				lp_token: BSX_TKN1_SHARE_ID,
				amount: deposited_amount,
				nft_class_id: LIQ_MINING_NFT_CLASS,
				nft_instance_id: PREDEFINED_NFT_IDS[0],
			}),
		]);

		assert_eq!(
			LiquidityMining::global_pool(GC_FARM).unwrap(),
			GlobalPool {
				id: GC_FARM,
				updated_at: 0,
				reward_currency: BSX,
				yield_per_period: Permill::from_percent(50),
				planned_yielding_periods: 500_u64,
				blocks_per_period: 100_u64,
				owner: GC,
				incentivized_asset: BSX,
				max_reward_per_period: 60_000_000,
				accumulated_rpz: 0,
				liq_pools_count: 2,
				paid_accumulated_rewards: 0,
				total_shares_z: 12_500,
				accumulated_rewards: 0
			}
		);

		assert_eq!(
			LiquidityMining::liquidity_pool(GC_FARM, BSX_TKN1_AMM).unwrap(),
			LiquidityPoolYieldFarm {
				id: BSX_TKN1_LIQ_POOL_ID,
				updated_at: 0,
				accumulated_rpvs: 0,
				accumulated_rpz: 0,
				total_shares: 50,
				total_valued_shares: 2_500,
				loyalty_curve: Some(LoyaltyCurve::default()),
				stake_in_global_pool: 12_500,
				multiplier: FixedU128::from(5_u128),
				canceled: false,
			},
		);

		assert_eq!(
			LiquidityMining::liq_pool_meta(BSX_TKN1_LIQ_POOL_ID).unwrap(),
			(bsx_tkn1_assets, 1, GC_FARM)
		);

		assert_eq!(
			LiquidityMining::deposit(PREDEFINED_NFT_IDS[0]).unwrap(),
			Deposit {
				shares: deposited_amount,
				valued_shares: 2_500,
				accumulated_rpvs: 0,
				accumulated_claimed_rewards: 0,
				entered_at: 18,
				updated_at: 18,
			},
		);

		//check if shares was transferd from extrinsic caller
		assert_eq!(
			Tokens::free_balance(BSX_TKN1_SHARE_ID, &ALICE),
			bsx_tkn1_alice_shares - deposited_amount
		);

		//check if shares was transferd to liq. mining pallet account
		assert_eq!(
			Tokens::free_balance(BSX_TKN1_SHARE_ID, &pallet_account),
			deposited_amount
		);

		// DEPOSIT 2 (deposit in same period):
		let bsx_tkn1_bob_shares = Tokens::free_balance(BSX_TKN1_SHARE_ID, &BOB);

		//this is done because amount of incetivized token in AMM is used in calculations.
		Tokens::set_balance(Origin::root(), bsx_tkn1_amm_account, BSX, 52, 0).unwrap();

		let deposited_amount = 80;
		assert_ok!(LiquidityMining::deposit_shares(
			Origin::signed(BOB),
			farm_id,
			bsx_tkn1_assets,
			deposited_amount
		));

		expect_events(vec![
			mock::Event::NFT(pallet_nft::Event::InstanceMinted {
				owner: BOB,
				class_id: LIQ_MINING_NFT_CLASS,
				instance_id: PREDEFINED_NFT_IDS[1],
				metadata: Default::default(),
			}),
			mock::Event::LiquidityMining(Event::SharesDeposited {
				farm_id: GC_FARM,
				liq_pool_farm_id: BSX_TKN1_LIQ_POOL_ID,
				who: BOB,
				lp_token: BSX_TKN1_SHARE_ID,
				amount: deposited_amount,
				nft_class_id: LIQ_MINING_NFT_CLASS,
				nft_instance_id: PREDEFINED_NFT_IDS[1],
			}),
		]);

		assert_eq!(
			LiquidityMining::global_pool(GC_FARM).unwrap(),
			GlobalPool {
				id: GC_FARM,
				updated_at: 18,
				reward_currency: BSX,
				yield_per_period: Permill::from_percent(50),
				planned_yielding_periods: 500_u64,
				blocks_per_period: 100_u64,
				owner: GC,
				incentivized_asset: BSX,
				max_reward_per_period: 60_000_000,
				accumulated_rpz: 9,
				liq_pools_count: 2,
				paid_accumulated_rewards: 112_500,
				total_shares_z: 33_300,
				accumulated_rewards: 0,
			}
		);

		assert_eq!(
			LiquidityMining::liquidity_pool(GC_FARM, BSX_TKN1_AMM).unwrap(),
			LiquidityPoolYieldFarm {
				id: BSX_TKN1_LIQ_POOL_ID,
				updated_at: 18,
				accumulated_rpvs: 45,
				accumulated_rpz: 9,
				total_shares: 130,
				total_valued_shares: 6_660,
				loyalty_curve: Some(LoyaltyCurve::default()),
				stake_in_global_pool: 33_300,
				multiplier: FixedU128::from(5_u128),
				canceled: false,
			},
		);

		assert_eq!(
			LiquidityMining::liq_pool_meta(BSX_TKN1_LIQ_POOL_ID).unwrap(),
			(bsx_tkn1_assets, 2, GC_FARM)
		);

		assert_eq!(
			LiquidityMining::deposit(PREDEFINED_NFT_IDS[1]).unwrap(),
			Deposit {
				shares: deposited_amount,
				valued_shares: 4_160,
				accumulated_rpvs: 45,
				accumulated_claimed_rewards: 0,
				entered_at: 18,
				updated_at: 18,
			},
		);

		//check if shares was transfered from deposit owner
		assert_eq!(
			Tokens::free_balance(BSX_TKN1_SHARE_ID, &BOB),
			bsx_tkn1_bob_shares - deposited_amount
		);
		assert_eq!(Tokens::free_balance(BSX_TKN1_SHARE_ID, &pallet_account), 130); //130 - sum of all deposited shares until now

		assert_eq!(
			Tokens::free_balance(BSX, &global_pool_account),
			(30_000_000_000 - 112_500) //total_rewards - sum(claimed rewards by all liq. pools until now)
		);

		//check if claim from global pool was transfered to liq. pool account
		assert_eq!(Tokens::free_balance(BSX, &bsx_tkn1_liq_pool_account), 112_500);

		// DEPOSIT 3 (same period, second liq pool yield farm):
		let bsx_tkn2_bob_shares = Tokens::free_balance(BSX_TKN2_SHARE_ID, &BOB);

		//this is done because amount of incetivized token in AMM is used in calculations.
		Tokens::set_balance(Origin::root(), bsx_tkn2_amm_account, BSX, 8, 0).unwrap();

		let deposited_amount = 25;
		assert_ok!(LiquidityMining::deposit_shares(
			Origin::signed(BOB),
			farm_id,
			bsx_tkn2_assets,
			deposited_amount
		));

		expect_events(vec![
			mock::Event::NFT(pallet_nft::Event::InstanceMinted {
				owner: BOB,
				class_id: LIQ_MINING_NFT_CLASS,
				instance_id: PREDEFINED_NFT_IDS[2],
				metadata: Default::default(),
			}),
			mock::Event::LiquidityMining(Event::SharesDeposited {
				farm_id: GC_FARM,
				liq_pool_farm_id: BSX_TKN2_LIQ_POOL_ID,
				who: BOB,
				lp_token: BSX_TKN2_SHARE_ID,
				amount: deposited_amount,
				nft_class_id: LIQ_MINING_NFT_CLASS,
				nft_instance_id: PREDEFINED_NFT_IDS[2],
			}),
		]);

		assert_eq!(
			LiquidityMining::global_pool(GC_FARM).unwrap(),
			GlobalPool {
				id: GC_FARM,
				updated_at: 18,
				reward_currency: BSX,
				yield_per_period: Permill::from_percent(50),
				planned_yielding_periods: 500_u64,
				blocks_per_period: 100_u64,
				owner: GC,
				incentivized_asset: BSX,
				max_reward_per_period: 60_000_000,
				accumulated_rpz: 9,
				liq_pools_count: 2,
				paid_accumulated_rewards: 112_500,
				total_shares_z: 35_300,
				accumulated_rewards: 0,
			}
		);

		assert_eq!(
			LiquidityMining::liquidity_pool(GC_FARM, BSX_TKN2_AMM).unwrap(),
			LiquidityPoolYieldFarm {
				id: BSX_TKN2_LIQ_POOL_ID,
				updated_at: 0,
				accumulated_rpvs: 0,
				accumulated_rpz: 0,
				total_shares: 25,
				total_valued_shares: 200,
				loyalty_curve: Some(LoyaltyCurve::default()),
				stake_in_global_pool: 2_000,
				multiplier: FixedU128::from(10_u128),
				canceled: false,
			},
		);

		assert_eq!(
			LiquidityMining::liq_pool_meta(BSX_TKN2_LIQ_POOL_ID).unwrap(),
			(bsx_tkn2_assets, 1, GC_FARM)
		);

		assert_eq!(
			LiquidityMining::deposit(PREDEFINED_NFT_IDS[2]).unwrap(),
			Deposit {
				shares: deposited_amount,
				valued_shares: 200,
				accumulated_rpvs: 0,
				accumulated_claimed_rewards: 0,
				entered_at: 18,
				updated_at: 18,
			},
		);

		//check if shares was transfered from deposit owner
		assert_eq!(
			Tokens::free_balance(BSX_TKN2_SHARE_ID, &BOB),
			bsx_tkn2_bob_shares - deposited_amount
		);
		assert_eq!(Tokens::free_balance(BSX_TKN2_SHARE_ID, &pallet_account), 25); //25 - sum of all deposited shares until now

		//pool wasn't updated in this period so no claim from global pool
		assert_eq!(
			Tokens::free_balance(BSX, &global_pool_account),
			(30_000_000_000 - 112_500) //total_rewards - claimed rewards by liq. pool
		);

		// no claim happed for this pool so this is same as after previous deposit
		assert_eq!(Tokens::free_balance(BSX, &bsx_tkn1_liq_pool_account), 112_500);
		//check if claim from global pool was transfered to liq. pool account
		//(there was no clai for this pool)
		assert_eq!(Tokens::free_balance(BSX, &bsx_tkn2_liq_pool_account), 0);

		// DEPOSIT 4 (new period):
		set_block_number(2051); //period 20
		let bsx_tkn2_bob_shares = Tokens::free_balance(BSX_TKN2_SHARE_ID, &BOB);

		//this is done because amount of incetivized token in AMM is used in calculations.
		Tokens::set_balance(Origin::root(), bsx_tkn2_amm_account, BSX, 58, 0).unwrap();

		let deposited_amount = 800;
		assert_ok!(LiquidityMining::deposit_shares(
			Origin::signed(BOB),
			farm_id,
			bsx_tkn2_assets,
			deposited_amount
		));

		expect_events(vec![
			mock::Event::NFT(pallet_nft::Event::InstanceMinted {
				owner: BOB,
				class_id: LIQ_MINING_NFT_CLASS,
				instance_id: PREDEFINED_NFT_IDS[3],
				metadata: Default::default(),
			}),
			mock::Event::LiquidityMining(Event::SharesDeposited {
				farm_id: GC_FARM,
				liq_pool_farm_id: BSX_TKN2_LIQ_POOL_ID,
				who: BOB,
				lp_token: BSX_TKN2_SHARE_ID,
				amount: deposited_amount,
				nft_class_id: LIQ_MINING_NFT_CLASS,
				nft_instance_id: PREDEFINED_NFT_IDS[3],
			}),
		]);

		assert_eq!(
			LiquidityMining::global_pool(GC_FARM).unwrap(),
			GlobalPool {
				id: GC_FARM,
				updated_at: 20,
				reward_currency: BSX,
				yield_per_period: Permill::from_percent(50),
				planned_yielding_periods: 500_u64,
				blocks_per_period: 100_u64,
				owner: GC,
				incentivized_asset: BSX,
				max_reward_per_period: 60_000_000,
				accumulated_rpz: 10,
				liq_pools_count: 2,
				paid_accumulated_rewards: 132_500,
				total_shares_z: 499_300,
				accumulated_rewards: 15_300,
			}
		);

		assert_eq!(
			LiquidityMining::liquidity_pool(GC_FARM, BSX_TKN2_AMM).unwrap(),
			LiquidityPoolYieldFarm {
				id: BSX_TKN2_LIQ_POOL_ID,
				updated_at: 20,
				accumulated_rpvs: 100,
				accumulated_rpz: 10,
				total_shares: 825,
				total_valued_shares: 46_600,
				loyalty_curve: Some(LoyaltyCurve::default()),
				stake_in_global_pool: 466_000,
				multiplier: FixedU128::from(10_u128),
				canceled: false,
			},
		);

		assert_eq!(
			LiquidityMining::liq_pool_meta(BSX_TKN2_LIQ_POOL_ID).unwrap(),
			(bsx_tkn2_assets, 2, GC_FARM)
		);

		assert_eq!(
			LiquidityMining::deposit(PREDEFINED_NFT_IDS[3]).unwrap(),
			Deposit {
				shares: deposited_amount,
				valued_shares: 46_400,
				accumulated_rpvs: 100,
				accumulated_claimed_rewards: 0,
				entered_at: 20,
				updated_at: 20,
			},
		);

		//check if shares was transfered from deposit owner
		assert_eq!(
			Tokens::free_balance(BSX_TKN2_SHARE_ID, &BOB),
			bsx_tkn2_bob_shares - deposited_amount
		);
		assert_eq!(Tokens::free_balance(BSX_TKN2_SHARE_ID, &pallet_account), 825); //825 - sum of all deposited shares until now

		assert_eq!(
			Tokens::free_balance(BSX, &global_pool_account),
			(30_000_000_000 - 132_500) //total_rewards - sum(claimed rewards by all liq. pools until now)
		);

		//check if claim from global pool was transfered to liq. pool account
		assert_eq!(Tokens::free_balance(BSX, &bsx_tkn1_liq_pool_account), 112_500);
		//check if claim from global pool was transfered to liq. pool account
		assert_eq!(Tokens::free_balance(BSX, &bsx_tkn2_liq_pool_account), 20_000);

		// DEPOSIT 5 (same period, second liq pool yield farm):
		set_block_number(2_586); //period 20
		let bsx_tkn2_alice_shares = Tokens::free_balance(BSX_TKN2_SHARE_ID, &ALICE);

		//this is done because amount of incetivized token in AMM is used in calculations.
		Tokens::set_balance(Origin::root(), bsx_tkn2_amm_account, BSX, 3, 0).unwrap();

		let deposited_amount = 87;
		assert_ok!(LiquidityMining::deposit_shares(
			Origin::signed(ALICE),
			farm_id,
			bsx_tkn2_assets,
			deposited_amount,
		));

		expect_events(vec![
			mock::Event::NFT(pallet_nft::Event::InstanceMinted {
				owner: ALICE,
				class_id: LIQ_MINING_NFT_CLASS,
				instance_id: PREDEFINED_NFT_IDS[4],
				metadata: Default::default(),
			}),
			mock::Event::LiquidityMining(Event::SharesDeposited {
				farm_id: GC_FARM,
				liq_pool_farm_id: BSX_TKN2_LIQ_POOL_ID,
				who: ALICE,
				lp_token: BSX_TKN2_SHARE_ID,
				amount: 87,
				nft_class_id: LIQ_MINING_NFT_CLASS,
				nft_instance_id: PREDEFINED_NFT_IDS[4],
			}),
		]);

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
				total_shares_z: 501_910,
				accumulated_rewards: 331_550,
				paid_accumulated_rewards: 1_064_500,
			}
		);

		assert_eq!(
			LiquidityMining::liquidity_pool(GC_FARM, BSX_TKN2_AMM).unwrap(),
			LiquidityPoolYieldFarm {
				id: BSX_TKN2_LIQ_POOL_ID,
				updated_at: 25,
				accumulated_rpvs: 120,
				accumulated_rpz: 12,
				total_shares: 912,
				total_valued_shares: 46_861,
				loyalty_curve: Some(LoyaltyCurve::default()),
				stake_in_global_pool: 468_610,
				multiplier: FixedU128::from(10_u128),
				canceled: false,
			},
		);

		assert_eq!(
			LiquidityMining::liq_pool_meta(BSX_TKN2_LIQ_POOL_ID).unwrap(),
			(bsx_tkn2_assets, 3, GC_FARM)
		);

		assert_eq!(
			LiquidityMining::deposit(PREDEFINED_NFT_IDS[4]).unwrap(),
			Deposit {
				shares: deposited_amount,
				valued_shares: 261,
				accumulated_rpvs: 120,
				accumulated_claimed_rewards: 0,
				entered_at: 25,
				updated_at: 25,
			},
		);

		//check if shares was transfered from deposit owner
		assert_eq!(
			Tokens::free_balance(BSX_TKN2_SHARE_ID, &ALICE),
			bsx_tkn2_alice_shares - 87
		);
		assert_eq!(Tokens::free_balance(BSX_TKN2_SHARE_ID, &pallet_account), 912); //912 - sum of all deposited shares until now

		assert_eq!(
			Tokens::free_balance(BSX, &global_pool_account),
			(30_000_000_000 - 1_064_500) //total_rewards - sum(claimed rewards by all liq. pools until now)
		);

		//check if claim from global pool was transfered to liq. pool account
		assert_eq!(Tokens::free_balance(BSX, &bsx_tkn1_liq_pool_account), 112_500); //total_rewards - sum(claimed rewards by all liq. pools until now)
		assert_eq!(Tokens::free_balance(BSX, &bsx_tkn2_liq_pool_account), 952_000); //total_rewards - sum(claimed rewards by all liq. pools until now)

		// DEPOSIT 6 (same period):
		set_block_number(2_596); //period 20
		let bsx_tkn2_alice_shares = Tokens::free_balance(BSX_TKN2_SHARE_ID, &ALICE);

		//this is done because amount of incetivized token in AMM is used in calculations.
		Tokens::set_balance(Origin::root(), bsx_tkn2_amm_account, BSX, 16, 0).unwrap();

		let deposited_amount = 48;
		assert_ok!(LiquidityMining::deposit_shares(
			Origin::signed(ALICE),
			farm_id,
			bsx_tkn2_assets,
			deposited_amount,
		));

		expect_events(vec![
			mock::Event::NFT(pallet_nft::Event::InstanceMinted {
				owner: ALICE,
				class_id: LIQ_MINING_NFT_CLASS,
				instance_id: PREDEFINED_NFT_IDS[5],
				metadata: Default::default(),
			}),
			mock::Event::LiquidityMining(Event::SharesDeposited {
				farm_id: GC_FARM,
				liq_pool_farm_id: BSX_TKN2_LIQ_POOL_ID,
				who: ALICE,
				lp_token: BSX_TKN2_SHARE_ID,
				amount: deposited_amount,
				nft_class_id: LIQ_MINING_NFT_CLASS,
				nft_instance_id: PREDEFINED_NFT_IDS[5],
			}),
		]);

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
				total_shares_z: 509_590,
				accumulated_rewards: 331_550,
				paid_accumulated_rewards: 1_064_500,
			}
		);

		assert_eq!(
			LiquidityMining::liquidity_pool(GC_FARM, BSX_TKN2_AMM).unwrap(),
			LiquidityPoolYieldFarm {
				id: BSX_TKN2_LIQ_POOL_ID,
				updated_at: 25,
				accumulated_rpvs: 120,
				accumulated_rpz: 12,
				total_shares: 960,
				total_valued_shares: 47_629,
				loyalty_curve: Some(LoyaltyCurve::default()),
				stake_in_global_pool: 476_290,
				multiplier: FixedU128::from(10_u128),
				canceled: false,
			},
		);

		assert_eq!(
			LiquidityMining::liq_pool_meta(BSX_TKN2_LIQ_POOL_ID).unwrap(),
			(bsx_tkn2_assets, 4, GC_FARM)
		);

		assert_eq!(
			LiquidityMining::deposit(PREDEFINED_NFT_IDS[5]).unwrap(),
			Deposit {
				shares: deposited_amount,
				valued_shares: 768,
				accumulated_rpvs: 120,
				accumulated_claimed_rewards: 0,
				entered_at: 25,
				updated_at: 25,
			},
		);

		//check if shares was transfered from deposit owner
		assert_eq!(
			Tokens::free_balance(BSX_TKN2_SHARE_ID, &ALICE),
			bsx_tkn2_alice_shares - deposited_amount
		);
		assert_eq!(Tokens::free_balance(BSX_TKN2_SHARE_ID, &pallet_account), 960); //960 - sum of all deposited shares until now

		assert_eq!(
			Tokens::free_balance(BSX, &global_pool_account),
			(30_000_000_000 - 1_064_500) //total_rewards - sum(claimed rewards by all liq. pools until now)
		);

		assert_eq!(Tokens::free_balance(BSX, &bsx_tkn1_liq_pool_account), 112_500); //total_rewards - sum(claimed rewards by all liq. pools until now)
		assert_eq!(Tokens::free_balance(BSX, &bsx_tkn2_liq_pool_account), 952_000); //total_rewards - sum(claimed rewards by all liq. pools until now)

		// DEPOSIT 7 : (same period differen liq poll farm)
		set_block_number(2_596); //period 20
		let bsx_tkn1_alice_shares = Tokens::free_balance(BSX_TKN1_SHARE_ID, &ALICE);

		//this is done because amount of incetivized token in AMM is used in calculations.
		Tokens::set_balance(Origin::root(), bsx_tkn1_amm_account, BSX, 80, 0).unwrap();

		let deposited_amount = 486;
		assert_ok!(LiquidityMining::deposit_shares(
			Origin::signed(ALICE),
			farm_id,
			bsx_tkn1_assets,
			486
		));

		expect_events(vec![
			mock::Event::NFT(pallet_nft::Event::InstanceMinted {
				owner: ALICE,
				class_id: LIQ_MINING_NFT_CLASS,
				instance_id: PREDEFINED_NFT_IDS[6],
				metadata: Default::default(),
			}),
			mock::Event::LiquidityMining(Event::SharesDeposited {
				farm_id: GC_FARM,
				liq_pool_farm_id: BSX_TKN1_LIQ_POOL_ID,
				who: ALICE,
				lp_token: BSX_TKN1_SHARE_ID,
				amount: deposited_amount,
				nft_class_id: LIQ_MINING_NFT_CLASS,
				nft_instance_id: PREDEFINED_NFT_IDS[6],
			}),
		]);

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
				id: BSX_TKN1_LIQ_POOL_ID,
				updated_at: 25,
				accumulated_rpvs: 60,
				accumulated_rpz: 12,
				total_shares: 616,
				total_valued_shares: 45_540,
				loyalty_curve: Some(LoyaltyCurve::default()),
				stake_in_global_pool: 227_700,
				multiplier: FixedU128::from(5_u128),
				canceled: false,
			},
		);

		assert_eq!(
			LiquidityMining::liq_pool_meta(BSX_TKN1_LIQ_POOL_ID).unwrap(),
			(bsx_tkn1_assets, 3, GC_FARM)
		);

		assert_eq!(
			LiquidityMining::deposit(PREDEFINED_NFT_IDS[6]).unwrap(),
			Deposit {
				shares: deposited_amount,
				valued_shares: 38_880,
				accumulated_rpvs: 60,
				accumulated_claimed_rewards: 0,
				entered_at: 25,
				updated_at: 25,
			},
		);

		//check if shares was transfered from deposit owner
		assert_eq!(
			Tokens::free_balance(BSX_TKN1_SHARE_ID, &ALICE),
			bsx_tkn1_alice_shares - deposited_amount
		);
		assert_eq!(Tokens::free_balance(BSX_TKN1_SHARE_ID, &pallet_account), 616); //616 - sum of all deposited shares until now

		assert_eq!(
			Tokens::free_balance(BSX, &global_pool_account),
			(30_000_000_000 - 1_164_400) //total_rewards - sum(claimed rewards by all liq. pools until now)
		);

		//check if claim from global pool was transfered to liq. pool account
		assert_eq!(Tokens::free_balance(BSX, &bsx_tkn1_liq_pool_account), 212_400); //total_rewards - sum(claimed rewards by all liq. pools until now)
		assert_eq!(Tokens::free_balance(BSX, &bsx_tkn2_liq_pool_account), 952_000); //total_rewards - sum(claimed rewards by all liq. pools until now)
	});

	//deposit to farm with different incentivized_asset and reward_currency
	//charlie's farm inncetivize KSM and reward currency is ACA
	//This test only check if valued shares are correctly calculated if reward and incentivized
	//assts are different, otherwise pool behaviour is same as in test above.
	predefined_test_ext().execute_with(|| {
		let aca_ksm_assets = AssetPair {
			asset_in: ACA,
			asset_out: KSM,
		};

		let aca_ksm_amm_account = AMM_POOLS.with(|v| v.borrow().get(&asset_pair_to_map_key(aca_ksm_assets)).unwrap().0);
		let ksm_balance_in_amm = 16;

		//this is done because amount of incetivized token in AMM is used in calculations.
		Tokens::set_balance(Origin::root(), aca_ksm_amm_account, KSM, ksm_balance_in_amm, 0).unwrap();
		Tokens::set_balance(Origin::root(), aca_ksm_amm_account, ACA, 20, 0).unwrap();

		set_block_number(2_596); //period 25

		let deposited_amount = 1_000_000;
		assert_ok!(LiquidityMining::deposit_shares(
			Origin::signed(ALICE),
			CHARLIE_FARM,
			aca_ksm_assets,
			deposited_amount
		));

		assert_eq!(
			LiquidityMining::deposit(4294967303).unwrap(),
			Deposit {
				shares: deposited_amount,
				valued_shares: deposited_amount * ksm_balance_in_amm,
				accumulated_rpvs: 0,
				accumulated_claimed_rewards: 0,
				entered_at: 25,
				updated_at: 25,
			}
		);
	});
}

#[test]
fn deposit_shares_zero_deposit_should_not_work() {
	predefined_test_ext_with_deposits().execute_with(|| {
		let bsx_tkn1_assets = AssetPair {
			asset_in: BSX,
			asset_out: TKN1,
		};

		assert_noop!(
			LiquidityMining::deposit_shares(Origin::signed(ALICE), GC_FARM, bsx_tkn1_assets, 0),
			Error::<Test>::InvalidDepositAmount
		);
	});
}

#[test]
fn deposit_shares_insufficient_shares_balance_should_not_work() {
	predefined_test_ext_with_deposits().execute_with(|| {
		let bsx_tkn1_assets = AssetPair {
			asset_in: BSX,
			asset_out: TKN1,
		};

		assert_noop!(
			LiquidityMining::deposit_shares(Origin::signed(ALICE), GC_FARM, bsx_tkn1_assets, 4_000_000),
			Error::<Test>::InsufficientAmmSharesBalance
		);
	});
}

#[test]
fn deposit_shares_non_existing_liq_pool_should_not_work() {
	predefined_test_ext_with_deposits().execute_with(|| {
		let bsx_dot_assets = AssetPair {
			asset_in: BSX,
			asset_out: DOT,
		};

		assert_noop!(
			LiquidityMining::deposit_shares(Origin::signed(ALICE), GC_FARM, bsx_dot_assets, 10_000),
			Error::<Test>::LiquidityPoolNotFound
		);
	});
}

#[test]
fn deposit_shares_canceled_liq_pool_should_not_work() {
	predefined_test_ext_with_deposits().execute_with(|| {
		let bsx_tkn1_assets = AssetPair {
			asset_in: BSX,
			asset_out: TKN1,
		};

		assert_ok!(LiquidityMining::cancel_liquidity_pool(
			Origin::signed(GC),
			GC_FARM,
			bsx_tkn1_assets
		));

		assert_noop!(
			LiquidityMining::deposit_shares(Origin::signed(ALICE), GC_FARM, bsx_tkn1_assets, 10_000),
			Error::<Test>::LiquidityMiningCanceled
		);
	});
}
