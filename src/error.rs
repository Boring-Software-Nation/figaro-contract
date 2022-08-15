use cosmwasm_std::StdError;
use thiserror::Error;

use crate::models::Status;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
  #[error("{0}")]
  Std(#[from] StdError),

  #[error("Unauthorized")]
  Unauthorized {},

  #[error("Method expects owner")]
  OwnerExpected {},

  #[error("Method expects courier")]
  CourierExpected {},

  #[error("Method expects courier or owner")]
  OwnerOrCourierExpected {},

  #[error("Already Paid")]
  AlreadyPaid {},

  #[error("Owner cannot be a courier")]
  OwnerCannotBeACourier {},

  #[error("The courier has not apply yet")]
  CourierNotApplyYet {},

  #[error("Invalid secp256k1 public key")]
  InvalidPublicKey,

  #[error("Invalid secp256k1 signature")]
  InvalidSignature,

  #[error(
    "Invalid status, you can call this method only with expected status"
  )]
  UnexpectedStatus(Status, Status), // (current_status, expected_status)
}
