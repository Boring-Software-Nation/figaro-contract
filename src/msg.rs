use cosmwasm_std::{Addr, DepsMut, MessageInfo, Uint128};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

use crate::error::ContractError;
use crate::models::*;
use crate::state::*;
use crate::utils::*;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct InstantiateLocationInfo {
  pub from: String,
  pub to: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct DetailsLocationInfo {
  pub from: String,
  pub to: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct InstantiateMsg {
  // approximate areas of delivery, from where and to
  pub location: InstantiateLocationInfo,
  // address of the contract cw20 tokens for payment
  pub token_address: Addr,
  // public key of the coupon for delivery verification
  pub confirm_public_key: String,

  // required deposit from the courier
  pub deposit_amount: Uint128,
  // amount of remuneration for delivery from the sender
  pub payment_amount: Uint128,

  // config for expiration times
  pub expiration_times: Option<ExpirationTimes>,
}

impl InstantiateMsg {
  pub fn setup(
    &self,
    deps: &mut DepsMut,
    info: &MessageInfo,
  ) -> Result<(), ContractError> {
    // check that the contract exists, and persistently save information about it in our contract
    let token_info = get_token_info(deps, self.token_address.to_string())?;
    TOKEN.save(deps.storage, &self.token_address)?;
    TOKEN_INFO.save(deps.storage, &token_info)?;

    // saving basic values
    ROUGH_FROM_LOCATION.save(deps.storage, &self.location.from)?;
    ROUGH_TO_LOCATION.save(deps.storage, &self.location.to)?;

    DEPOSIT_AMOUNT.save(deps.storage, &self.deposit_amount)?;
    PAYMENT_AMOUNT.save(deps.storage, &self.payment_amount)?;

    STATUS.save(deps.storage, &Status::WaitPaymentBySender)?;
    OWNER.save(deps.storage, &info.sender)?;

    let expiration_times = self.expiration_times.unwrap_or_default();
    EXPIRATION_TIMES.save(deps.storage, &expiration_times)?;

    // verify & set public key
    let public_key =
      check_and_serialize_public_key(self.confirm_public_key.clone())?;

    CONFIRM_PUBLIC_KEY.save(deps.storage, &public_key)?;

    Ok(())
  }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
  // Make payment for shipping from the sender
  MakePayForShipping {},
  // The courier accepts the order
  AcceptApplication {},
  // Make a deposit for delivery from the courier
  MakeDepositForShipping {},
  // Update ticket comment with encrypted exact details
  SetDetails {
    location: DetailsLocationInfo,
    comment: String,
  },
  // Cancel Delivery
  CancelDelivery {
    // @TODO: for example, add later the reasons for refusal
    // of delivery by the courier or sender
  },
  // The parcel was given to the courier, delivery in progress
  ParcelIssued {
    // @TODO: maybe add some details?
  },
  // The courier gave the parcel and received a coupon confirming
  // the signature to receive payment and deposit
  ConfirmDelivery {
    sign: String,
  },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
  // Get delivery status
  Status {},
  // Get information about the token used in the contract
  TokenInfo {},
  // Get information about the courier
  Courier {},
  // Get information about funds
  Funds {},
  // Location Information
  Locations {},
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct MigrateMsg {}
