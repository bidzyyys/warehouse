//                    :                     $$\   $$\                 $$\                    $$$$$$$\  $$\   $$\
//                  !YJJ^                   $$ |  $$ |                $$ |                   $$  __$$\ $$ |  $$ |
//                7B5. ~B5^                 $$ |  $$ |$$\   $$\  $$$$$$$ | $$$$$$\  $$$$$$\  $$ |  $$ |\$$\ $$  |
//             .?B@G    ~@@P~               $$$$$$$$ |$$ |  $$ |$$  __$$ |$$  __$$\ \____$$\ $$ |  $$ | \$$$$  /
//           :?#@@@Y    .&@@@P!.            $$  __$$ |$$ |  $$ |$$ /  $$ |$$ |  \__|$$$$$$$ |$$ |  $$ | $$  $$<
//         ^?J^7P&@@!  .5@@#Y~!J!.          $$ |  $$ |$$ |  $$ |$$ |  $$ |$$ |     $$  __$$ |$$ |  $$ |$$  /\$$\
//       ^JJ!.   :!J5^ ?5?^    ^?Y7.        $$ |  $$ |\$$$$$$$ |\$$$$$$$ |$$ |     \$$$$$$$ |$$$$$$$  |$$ /  $$ |
//     ~PP: 7#B5!.         :?P#G: 7G?.      \__|  \__| \____$$ | \_______|\__|      \_______|\_______/ \__|  \__|
//  .!P@G    7@@@#Y^    .!P@@@#.   ~@&J:              $$\   $$ |
//  !&@@J    :&@@@@P.   !&@@@@5     #@@P.             \$$$$$$  |
//   :J##:   Y@@&P!      :JB@@&~   ?@G!                \______/
//     .?P!.?GY7:   .. .    ^?PP^:JP~
//       .7Y7.  .!YGP^ ?BP?^   ^JJ^         This file is part of https://github.com/galacticcouncil/HydraDX-node
//         .!Y7Y#@@#:   ?@@@G?JJ^           Built with <3 for decentralisation.
//            !G@@@Y    .&@@&J:
//              ^5@#.   7@#?.               Copyright (C) 2021-2023  Intergalactic, Limited (GIB).
//                :5P^.?G7.                 SPDX-License-Identifier: Apache-2.0
//                  :?Y!                    Licensed under the Apache License, Version 2.0 (the "License");
//                                          you may not use this file except in compliance with the License.
//                                          http://www.apache.org/licenses/LICENSE-2.0

#![cfg_attr(not(feature = "std"), no_std)]

use codec::MaxEncodedLen;
use frame_support::{pallet_prelude::*, require_transactional, transactional};
use frame_system::{ensure_signed, pallet_prelude::OriginFor};
use hydradx_traits::Registry;
use orml_traits::{GetByKey, MultiCurrency, MultiReservableCurrency, NamedMultiReservableCurrency};
use sp_runtime::{
    traits::{BlakeTwo256, Hash, One},
    DispatchError,
};

use sp_std::{result, vec::Vec};
#[cfg(test)]
mod tests;

#[cfg(any(feature = "runtime-benchmarks", test))]
mod benchmarks;

pub mod types;
pub mod weights;

use weights::WeightInfo;

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

use crate::types::*;

#[frame_support::pallet]
pub mod pallet {

    use super::*;
    use codec::HasCompact;

    #[pallet::pallet]
    #[pallet::generate_store(pub(crate) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Identifier for the class of asset.
        type AssetId: Member
            + Parameter
            + Ord
            + Default
            + Copy
            + HasCompact
            + MaybeSerializeDeserialize
            + MaxEncodedLen
            + TypeInfo;

        /// Asset Registry mechanism - used to check if asset is correctly registered in asset registry
        type AssetRegistry: Registry<Self::AssetId, Vec<u8>, Balance, DispatchError>;

        /// Named reservable multi currency
        type Currency: MultiCurrency<Self::AccountId, CurrencyId = Self::AssetId, Balance = Balance>
            + NamedMultiReservableCurrency<Self::AccountId, ReserveIdentifier = NamedReserveIdentifier>;

        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type ExistentialDeposits: GetByKey<Self::AssetId, Balance>;

        #[pallet::constant]
        type ExistentialDepositMultiplier: Get<u8>;

        /// Weight information for the extrinsics.
        type WeightInfo: WeightInfo;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(crate) fn deposit_event)]
    pub enum Event<T: Config> {
        /// An Order has been cancelled
        OrderCancelled { order_id: OrderId },
        /// An Order has been completely filled
        OrderFilled {
            order_id: OrderId,
            who: T::AccountId,
            amount: Balance,
        },
        /// An Order has been partially filled
        OrderPartiallyFilled {
            order_id: OrderId,
            who: T::AccountId,
            amount: Balance,
            amount_receive: Balance,
        },
        /// An Order has been placed
        OrderPlaced {
            order_id: OrderId,
            asset_buy: T::AssetId,
            asset_sell: T::AssetId,
            amount_buy: Balance,
            amount_sell: Balance,
            partially_fillable: bool,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Asset does not exist in registry
        AssetNotRegistered,
        /// The asset used to fill the order is different than asset_buy of the order
        AssetNotInOrder,
        /// When filling and order, the fill amount cannot be greater than the remaining order amount
        CannotFillMoreThanOrdered,
        /// Free balance is too low to place the order
        InsufficientBalance,
        /// Order cannot be found
        OrderNotFound,
        /// Size of order ID exceeds the bound
        OrderIdOutOfBound,
        /// Cannot partially fill an order which is not partially fillable
        OrderNotPartiallyFillable,
        /// Order amount_buy and amount_sell must at all times be greater than the existential deposit
        /// for the asset multiplied by the ExistentialDepositMultiplier.
        /// A fill order may not leave behind amounts smaller than this.
        OrderAmountTooSmall,
        /// Error with math calculations
        MathError,
        /// The caller does not have permission to complete the action
        NoPermission,
    }

    /// ID sequencer for Orders
    #[pallet::storage]
    #[pallet::getter(fn next_order_id)]
    pub type NextOrderId<T: Config> = StorageValue<_, OrderId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn orders)]
    pub type Orders<T: Config> = StorageMap<_, Blake2_128Concat, OrderId, Order<T::AccountId, T::AssetId>, OptionQuery>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(<T as Config>::WeightInfo::place_order())]
        #[transactional]
        pub fn place_order(
            origin: OriginFor<T>,
            asset_buy: T::AssetId,
            asset_sell: T::AssetId,
            amount_buy: Balance,
            amount_sell: Balance,
            partially_fillable: bool,
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;

            let order = Order {
                owner,
                asset_buy,
                asset_sell,
                amount_buy,
                partially_fillable,
            };

            Self::validate_place_order(&order, amount_sell)?;

            let order_id = <NextOrderId<T>>::try_mutate(|next_id| -> result::Result<OrderId, DispatchError> {
                let current_id = *next_id;
                *next_id = next_id.checked_add(One::one()).ok_or(Error::<T>::OrderIdOutOfBound)?;
                Ok(current_id)
            })?;

            let reserve_id = Self::named_reserve_identifier(order_id);
            T::Currency::reserve_named(&reserve_id, order.asset_sell, &order.owner, amount_sell)?;

            <Orders<T>>::insert(order_id, &order);
            Self::deposit_event(Event::OrderPlaced {
                order_id,
                asset_buy: order.asset_buy,
                asset_sell: order.asset_sell,
                amount_buy: order.amount_buy,
                amount_sell,
                partially_fillable: order.partially_fillable,
            });

            Ok(())
        }

        #[pallet::weight(<T as Config>::WeightInfo::fill_order())]
        #[transactional]
        pub fn fill_order(
            origin: OriginFor<T>,
            order_id: OrderId,
            asset: T::AssetId,
            amount: Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            <Orders<T>>::try_mutate_exists(order_id, |maybe_order| -> DispatchResult {
                let order = maybe_order.as_mut().ok_or(Error::<T>::OrderNotFound)?;

                let amount_sell = Self::fetch_amount_sell(order_id, order);

                let amount_receive = Self::calculate_amount_receive(order, amount_sell, amount)?;

                Self::validate_fill_order(order, &who, asset, amount, amount_sell, amount_receive)?;

                Self::execute_deal(order_id, order, &who, amount, amount_receive)?;

                let remaining_amount_buy = Self::calculate_difference(order.amount_buy, amount)?;

                if remaining_amount_buy > 0 {
                    let updated_order = Self::update_order(order, remaining_amount_buy);
                    *order = updated_order;

                    Self::deposit_event(Event::OrderPartiallyFilled {
                        order_id,
                        who,
                        amount,
                        amount_receive,
                    });
                } else {
                    // cleanup storage
                    *maybe_order = None;
                    Self::deposit_event(Event::OrderFilled { order_id, who, amount });
                }

                Ok(())
            })
        }

        #[pallet::weight(<T as Config>::WeightInfo::cancel_order())]
        #[transactional]
        pub fn cancel_order(origin: OriginFor<T>, order_id: OrderId) -> DispatchResult {
            let who = ensure_signed(origin)?;

            <Orders<T>>::try_mutate_exists(order_id, |maybe_order| -> DispatchResult {
                let order = maybe_order.as_mut().ok_or(Error::<T>::OrderNotFound)?;

                ensure!(order.owner == who, Error::<T>::NoPermission);

                let amount_sell = Self::fetch_amount_sell(order_id, order);
                let reserve_id = Self::named_reserve_identifier(order_id);
                T::Currency::unreserve_named(&reserve_id, order.asset_sell, &order.owner, amount_sell);

                *maybe_order = None;

                Self::deposit_event(Event::OrderCancelled { order_id });

                Ok(())
            })
        }
    }
}

impl<T: Config> Pallet<T> {
    fn validate_place_order(order: &Order<T::AccountId, T::AssetId>, amount_sell: Balance) -> DispatchResult {
        ensure!(
            T::AssetRegistry::exists(order.asset_sell),
            Error::<T>::AssetNotRegistered
        );

        ensure!(
            T::AssetRegistry::exists(order.asset_buy),
            Error::<T>::AssetNotRegistered
        );

        ensure!(
            T::Currency::can_reserve(order.asset_sell, &order.owner, amount_sell),
            Error::<T>::InsufficientBalance
        );

        Self::validate_min_order_amount(order.asset_buy, order.amount_buy)?;
        Self::validate_min_order_amount(order.asset_sell, amount_sell)?;

        Ok(())
    }

    fn validate_fill_order(
        order: &Order<T::AccountId, T::AssetId>,
        who: &T::AccountId,
        asset: T::AssetId,
        amount: Balance,
        amount_sell: Balance,
        amount_receive: Balance,
    ) -> DispatchResult {
        ensure!(order.asset_buy == asset, Error::<T>::AssetNotInOrder);

        ensure!(order.amount_buy >= amount, Error::<T>::CannotFillMoreThanOrdered);

        ensure!(
            T::Currency::ensure_can_withdraw(asset, who, amount).is_ok(),
            Error::<T>::InsufficientBalance
        );

        if !order.partially_fillable {
            ensure!(amount == order.amount_buy, Error::<T>::OrderNotPartiallyFillable)
        } else {
            let remaining_amount_buy = Self::calculate_difference(order.amount_buy, amount)?;
            if remaining_amount_buy > 0 {
                Self::validate_min_order_amount(order.asset_buy, remaining_amount_buy)?;
            }

            let remaining_amount_sell = Self::calculate_difference(amount_sell, amount_receive)?;
            if remaining_amount_sell > 0 {
                Self::validate_min_order_amount(order.asset_sell, remaining_amount_sell)?;
            }
        }

        Ok(())
    }

    fn validate_min_order_amount(asset: T::AssetId, amount: Balance) -> DispatchResult {
        let min_amount = T::ExistentialDeposits::get(&asset)
            .checked_mul(T::ExistentialDepositMultiplier::get().into())
            .ok_or(Error::<T>::MathError)?;

        ensure!(amount >= min_amount, Error::<T>::OrderAmountTooSmall);

        Ok(())
    }

    fn named_reserve_identifier(order_id: OrderId) -> [u8; 8] {
        let prefix = b"otc";
        let mut result = [0; 8];
        result[0..3].copy_from_slice(prefix);
        result[3..7].copy_from_slice(&order_id.to_be_bytes());

        let hashed = BlakeTwo256::hash(&result);
        let mut hashed_array = [0; 8];
        hashed_array.copy_from_slice(&hashed.as_ref()[..8]);
        hashed_array
    }

    fn fetch_amount_sell(order_id: OrderId, order: &Order<T::AccountId, T::AssetId>) -> Balance {
        let reserve_id = Self::named_reserve_identifier(order_id);
        T::Currency::reserved_balance_named(&reserve_id, order.asset_sell, &order.owner)
    }

    fn calculate_amount_receive(
        order: &Order<T::AccountId, T::AssetId>,
        amount_sell: Balance,
        amount_fill: Balance,
    ) -> Result<Balance, Error<T>> {
        amount_sell
            .checked_mul(amount_fill)
            .and_then(|v| v.checked_div(order.amount_buy))
            .ok_or(Error::<T>::MathError)
    }

    fn calculate_difference(amount_initial: Balance, amount_change: Balance) -> Result<Balance, Error<T>> {
        amount_initial.checked_sub(amount_change).ok_or(Error::<T>::MathError)
    }

    #[require_transactional]
    fn execute_deal(
        order_id: OrderId,
        order: &Order<T::AccountId, T::AssetId>,
        who: &T::AccountId,
        amount_fill: Balance,
        amount_receive: Balance,
    ) -> DispatchResult {
        let reserve_id = Self::named_reserve_identifier(order_id);
        T::Currency::unreserve_named(&reserve_id, order.asset_sell, &order.owner, amount_receive);

        T::Currency::transfer(order.asset_buy, who, &order.owner, amount_fill)?;

        T::Currency::transfer(order.asset_sell, &order.owner, who, amount_receive)?;

        Ok(())
    }

    fn update_order(
        order: &Order<T::AccountId, T::AssetId>,
        new_amount_buy: Balance,
    ) -> Order<T::AccountId, T::AssetId> {
        Order {
            owner: order.owner.clone(),
            asset_buy: order.asset_buy,
            asset_sell: order.asset_sell,
            amount_buy: new_amount_buy,
            partially_fillable: order.partially_fillable,
        }
    }
}
