use super::*;
use codec::{Decode, Encode};
use frame_support::weights::IdentityFee;
use smallvec::smallvec;
use sp_runtime::{traits::One, DispatchError, DispatchResult, Perbill};
use sp_std::collections::btree_set::BTreeSet;
use std::sync::Mutex;

type AccountId = u32;
type AssetId = u32;
type Balance = u128;

const CORE_ASSET_ID: AssetId = 0;
const TEST_ASSET_ID: AssetId = 123;
const CHEAP_ASSET_ID: AssetId = 420;
const OVERFLOW_ASSET_ID: AssetId = 1_000;

struct MockOracle;
impl PriceOracle<AssetId, Price> for MockOracle {
    fn price(currency: AssetId) -> Option<Price> {
        match currency {
            CORE_ASSET_ID => Some(Price::one()),
            TEST_ASSET_ID => Some(Price::from_float(0.5)),
            CHEAP_ASSET_ID => Some(Price::saturating_from_integer(4)),
            OVERFLOW_ASSET_ID => Some(Price::saturating_from_integer(2_147_483_647)),
            _ => None,
        }
    }
}

struct MockConvert;
impl Convert<AssetId, Option<MultiLocation>> for MockConvert {
    fn convert(id: AssetId) -> Option<MultiLocation> {
        match id {
            CORE_ASSET_ID | TEST_ASSET_ID | CHEAP_ASSET_ID | OVERFLOW_ASSET_ID => {
                Some(MultiLocation::new(0, X1(GeneralKey(id.encode()))))
            }
            _ => None,
        }
    }
}

impl Convert<MultiLocation, Option<AssetId>> for MockConvert {
    fn convert(location: MultiLocation) -> Option<AssetId> {
        match location {
            MultiLocation {
                parents: 0,
                interior: X1(GeneralKey(key)),
            } => {
                if let Ok(currency_id) = AssetId::decode(&mut &key[..]) {
                    // we currently have only one native asset
                    match currency_id {
                        CORE_ASSET_ID | TEST_ASSET_ID | CHEAP_ASSET_ID | OVERFLOW_ASSET_ID => Some(currency_id),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl Convert<MultiAsset, Option<AssetId>> for MockConvert {
    fn convert(asset: MultiAsset) -> Option<AssetId> {
        if let MultiAsset {
            id: Concrete(location), ..
        } = asset
        {
            Self::convert(location)
        } else {
            None
        }
    }
}

macro_rules! generate_revenue_type {
    ($name:ident) => {
        lazy_static::lazy_static! {
            pub static ref TAKEN: Mutex<BTreeSet<MultiAsset>> = Mutex::new(BTreeSet::new());
            pub static ref EXPECTED: Mutex<BTreeSet<MultiAsset>> = Mutex::new(BTreeSet::new());
        };

        struct $name;
        impl $name {
            #[allow(unused)]
            fn register_expected_asset(asset: MultiAsset) {
                EXPECTED.lock().unwrap().insert(asset);
            }

            #[allow(unused)]
            fn expect_revenue() {
                for asset in EXPECTED.lock().unwrap().iter() {
                    assert!(TAKEN.lock().unwrap().contains(dbg!(asset)));
                }
            }

            #[allow(unused)]
            fn expect_no_revenue() {
                assert!(TAKEN.lock().unwrap().is_empty(), "There should be no revenue taken.");
            }

            /// Reset the global mutable state.
            #[allow(unused)]
            fn reset() {
                *TAKEN.lock().unwrap() = BTreeSet::new();
                *EXPECTED.lock().unwrap() = BTreeSet::new();
            }
        }

        impl TakeRevenue for $name {
            fn take_revenue(asset: MultiAsset) {
                TAKEN.lock().unwrap().insert(asset);
            }
        }
    };
}

#[test]
fn can_buy_weight() {
    generate_revenue_type!(ExpectRevenue);

    type Trader = MultiCurrencyTrader<AssetId, Balance, IdentityFee<Balance>, MockOracle, MockConvert, ExpectRevenue>;

    let core_id = MockConvert::convert(CORE_ASSET_ID).unwrap();
    let test_id = MockConvert::convert(TEST_ASSET_ID).unwrap();
    let cheap_id = MockConvert::convert(CHEAP_ASSET_ID).unwrap();

    {
        let mut trader = Trader::new();

        let core_payment: MultiAsset = (Concrete(core_id.clone()), 1_000_000).into();
        let res = dbg!(trader.buy_weight(1_000_000, core_payment.clone().into()));
        assert!(res
            .expect("buy_weight should succeed because payment == weight")
            .is_empty());
        ExpectRevenue::register_expected_asset(core_payment);

        let test_payment: MultiAsset = (Concrete(test_id), 500_000).into();
        let res = dbg!(trader.buy_weight(1_000_000, test_payment.clone().into()));
        assert!(res
            .expect("buy_weight should succeed because payment == 0.5 * weight")
            .is_empty());
        ExpectRevenue::register_expected_asset(test_payment);

        let cheap_payment: MultiAsset = (Concrete(cheap_id), 4_000_000).into();
        let res = dbg!(trader.buy_weight(1_000_000, cheap_payment.clone().into()));
        assert!(res
            .expect("buy_weight should succeed because payment == 4 * weight")
            .is_empty());
        ExpectRevenue::register_expected_asset(cheap_payment);
    }
    ExpectRevenue::expect_revenue();
}

#[test]
fn cannot_buy_with_too_few_tokens() {
    type Trader = MultiCurrencyTrader<AssetId, Balance, IdentityFee<Balance>, MockOracle, MockConvert, ()>;

    let core_id = MockConvert::convert(CORE_ASSET_ID).unwrap();

    let mut trader = Trader::new();

    let payment: MultiAsset = (Concrete(core_id), 69).into();
    let res = dbg!(trader.buy_weight(1_000_000, payment.into()));
    assert_eq!(res, Err(XcmError::TooExpensive));
}

#[test]
fn cannot_buy_with_unknown_token() {
    type Trader = MultiCurrencyTrader<AssetId, Balance, IdentityFee<Balance>, MockOracle, MockConvert, ()>;

    let unknown_token = GeneralKey(9876u32.encode());

    let mut trader = Trader::new();
    let payment: MultiAsset = (Concrete(unknown_token.into()), 1_000_000).into();
    let res = dbg!(trader.buy_weight(1_000_000, payment.into()));
    assert_eq!(res, Err(XcmError::AssetNotFound));
}

#[test]
fn overflow_errors() {
    use frame_support::weights::{WeightToFeeCoefficient, WeightToFeeCoefficients};
    // Create a mock fee calculator that always returns `max_value`.
    pub struct MaxFee;
    impl WeightToFeePolynomial for MaxFee {
        type Balance = Balance;

        fn polynomial() -> WeightToFeeCoefficients<Balance> {
            smallvec!(WeightToFeeCoefficient {
                coeff_integer: Balance::max_value(),
                coeff_frac: Perbill::zero(),
                negative: false,
                degree: 1,
            })
        }
    }
    type Trader = MultiCurrencyTrader<AssetId, Balance, MaxFee, MockOracle, MockConvert, ()>;

    let overflow_id = MockConvert::convert(OVERFLOW_ASSET_ID).unwrap();

    let mut trader = Trader::new();

    let amount = 1_000;
    let payment: MultiAsset = (Concrete(overflow_id), amount).into();
    let weight = 1_000;
    let res = dbg!(trader.buy_weight(weight, payment.into()));
    assert_eq!(res, Err(XcmError::Overflow));
}

#[test]
fn refunds_first_asset_completely() {
    generate_revenue_type!(ExpectRevenue);

    type Trader = MultiCurrencyTrader<AssetId, Balance, IdentityFee<Balance>, MockOracle, MockConvert, ExpectRevenue>;

    let core_id = MockConvert::convert(CORE_ASSET_ID).unwrap();

    {
        let mut trader = Trader::new();

        let weight = 1_000_000;
        let tokens = 1_000_000;
        let core_payment: MultiAsset = (Concrete(core_id), tokens).into();
        let res = dbg!(trader.buy_weight(weight, core_payment.clone().into()));
        assert!(res
            .expect("buy_weight should succeed because payment == weight")
            .is_empty());
        assert_eq!(trader.refund_weight(weight), Some(core_payment.into()));
    }
    ExpectRevenue::expect_no_revenue();
}

#[test]
fn needs_multiple_refunds_for_multiple_currencies() {
    generate_revenue_type!(ExpectRevenue);

    type Trader = MultiCurrencyTrader<AssetId, Balance, IdentityFee<Balance>, MockOracle, MockConvert, ExpectRevenue>;

    let core_id = MockConvert::convert(CORE_ASSET_ID).unwrap();
    let test_id = MockConvert::convert(TEST_ASSET_ID).unwrap();

    {
        let mut trader = Trader::new();

        let weight = 1_000_000;
        let core_payment: MultiAsset = (Concrete(core_id), 1_000_000).into();
        let res = dbg!(trader.buy_weight(weight, core_payment.clone().into()));
        assert!(res
            .expect("buy_weight should succeed because payment == weight")
            .is_empty());

        let test_payment: MultiAsset = (Concrete(test_id), 500_000).into();
        let res = dbg!(trader.buy_weight(weight, test_payment.clone().into()));
        assert!(res
            .expect("buy_weight should succeed because payment == 0.5 * weight")
            .is_empty());

        assert_eq!(trader.refund_weight(weight), Some(core_payment.into()));
        assert_eq!(trader.refund_weight(weight), Some(test_payment.into()));
    }
    ExpectRevenue::expect_no_revenue();
}

macro_rules! generate_deposit_type {
    ($name:ident) => {
        lazy_static::lazy_static! {
            pub static ref EXPECTED: Mutex<BTreeSet<(AccountId, AssetId, Balance)>> =
                Mutex::new(BTreeSet::new());
        };

        struct $name;
        impl $name {
            #[allow(unused)]
            fn register_expected_fee(who: AccountId, asset: AssetId, amount: Balance) {
                EXPECTED.lock().unwrap().insert((who, asset, amount));
            }

            /// Reset the global mutable state.
            #[allow(unused)]
            fn reset() {
                *EXPECTED.lock().unwrap() = BTreeSet::new();
            }
        }

        impl DepositFee<AccountId, AssetId, Balance> for $name {
            fn deposit_fee(who: &AccountId, asset: AssetId, amount: Balance) -> DispatchResult {
                log::trace!("Depositing {} of {} to {}", amount, asset, who);
                assert!(
                    EXPECTED.lock().unwrap().remove(&(*who, asset, amount)),
                    "Unexpected combination of receiver and fee {:?} deposited that was not expected.",
                    (*who, asset, amount)
                );
                let remaining = EXPECTED.lock().unwrap();
                assert!(
                    remaining.is_empty(),
                    "There should be no expected fees remaining. Remaining: {:?}",
                    remaining
                );
                Ok(())
            }
        }
    };
}

#[test]
fn revenue_goes_to_fee_receiver() {
    generate_deposit_type!(ExpectDeposit);

    struct MockFeeReceiver;
    impl TransactionMultiPaymentDataProvider<AccountId, AssetId, Price> for MockFeeReceiver {
        fn get_currency_and_price(_who: &AccountId) -> Result<(AssetId, Option<Price>), DispatchError> {
            Err("not implemented".into())
        }

        fn get_fee_receiver() -> AccountId {
            42
        }
    }

    type Revenue = ToFeeReceiver<AccountId, AssetId, Balance, Price, MockConvert, ExpectDeposit, MockFeeReceiver>;

    let core_id = MockConvert::convert(CORE_ASSET_ID).unwrap();

    ExpectDeposit::register_expected_fee(42, CORE_ASSET_ID, 1234);

    Revenue::take_revenue((core_id, 1234).into());
}
