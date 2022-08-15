use cw20::{BalanceResponse, Cw20QueryMsg, TokenInfoResponse};
use sha2::{Digest, Sha256};
use cw_asset::AssetBase;
use hex::FromHex;

use cosmwasm_std::{
  QueryRequest,
  MessageInfo,
  WasmQuery,
  to_binary,
  StdResult,
  DepsMut,
  Uint128,
  SubMsg,
  Addr,
  Env,
};

use crate::models::*;
use crate::error::*;
use crate::state::*;

pub fn get_token_info(
  deps: &DepsMut,
  contract_addr: String,
) -> StdResult<TokenInfoResponse> {
  let msg = to_binary(&Cw20QueryMsg::TokenInfo {})?;

  let info: TokenInfoResponse = deps
    .querier
    .query(&QueryRequest::Wasm(WasmQuery::Smart { contract_addr, msg }))?;

  Ok(info)
}

pub fn get_balance(
  deps: &DepsMut,
  token_contract_address: String,
  address: String,
) -> StdResult<Uint128> {
  let msg = to_binary(&Cw20QueryMsg::Balance { address })?;

  let info: BalanceResponse =
    deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
      contract_addr: token_contract_address,
      msg,
    }))?;

  Ok(info.balance)
}

pub fn get_contract_balance(
  deps: &DepsMut,
  env: &Env,
) -> Result<Uint128, ContractError> {
  let token_contract_address = TOKEN.load(deps.storage)?;

  get_balance(
    deps,
    token_contract_address.to_string(),
    env.contract.address.to_string(),
  )
  .map_err(|error| ContractError::Std(error))
}

pub fn get_payments_info(
  deps: &DepsMut,
) -> Result<(Uint128, Uint128), ContractError> {
  let payment_amount = PAYMENT_AMOUNT.load(deps.storage)?;
  let deposit_amount = DEPOSIT_AMOUNT.load(deps.storage)?;

  Ok((payment_amount, deposit_amount))
}

pub fn pay_to_contract_by_sender(
  deps: &DepsMut,
  env: &Env,
  info: &MessageInfo,

  amount: Uint128,
  reply_id: u64,
) -> Result<SubMsg, ContractError> {
  let token_contract_address = TOKEN.load(deps.storage)?;

  let transfer_msg = AssetBase::cw20(token_contract_address, amount)
    .transfer_from_msg(
      info.sender.to_string(),
      env.contract.address.to_string(),
    )?;

  Ok(SubMsg::reply_on_success(transfer_msg, reply_id))
}

pub fn pay_from_contract(
  deps: &DepsMut,
  receiver: Addr,
  amount: Uint128,
  reply_id: u64,
) -> Result<SubMsg, ContractError> {
  let token_contract_address = TOKEN.load(deps.storage)?;

  let transfer_msg =
    AssetBase::cw20(token_contract_address, amount).transfer_msg(receiver)?;

  Ok(SubMsg::reply_on_success(transfer_msg, reply_id))
}

pub fn get_owner(deps: &DepsMut) -> Result<Addr, ContractError> {
  OWNER
    .load(deps.storage)
    .map_err(|error| ContractError::Std(error))
}

pub fn check_is_owner(
  deps: &DepsMut,
  info: &MessageInfo,
) -> Result<Addr, ContractError> {
  let owner = get_owner(deps)?;

  if owner != info.sender {
    Err(ContractError::OwnerExpected {})
  } else {
    Ok(owner)
  }
}

pub fn get_courier(deps: &DepsMut) -> Result<Addr, ContractError> {
  COURIER
    .may_load(deps.storage)
    .map_err(|error| ContractError::Std(error))
    .and_then(|r| r.ok_or(ContractError::CourierNotApplyYet {}))
}

pub fn check_is_courier(
  deps: &DepsMut,
  info: &MessageInfo,
) -> Result<Addr, ContractError> {
  let courier = get_courier(deps)?;

  if info.sender != courier {
    Err(ContractError::CourierExpected {})
  } else {
    Ok(courier)
  }
}

pub fn is_time_over(deps: &DepsMut, env: &Env) -> Result<bool, ContractError> {
  let fixation_time = FIXATION_TIME
    .may_load(deps.storage)
    .and_then(|t| Ok(t.unwrap_or_default()))?;

  let available_time = AVAILABLE_TIME
    .may_load(deps.storage)
    .and_then(|t| Ok(t.unwrap_or_default()))?;

  let expiration_time = fixation_time.plus_seconds(available_time);
  Ok(expiration_time > env.block.time)
}

#[rustfmt::skip]
pub fn owner_can_cancel(
  deps: &DepsMut,
  env: &Env,
) -> Result<Option<(RefundReceiver, AfterRefund)>, ContractError> {
  let status = STATUS.load(deps.storage)?;

  #[inline]
  fn check(
    deps: &DepsMut,
    env: &Env,
    receiver: RefundReceiver,
    action: AfterRefund,
  ) -> Result<Option<(RefundReceiver, AfterRefund)>, ContractError> {
    let can_cancel = is_time_over(deps, env)?;
    Ok(can_cancel.then(|| (receiver, action)))
  }

  match status {
    // Refund both - payment and deposit, if expiration time is over,
    // because courier dont give parcel. Set status `Failed` for contract
    Status::WaitCourierInDepartment => {
      check(deps, env, RefundReceiver::Both, AfterRefund::SetFailed)
    }
    // Refund both - payment and deposit, if expiration time is over,
    // because courier dont give details of location or parcel. Set status `Failed` for contract
    Status::WaitSenderDetails => {
      check(deps, env, RefundReceiver::Both, AfterRefund::SetFailed)
    }
    // Refund owner payment, and close contract, if expiration time is over
    // (?) maybe add variation for cancel courier and start sratch courier over
    Status::WaitDepositByCourier => {
      check(deps, env, RefundReceiver::Owner, AfterRefund::SetFailed)
    }
    // Refund payment and deposit to owner, if expiration time is over,
    // because his give parcel, and time for delivery is over.
    Status::InProgress => {
      check(deps, env, RefundReceiver::Owner, AfterRefund::SetFailed)
    }

    other_case => {
      // if owner only create contract or make payment but dont find courier
      // he can close contract and give all funds
      Ok(
        (other_case == Status::WaitPaymentBySender
          || other_case == Status::WaitForCourier)
          .then(|| (RefundReceiver::Owner, AfterRefund::SetClosed)),
      )
    }
  }
}

#[rustfmt::skip]
pub fn courier_can_cancel(
  deps: &DepsMut,
  env: &Env,
) -> Result<Option<(RefundReceiver, AfterRefund)>, ContractError>
{
  let status = STATUS.load(deps.storage)?;

  #[inline]
  fn check(
    deps: &DepsMut,
    env: &Env,
    receiver: RefundReceiver,
    action: AfterRefund,
  ) -> Result<Option<(RefundReceiver, AfterRefund)>, ContractError>
  {
    let can_cancel = is_time_over(deps, env)?;
    Ok(can_cancel.then(|| (receiver, action)))
  }

  match status {
    // Refund for courier deposit, because courier dont give parcel. After refund start find courier over.
    // With this status = courier can cancel without wait end of expiration
    Status::WaitCourierInDepartment => {
      Ok(Some((RefundReceiver::Courier, AfterRefund::StartOver)))
    }
    // Refund courier deposit, if expiration time is over, 
    // because courier dont give details of location or parcel. 
    Status::WaitSenderDetails => {
      check(deps, env, RefundReceiver::Courier, AfterRefund::StartOver)
    }
    // No-one give refund, after that start find courier over.
    // With this status = courier can cancel without wait end of expiration
    Status::WaitDepositByCourier => {
      Ok(Some((RefundReceiver::NoOne, AfterRefund::StartOver)))
    }
    // Refund payment and deposit to owner, because his give parcel, and time for delivery is over.
    Status::InProgress => {
      check(deps, env, RefundReceiver::Owner, AfterRefund::SetFailed)
    }
    // With other status courier not applied, and only owner can cancel request
    _ => Ok(None),
  }
}

pub fn check_and_serialize_public_key(
  source: String,
) -> Result<String, ContractError> {
  // @TODO: need complex verify
  <Vec<u8>>::from_hex(&*source).or(Err(ContractError::InvalidPublicKey))?;

  Ok(source)
}

pub fn check_courier_signature(
  deps: &DepsMut,
  contract_address: String,
  raw_signature: String,
  confirm_key: String,
) -> Result<(), ContractError> {
  let mut message_digest = Sha256::new();
  message_digest.update(&*contract_address);

  let message_hash = message_digest.finalize();

  let signature = <Vec<u8>>::from_hex(&*raw_signature)
    .or(Err(ContractError::InvalidSignature))?;

  let public_key = <Vec<u8>>::from_hex(&*confirm_key)
    .or(Err(ContractError::InvalidPublicKey))?;

  deps
    .api
    .secp256k1_verify(&message_hash[..], &signature, &public_key)
    .or(Err(ContractError::InvalidSignature))?;

  Ok(())
}
