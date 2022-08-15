use cw2::set_contract_version;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{
  MessageInfo,
  StdResult,
  to_binary,
  StdError,
  Response,
  DepsMut,
  Binary,
  Reply,
  Deps,
  Env,
};

use crate::methods::{
  REPLY_DEPOSIT_RECEIVED_BY_COURIER,
  REPLY_PAYMENT_RECEIVED_BY_SENDER,
  REPLY_PAYMENT_TO_COURIER,
  REPLY_COURIER_REFUND,
  REPLY_OWNER_REFUND,
};

use crate::methods;
use crate::queries;
use crate::error::*;
use crate::msg::*;

const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const CONTRACT_NAME: &str = "crates.io:cosm-figaro";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
  mut deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  msg: InstantiateMsg,
) -> Result<Response, ContractError> {
  set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
  msg.setup(&mut deps, &info)?;

  Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  msg: ExecuteMsg,
) -> Result<Response, ContractError> {
  match msg {
    ExecuteMsg::MakePayForShipping {} => {
      methods::sender_make_pay_for_shipping(deps, env, info)
    }
    ExecuteMsg::AcceptApplication {} => {
      methods::courier_accept_application(deps, env, info)
    }
    ExecuteMsg::MakeDepositForShipping {} => {
      methods::courier_make_deposit_for_shipping(deps, env, info)
    }
    ExecuteMsg::SetDetails { location, comment } => {
      methods::sender_set_details(deps, env, info, location, comment)
    }
    ExecuteMsg::ParcelIssued {} => {
      methods::sender_gave_parcel_to_courier(deps, env, info)
    }
    ExecuteMsg::ConfirmDelivery { sign } => {
      methods::courier_confirm_delivery(deps, env, info, sign)
    }
    ExecuteMsg::CancelDelivery {} => {
      methods::universal_cancel_delivery_and_payback(deps, env, info)
    }
  }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
  match msg {
    QueryMsg::TokenInfo {} => to_binary(&queries::query_get_token_info(deps)?),
    QueryMsg::Locations {} => to_binary(&queries::query_get_locations(deps)?),
    QueryMsg::Courier {} => to_binary(&queries::query_get_courier(deps)?),
    QueryMsg::Status {} => to_binary(&queries::query_get_status(deps)?),
    QueryMsg::Funds {} => to_binary(&queries::query_get_funds(deps)?),
  }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> StdResult<Response> {
  match msg.id {
    REPLY_PAYMENT_TO_COURIER => {
      methods::handle_reply_transfer_payment_to_courier(deps, env, msg)
    }
    REPLY_DEPOSIT_RECEIVED_BY_COURIER => {
      methods::handle_reply_transfer_deposit(deps, env, msg)
    }
    REPLY_PAYMENT_RECEIVED_BY_SENDER => {
      methods::handle_reply_transfer_payment(deps, env, msg)
    }
    REPLY_COURIER_REFUND => methods::handle_reply_transfer_refund(
      deps,
      env,
      msg,
      REPLY_COURIER_REFUND,
    ),
    REPLY_OWNER_REFUND => {
      methods::handle_reply_transfer_refund(deps, env, msg, REPLY_OWNER_REFUND)
    }
    id => Err(StdError::generic_err(format!("Unknown reply id: {}", id))),
  }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
  _deps: DepsMut,
  _env: Env,
  _msg: MigrateMsg,
) -> Result<Response, ContractError> {
  Ok(Response::default())
}
