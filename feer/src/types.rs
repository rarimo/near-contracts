use near_sdk::AccountId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

use shared::{Hash, RecoveryID};

use crate::fee_tokens::{FeeToken, TokenType};

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Clone)]
pub struct FeeManageOperation {
    pub token: FeeToken,
    pub origin: String,
    pub path: Vec<Hash>,
    pub signature: String,
    pub recovery_id: RecoveryID,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, PartialEq, Debug, Clone)]
pub enum OperationType {
    AddFeeToken,
    RemoveFeeToken,
    UpdateFeeToken,
    Withdraw,
}

impl Into<u8> for OperationType {
    fn into(self) -> u8 {
        match self {
            OperationType::AddFeeToken => 1,
            OperationType::RemoveFeeToken => 2,
            OperationType::UpdateFeeToken => 3,
            OperationType::Withdraw => 4,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, PartialEq, Debug, Clone)]
pub enum TransferType {
    Fee,
    Deposit,
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DepositLog {
    pub fee_token_addr: Option<AccountId>,
    pub token_addr: Option<AccountId>,
    pub token_type: TokenType,
    pub transfer_type: TransferType,
    // Transfer log fields
    pub receiver: String,
    pub chain_to: String,
    pub is_wrapped: bool,
    pub bundle_data: Option<String>,
    pub bundle_salt: Option<String>,
}

