use near_sdk::AccountId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct TransferLog {
    pub sender: AccountId,
    pub receiver: String,
    pub chain_to: String,
    pub is_wrapped: bool,
    pub bundle_data: Option<String>,
    pub bundle_salt: Option<String>,
}
