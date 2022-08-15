use cosmwasm_std::{
  MessageInfo,
  to_binary,
  StdResult,
  Response,
  StdError,
  DepsMut,
  Reply,
  Env,
};

use crate::msg::DetailsLocationInfo;
use crate::models::*;
use crate::error::*;
use crate::state::*;
use crate::utils::*;

// Contract calls reply id
pub const REPLY_DEPOSIT_RECEIVED_BY_COURIER: u64 = 2;
pub const REPLY_PAYMENT_RECEIVED_BY_SENDER: u64 = 1;
pub const REPLY_PAYMENT_TO_COURIER: u64 = 3;
pub const REPLY_COURIER_REFUND: u64 = 5;
pub const REPLY_OWNER_REFUND: u64 = 4;

pub fn sender_make_pay_for_shipping(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
) -> Result<Response, ContractError> {
  let payment_amount = PAYMENT_AMOUNT.load(deps.storage)?;
  let status = STATUS.load(deps.storage)?;

  status.expected(Status::WaitPaymentBySender)?;
  check_is_owner(&deps, &info)?;

  let contract_balance = get_contract_balance(&deps, &env)?;

  if contract_balance >= payment_amount {
    STATUS.save(deps.storage, &Status::WaitForCourier)?;
    return Ok(Response::new().set_data(to_binary(&true)?));
  }

  let transfer_msg = pay_to_contract_by_sender(
    &deps,
    &env,
    &info,
    payment_amount,
    REPLY_PAYMENT_RECEIVED_BY_SENDER,
  )?;

  let response = Response::new()
    .add_submessage(transfer_msg)
    .set_data(to_binary(&true)?);

  Ok(response)
}

pub fn handle_reply_transfer_payment(
  deps: DepsMut,
  _env: Env,
  msg: Reply,
) -> StdResult<Response> {
  // @TODO: check if additional event checks need to be done
  let _reply = msg.result.into_result().map_err(StdError::generic_err)?;

  let payment_amount = PAYMENT_AMOUNT.load(deps.storage)?;
  let owner = OWNER.load(deps.storage)?;

  // Установить статус ожидания курьера, учитывать что трансфер прошел успешно.
  STATUS.save(deps.storage, &Status::WaitForCourier)?;

  let response = Response::new()
    .add_attribute("action", "owner_made_payment")
    .add_attribute("owner", owner)
    .add_attribute("deposit", payment_amount)
    .set_data(to_binary(&true)?);

  Ok(response)
}

pub fn courier_make_deposit_for_shipping(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
) -> Result<Response, ContractError> {
  let status = STATUS.load(deps.storage)?;
  status.expected(Status::WaitDepositByCourier)?;

  check_is_courier(&deps, &info)?;

  let (payment_amount, deposit_amount) = get_payments_info(&deps)?;
  let contract_balance = get_contract_balance(&deps, &env)?;

  if contract_balance >= payment_amount + deposit_amount {
    STATUS.save(deps.storage, &Status::WaitForCourier)?;
    return Ok(Response::new().set_data(to_binary(&true)?));
  }

  let transfer_msg = pay_to_contract_by_sender(
    &deps,
    &env,
    &info,
    deposit_amount,
    REPLY_DEPOSIT_RECEIVED_BY_COURIER,
  )?;

  Ok(
    Response::new()
      .add_submessage(transfer_msg)
      .set_data(to_binary(&true)?),
  )
}

pub fn handle_reply_transfer_deposit(
  mut deps: DepsMut,
  env: Env,
  msg: Reply,
) -> StdResult<Response> {
  // @TODO: check if additional event checks need to be done
  let _reply = msg.result.into_result().map_err(StdError::generic_err)?;

  let deposit_amount = DEPOSIT_AMOUNT.load(deps.storage)?;

  let courier =
    get_courier(&deps).or(Err(StdError::generic_err("Courier not found")))?;

  // Set the status of waiting for delivery details, consider that the transfer was successful.
  STATUS.save(deps.storage, &Status::WaitSenderDetails)?;

  // set expiration for set details by sender
  let expiration_times = EXPIRATION_TIMES.load(deps.storage)?;
  expiration_times
    .set_expiration_by_status(&mut deps, env)
    .or(Err(StdError::generic_err("Cannot set expiration")))?;

  let response = Response::new()
    .add_attribute("action", "courier_made_deposit")
    .add_attribute("courier", courier)
    .add_attribute("deposit", deposit_amount)
    .set_data(to_binary(&true)?);

  Ok(response)
}

pub fn courier_accept_application(
  mut deps: DepsMut,
  env: Env,
  info: MessageInfo,
) -> Result<Response, ContractError> {
  let status = STATUS.load(deps.storage)?;
  status.expected(Status::WaitForCourier)?;

  if check_is_owner(&deps, &info).is_ok() {
    return Err(ContractError::OwnerCannotBeACourier {});
  }

  COURIER.save(deps.storage, &info.sender)?;
  STATUS.save(deps.storage, &Status::WaitDepositByCourier)?;

  let expiration_times = EXPIRATION_TIMES.load(deps.storage)?;
  expiration_times.set_expiration_by_status(&mut deps, env)?;

  let response = Response::new()
    .add_attribute("action", "courier_accepted_order")
    .add_attribute("courier", info.sender);

  Ok(response)
}

pub fn sender_set_details(
  mut deps: DepsMut,
  env: Env,
  info: MessageInfo,
  location: DetailsLocationInfo,
  comment: String,
) -> Result<Response, ContractError> {
  let status = STATUS.load(deps.storage)?;
  status.expected(Status::WaitSenderDetails)?;

  check_is_owner(&deps, &info)?;

  EXACT_FROM_LOCATION.save(deps.storage, &location.from)?;
  EXACT_TO_LOCATION.save(deps.storage, &location.to)?;
  COMMENT.save(deps.storage, &comment)?;
  STATUS.save(deps.storage, &Status::WaitCourierInDepartment)?;

  let expiration_times = EXPIRATION_TIMES.load(deps.storage)?;
  expiration_times.set_expiration_by_status(&mut deps, env)?;

  let response =
    Response::new().add_attribute("action", "sender_provided_details");

  Ok(response)
}

pub fn universal_cancel_delivery_and_payback(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
) -> Result<Response, ContractError> {
  let courier = get_courier(&deps)?;
  let owner = get_owner(&deps)?;

  let is_courier = info.sender == courier;
  let is_owner = info.sender == owner;

  if !is_owner && !is_courier {
    return Err(ContractError::OwnerOrCourierExpected {});
  }

  let can_cancel = if is_owner {
    owner_can_cancel(&deps, &env)?
  } else {
    courier_can_cancel(&deps, &env)?
  };

  if let Some((refund_receiver, action_after_refund)) = can_cancel {
    let (_, deposit_amount) = get_payments_info(&deps)?;
    let contract_balance = get_contract_balance(&deps, &env)?;

    let mut response = Response::new();

    // Refund messages
    response = match refund_receiver {
      RefundReceiver::Owner => response.add_submessage(pay_from_contract(
        &deps,
        owner,
        contract_balance,
        REPLY_OWNER_REFUND,
      )?),

      RefundReceiver::Courier => response.add_submessage(pay_from_contract(
        &deps,
        courier,
        deposit_amount,
        REPLY_COURIER_REFUND,
      )?),

      RefundReceiver::Both => {
        let owner_amount = contract_balance - deposit_amount;

        response
          .add_submessage(pay_from_contract(
            &deps,
            owner,
            owner_amount,
            REPLY_OWNER_REFUND,
          )?)
          .add_submessage(pay_from_contract(
            &deps,
            courier,
            deposit_amount,
            REPLY_COURIER_REFUND,
          )?)
      }

      RefundReceiver::NoOne => response,
    };

    // actions after refund
    response = match action_after_refund {
      AfterRefund::SetClosed => {
        STATUS.save(deps.storage, &Status::Closed)?;
        response.add_attribute("action", "cancel.closed")
      }

      AfterRefund::SetFailed => {
        STATUS.save(deps.storage, &Status::Failed)?;
        response.add_attribute("action", "cancel.failed")
      }

      AfterRefund::StartOver => {
        STATUS.save(deps.storage, &Status::WaitForCourier)?;
        COURIER.remove(deps.storage);

        EXACT_FROM_LOCATION.remove(deps.storage);
        EXACT_TO_LOCATION.remove(deps.storage);
        COMMENT.remove(deps.storage);

        response.add_attribute("action", "cancel.start_over")
      }
    };

    // Clear fixation times
    AVAILABLE_TIME.remove(deps.storage);
    FIXATION_TIME.remove(deps.storage);

    Ok(response.set_data(to_binary(&true)?))
  } else {
    Ok(Response::new().set_data(to_binary(&false)?))
  }
}

pub fn handle_reply_transfer_refund(
  _deps: DepsMut,
  _env: Env,
  msg: Reply,
  reply_id: u64,
) -> StdResult<Response> {
  // @TODO: check if additional event checks need to be done
  let _reply = msg.result.into_result().map_err(StdError::generic_err)?;

  let refund_receiver = if reply_id == REPLY_OWNER_REFUND {
    "owner"
  } else {
    "courier"
  };

  let response = Response::new()
    .add_attribute("action", "refund_completed")
    .add_attribute("receiver", refund_receiver)
    .set_data(to_binary(&true)?);

  Ok(response)
}

pub fn sender_gave_parcel_to_courier(
  mut deps: DepsMut,
  env: Env,
  info: MessageInfo,
) -> Result<Response, ContractError> {
  let status = STATUS.load(deps.storage)?;
  status.expected(Status::WaitCourierInDepartment)?;
  check_is_owner(&deps, &info)?;

  STATUS.save(deps.storage, &Status::InProgress)?;

  let expiration_times = EXPIRATION_TIMES.load(deps.storage)?;
  expiration_times.set_expiration_by_status(&mut deps, env)?;

  let courier = get_courier(&deps)?;

  let response = Response::new()
    .add_attribute("action", "parcel_gave_to_courier")
    .add_attribute("courier", courier)
    .set_data(to_binary(&true)?);

  Ok(response)
}

pub fn courier_confirm_delivery(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  sign: String,
) -> Result<Response, ContractError> {
  let status = STATUS.load(deps.storage)?;
  status.expected(Status::InProgress)?;

  let (payment_amount, deposit_amount) = get_payments_info(&deps)?;
  let courier = check_is_courier(&deps, &info)?;

  check_courier_signature(
    &deps,
    env.contract.address.to_string(),
    sign,
    CONFIRM_PUBLIC_KEY.load(deps.storage)?,
  )?;

  let transfer_msg = pay_from_contract(
    &deps,
    courier,
    payment_amount + deposit_amount,
    REPLY_PAYMENT_TO_COURIER,
  )?;

  let response = Response::new()
    .add_submessage(transfer_msg)
    .set_data(to_binary(&true)?);

  Ok(response)
}

pub fn handle_reply_transfer_payment_to_courier(
  mut deps: DepsMut,
  env: Env,
  msg: Reply,
) -> StdResult<Response> {
  // @TODO: check whether it is necessary to do an additional check of events that the payment has passed
  let _reply = msg.result.into_result().map_err(StdError::generic_err)?;

  let courier =
    get_courier(&deps).or(Err(StdError::generic_err("Courier not found")))?;

  STATUS.save(deps.storage, &Status::Delivered)?;

  // clear expiration after delivery
  let expiration_times = EXPIRATION_TIMES.load(deps.storage)?;
  expiration_times
    .set_expiration_by_status(&mut deps, env)
    .or(Err(StdError::generic_err("Cannot set expiration")))?;

  let response = Response::new()
    .add_attribute("action", "parcel_delivered")
    .add_attribute("courier", courier)
    .set_data(to_binary(&true)?);

  Ok(response)
}
