use cosmwasm_std::{StdResult, Deps, Uint128, Addr};
use serde::{Deserialize, Serialize};
use cw20::TokenInfoResponse;
use schemars::JsonSchema;

use crate::models::*;
use crate::state::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct FundsInfoResponse {
  pub deposit: Uint128,
  pub payment: Uint128,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct Direction {
  pub from: String,
  pub to: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct LocationsResponse {
  pub exact: Direction,
  pub rough: Direction,
  pub comment: String,
}

pub fn query_get_status(deps: Deps) -> StdResult<Status> {
  STATUS.load(deps.storage)
}

pub fn query_get_token_info(deps: Deps) -> StdResult<(Addr, TokenInfoResponse)> {
  let token_address = TOKEN.load(deps.storage)?;
  let info = TOKEN_INFO.load(deps.storage)?;

  Ok((token_address, info))
}

pub fn query_get_courier(deps: Deps) -> StdResult<String> {
  let courier = COURIER.may_load(deps.storage)?;
  let addr = courier.and_then(|a| Some(a.to_string())).unwrap_or_default();
  Ok(addr)
}

pub fn query_get_funds(deps: Deps) -> StdResult<FundsInfoResponse> {
  let deposit = DEPOSIT_AMOUNT.load(deps.storage)?;
  let payment = PAYMENT_AMOUNT.load(deps.storage)?;

  Ok(FundsInfoResponse { deposit, payment })
}

pub fn query_get_locations(deps: Deps) -> StdResult<LocationsResponse> {
  let exact_from = EXACT_FROM_LOCATION.may_load(deps.storage)?;
  let exact_to = EXACT_TO_LOCATION.may_load(deps.storage)?;

  let rough_from = ROUGH_FROM_LOCATION.load(deps.storage)?;
  let rough_to = ROUGH_TO_LOCATION.load(deps.storage)?;

  let comment = COMMENT.may_load(deps.storage)?;

  Ok(LocationsResponse {
    comment: comment.unwrap_or_default(),

    exact: Direction {
      from: exact_from.unwrap_or_default(),
      to: exact_to.unwrap_or_default(),
    },
    
    rough: Direction {
      from: rough_from,
      to: rough_to,
    },
  })
}
