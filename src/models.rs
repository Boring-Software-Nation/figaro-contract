use cosmwasm_std::{DepsMut, Env, Timestamp};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use crate::ContractError;
use crate::state::*;

// Refund receiver after cancel send request
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub enum RefundReceiver {
  // Courier refund deposit
  Courier,
  // Noone give refund
  NoOne,
  // Owner refund all from contract
  Owner,
  // Courier refund deposit, rest funds from contract give owner
  Both,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub enum AfterRefund {
  SetFailed,
  SetClosed,
  StartOver,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub enum Status {
  // Waiting for payment in the contract from the sender, the basic information is crammed.
  WaitPaymentBySender,
  // If the payment from the sender is received - allow the couriers to pay for the order,
  // the courier's waiting status, at this time responses are written and the sender is expected to approve the courier.
  WaitForCourier,
  // Courier found and approved, waiting for deposit from courier,
  // at this stage and at the next stage, the courier and the sender
  // can refuse the application with a refund
  WaitDepositByCourier,
  // At this stage, the sender is expected to send the exact coordinates of the departure and delivery, and comments.
  // At this point, all participants can still opt out of delivery. If everyone refuses - all data is reset
  WaitSenderDetails,
  // At this stage, it is expected that the courier will pick up the parcel, after he
  // has picked it up - he must be noted in the contract, and the status
  // switches to the progress status, if it doesn't do so after the time
  // expires, the sender can remove it from execution.
  WaitCourierInDepartment,
  // Delivery in progress, the courier has left for the recipient, the delivery
  // countdown has begun if the courier does not deliver the package on time
  // the sender can call the refund method, which will return the funds from
  // the deposit to him, and reset the delivery status and cancel it in the contract
  InProgress,
  // The parcel was delivered, the courier entered the coupon into the contract
  //  and received the payment back and the deposit and the amount of payment for the parcel
  Delivered,
  // The parcel was not delivered on time, the funds together with the
  // deposit were returned to the sender, the contract was closed
  Failed,
  // Send request canceled by sender, funds returned to his wallet, contract closed.
  Closed,
}

impl Status {
  pub fn expected(&self, expected: Self) -> Result<bool, ContractError> {
    let current = self.clone();

    if current != expected {
      Err(ContractError::UnexpectedStatus(current, expected))
    } else {
      Ok(true)
    }
  }
}

// Expiration times (in seconds)
pub fn courier_deposit_time() -> u64 {
  2 * 3600
} // 2 hour

pub fn set_details_time() -> u64 {
  2 * 3600
} // 2 hour

pub fn wait_courier_in_department_time() -> u64 {
  24 * 3600
} // 24 hours (1 day)

pub fn wait_delivery_time() -> u64 {
  24 * 7 * 3600
} // 7 days for delivery

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Copy)]
pub struct ExpirationTimes {
  #[serde(default = "courier_deposit_time")]
  pub for_make_deposit: u64,
  #[serde(default = "set_details_time")]
  pub for_set_details: u64,
  #[serde(default = "wait_courier_in_department_time")]
  pub for_wait_courier_in_department: u64,
  #[serde(default = "wait_delivery_time")]
  pub for_wait_delivery: u64,
}

impl Default for ExpirationTimes {
  fn default() -> Self {
    Self {
      for_wait_courier_in_department: wait_courier_in_department_time(),
      for_make_deposit: courier_deposit_time(),
      for_wait_delivery: wait_delivery_time(),
      for_set_details: set_details_time(),
    }
  }
}

#[rustfmt::skip]
impl ExpirationTimes {
  pub fn set_expiration_by_status(
    &self,
    deps: &mut DepsMut,
    env: Env,
  ) -> Result<bool, ContractError> {
    let status = STATUS.load(deps.storage)?;
    let block_time = env.block.time;

    #[inline]
    fn set_fixation(deps: &mut DepsMut, block_time: Timestamp, time: u64) -> Result<bool, ContractError> {
      FIXATION_TIME.save(deps.storage, &block_time)?;
      AVAILABLE_TIME.save(deps.storage, &time)?;
      Ok(true)
    }

    #[inline]
    fn clear(deps: &mut DepsMut, ) -> Result<bool, ContractError> {
      AVAILABLE_TIME.remove(deps.storage);
      FIXATION_TIME.remove(deps.storage);
      Ok(true)
    }

    match status {
      Status::WaitCourierInDepartment => set_fixation(deps, block_time, self.for_wait_courier_in_department),
      Status::WaitSenderDetails => set_fixation(deps, block_time, self.for_wait_courier_in_department),
      Status::WaitDepositByCourier => set_fixation(deps, block_time, self.for_make_deposit),
      Status::InProgress => set_fixation(deps, block_time, self.for_wait_delivery),
      _ => clear(deps),
    }
  }
}
