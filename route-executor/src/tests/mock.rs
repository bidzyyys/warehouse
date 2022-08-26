// This file is part of HydraDX.

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
#[warn(non_upper_case_globals)]
use crate as router;
use crate::Config;
use frame_support::parameter_types;
use frame_support::traits::{Everything, GenesisBuild, Nothing};
use frame_system as system;
use hydradx_traits::router::{Executor, ExecutorError, PoolType, TradeCalculation};
use orml_traits::parameter_type_with_key;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup, One},
};
use std::borrow::Borrow;
use std::ops::Deref;
use std::{cell::RefCell};
use crate::types::Trade;
use pretty_assertions::assert_eq;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub type AssetId = u32;
pub type Balance = u128;

frame_support::construct_runtime!(
    pub enum Test where
     Block = Block,
     NodeBlock = Block,
     UncheckedExtrinsic = UncheckedExtrinsic,
     {
         System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
         Router: router::{Pallet, Call,Event<T>},
         Tokens: orml_tokens::{Pallet, Event<T>},
		 Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
     }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 63;
}

impl system::Config for Test {
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type DbWeight = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

pub type Amount = i128;

parameter_type_with_key! {
    pub ExistentialDeposits: |_currency_id: AssetId| -> Balance {
        One::one()
    };
}

impl orml_tokens::Config for Test {
    type Event = Event;
    type Balance = Balance;
    type Amount = Amount;
    type CurrencyId = AssetId;
    type WeightInfo = ();
    type ExistentialDeposits = ExistentialDeposits;
    type OnDust = ();
    type MaxLocks = ();
    type DustRemovalWhitelist = Nothing;
    type OnNewTokenAccount = ();
    type OnKilledTokenAccount = ();
}

parameter_types! {
	pub const ExistentialDeposit: u128 = 500;
	pub const MaxReserves: u32 = 50;
}


impl pallet_balances::Config for Test {
    type MaxLocks = ();
    type Balance = Balance;
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = frame_system::Pallet<Test>;
    type WeightInfo = ();
    type MaxReserves = ();
    type ReserveIdentifier = ();
}

type Pools = (XYK, StableSwap, OmniPool);

parameter_types! {
	pub NativeCurrencyId: AssetId = 1000;
}

impl Config for Test {
    type Event = Event;
    type AssetId = AssetId;
    type Balance = Balance;
    type Currency = Tokens;
    type AMM = Pools;
}

pub type AccountId = u64;

pub const ALICE: AccountId = 1;

pub const BSX: AssetId = 1000;
pub const AUSD: AssetId = 1001;
pub const MOVR: AssetId = 1002;
pub const KSM: AssetId = 1003;

pub const ALICE_INITIAL_NATIVE_BALANCE: u128 = 1000;



pub const XYK_SELL_CALCULATION_RESULT: TradeCalculation<Balance> = TradeCalculation {
    amount: 6,
    fee: 0
};

pub const XYK_BUY_CALCULATION_RESULT: TradeCalculation<Balance> = TradeCalculation {
    amount: 5,
    fee: 0
};
pub const STABLESWAP_SELL_CALCULATION_RESULT: TradeCalculation<Balance> = TradeCalculation {
    amount: 4,
    fee: 0
};
pub const STABLESWAP_BUY_CALCULATION_RESULT: TradeCalculation<Balance> = TradeCalculation {
    amount: 3,
    fee: 0
};
pub const OMNIPOOL_SELL_CALCULATION_RESULT: TradeCalculation<Balance> = TradeCalculation {
    amount: 2,
    fee: 0
};
pub const OMNIPOOL_BUY_CALCULATION_RESULT: TradeCalculation<Balance>= TradeCalculation {
    amount: 1,
    fee: 0
};
pub const INVALID_CALCULATION_AMOUNT: TradeCalculation<Balance> = TradeCalculation {
    amount: 999,
    fee: 0
};

pub const BSX_AUSD_TRADE_IN_XYK : Trade<AssetId> = Trade {
    pool: PoolType::XYK,
    asset_in: BSX,
    asset_out: AUSD,
};

pub struct ExtBuilder {
    endowed_accounts: Vec<(AccountId, AssetId, Balance)>,
}

// Returns default values for genesis config
impl Default for ExtBuilder {
    fn default() -> Self {
        Self {
            endowed_accounts: vec![(ALICE, BSX, 1000u128)],
        }
    }
}

impl ExtBuilder {
    pub fn with_endowed_accounts(mut self, accounts: Vec<(AccountId, AssetId, Balance)>) -> Self {
        self.endowed_accounts = accounts;
        self
    }

    pub fn build(self) -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

        pallet_balances::GenesisConfig::<Test> {
                balances: vec![
                    (AccountId::from(ALICE), ALICE_INITIAL_NATIVE_BALANCE), //TODO: Dani - use const
                ],
            }
            .assimilate_storage(&mut t)
            .unwrap();

        orml_tokens::GenesisConfig::<Test> {
            balances: self.endowed_accounts,
            }
            .assimilate_storage(&mut t)
            .unwrap();

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}

thread_local! {
    pub static EXECUTED_SELLS: RefCell<Vec<(PoolType<AssetId>, TradeCalculation<Balance>, AssetId, AssetId)>> = RefCell::new(Vec::default());
    pub static EXECUTED_BUYS: RefCell<Vec<(PoolType<AssetId>, TradeCalculation<Balance>, AssetId, AssetId)>> = RefCell::new(Vec::default());
}

macro_rules! impl_fake_executor {
    ($pool_struct:ident, $pool_type: pat, $sell_calculation_result: expr, $buy_calculation_result: expr)=>{
            impl Executor<AccountId, AssetId, Balance> for $pool_struct {
                type Output = TradeCalculation<Balance>;
                type Error = ();

                fn calculate_sell(
                    pool_type: PoolType<AssetId>,
                    _asset_in: AssetId,
                    _asset_out: AssetId,
                    amount_in: TradeCalculation<Balance>,
                ) -> Result<Self::Output, ExecutorError<Self::Error>> {
                    if !matches!(pool_type, $pool_type) {
                        return Err(ExecutorError::NotSupported);
                    }

                    if amount_in == INVALID_CALCULATION_AMOUNT {
                        return Err(ExecutorError::Error(()));
                    }

                    Ok($sell_calculation_result)
                }

                fn calculate_buy(
                    pool_type: PoolType<AssetId>,
                    _asset_in: AssetId,
                    _asset_out: AssetId,
                    amount_out: TradeCalculation<Balance>,
                ) -> Result<Self::Output, ExecutorError<Self::Error>> {
                    if !matches!(pool_type, $pool_type) {
                        return Err(ExecutorError::NotSupported);
                    }

                    if amount_out == INVALID_CALCULATION_AMOUNT {
                        return Err(ExecutorError::Error(()));
                    }

                    Ok($buy_calculation_result)
                }

                fn execute_sell(
                    pool_type: PoolType<AssetId>,
                    _who: &AccountId,
                    asset_in: AssetId,
                    asset_out: AssetId,
                    amount_in: TradeCalculation<Balance>,
                ) -> Result<(), ExecutorError<Self::Error>> {
                    EXECUTED_SELLS.with(|v| {
                        let mut m = v.borrow_mut();
                        m.push((pool_type, amount_in, asset_in, asset_out));
                    });

                    Ok(())
                }

                fn execute_buy(
                    pool_type: PoolType<AssetId>,
                    _who: &AccountId,
                    asset_in: AssetId,
                    asset_out: AssetId,
                    amount_out: TradeCalculation<Balance>,
                ) -> Result<(), ExecutorError<Self::Error>> {
                    EXECUTED_BUYS.with(|v| {
                        let mut m = v.borrow_mut();
                        m.push((pool_type, amount_out, asset_in, asset_out));
                    });

                    Ok(())
                }
            }
    }
}

pub struct XYK;
pub struct StableSwap;
pub struct OmniPool;

impl_fake_executor!(XYK, PoolType::XYK, XYK_SELL_CALCULATION_RESULT, XYK_BUY_CALCULATION_RESULT);
impl_fake_executor!(StableSwap, PoolType::Stableswap(_), STABLESWAP_SELL_CALCULATION_RESULT, STABLESWAP_BUY_CALCULATION_RESULT);
impl_fake_executor!(OmniPool, PoolType::Omnipool, OMNIPOOL_SELL_CALCULATION_RESULT, OMNIPOOL_BUY_CALCULATION_RESULT);

pub fn assert_executed_sell_trades(expected_trades: Vec<(PoolType<AssetId>,TradeCalculation<Balance>, AssetId, AssetId)>) {
    EXECUTED_SELLS.borrow().with(|v| {
        let trades = v.borrow().deref().clone();
        assert_eq!(trades, expected_trades);
    });
}

pub fn assert_executed_buy_trades(expected_trades: Vec<(PoolType<AssetId>, TradeCalculation<Balance>, AssetId, AssetId)>) {
    EXECUTED_BUYS.borrow().with(|v| {
        let trades = v.borrow().deref().clone();
        assert_eq!(trades, expected_trades);
    });
}

pub fn expect_events(e: Vec<Event>) {
    let last_events = last_events(e.len());
    assert_eq!(last_events, e);
}
fn last_events(n: usize) -> Vec<Event> {
    frame_system::Pallet::<Test>::events()
        .into_iter()
        .rev()
        .take(n)
        .rev()
        .map(|e| e.event)
        .collect()
}

