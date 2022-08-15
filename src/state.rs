use cosmwasm_std::{Addr, Uint128, Timestamp};
use cw20::TokenInfoResponse;
use cw_storage_plus::Item;
use crate::models::*;

// public key of the order confirmation coupon, according to which the courier receives his reward
pub const CONFIRM_PUBLIC_KEY: Item<String> = Item::new("confirm_public_key");
// config with preset of expiration times for cancel & refunds
pub const EXPIRATION_TIMES: Item<ExpirationTimes> =
  Item::new("expiration_times");
// block time on which the new count was recorded
pub const FIXATION_TIME: Item<Timestamp> = Item::new("fixation_time"); // Option<Timestamp>
// time available for action (for example, for the delivery time after which the courier or sender can cancel the order)
pub const AVAILABLE_TIME: Item<u64> = Item::new("available_time"); // Option<u64>

// the exact coordinates of the delivery departure in geohash format encrypted with the courier's public key
pub const EXACT_FROM_LOCATION: Item<String> =
  Item::new("from_exact_location"); // Option<String>
// exact coordinates of the delivery destination encrypted with the courier's public key in geohash format
pub const EXACT_TO_LOCATION: Item<String> =
  Item::new("to_exact_location"); // Option<String>

// a comment from the sender encrypted with the courier's public key
pub const COMMENT: Item<String> = Item::new("comment"); // Option<String>

// approximate area of the place of departure of the parcel
pub const ROUGH_FROM_LOCATION: Item<String> =
  Item::new("from_rough_location");
// approximate destination area of the parcel
pub const ROUGH_TO_LOCATION: Item<String> =
  Item::new("to_rough_location");

// the amount of the required deposit from the courier, is set when creating the contract and does not change
pub const DEPOSIT_AMOUNT: Item<Uint128> = Item::new("deposit_amount");
// the amount of the delivery fee, is set when creating the contract and does not change
pub const PAYMENT_AMOUNT: Item<Uint128> = Item::new("payment_amount");

// courier account address accepted for delivery
pub const COURIER: Item<Addr> = Item::new("courier"); // Option<Addr>
// account address of the sender, the owner of the shipment
pub const OWNER: Item<Addr> = Item::new("owner");
// address of the cw20 contract, for manipulation and verification of tokens
pub const TOKEN: Item<Addr> = Item::new("token");
// information about the token, from the result of the first check
pub const TOKEN_INFO: Item<TokenInfoResponse> = Item::new("token_info");
// current order and delivery status
pub const STATUS: Item<Status> = Item::new("status");
