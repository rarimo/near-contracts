use near_sdk::json_types::U128;
use shared::{Data, u128_to_bytes, vector_from_32_bytes};

use crate::fee_tokens::{FeeToken, TokenType};
use crate::types::OperationType;

pub struct OperationData {
    pub operation_type: OperationType,
    pub token: FeeToken,
    pub amount: Option<U128>
}

impl OperationData {
    pub fn new(operation_type: OperationType, token: FeeToken, amount: Option<U128>) -> Self {
        OperationData {
            operation_type,
            token,
            amount,
        }
    }
}


impl Data for OperationData {
    fn get_data(&self) -> Vec<u8> {
        let mut data = Vec::new();

        data.push(self.operation_type.clone().into());

        match self.token.token_type {
            TokenType::Native => {
                // Nothing to add
            }
            TokenType::NFT | TokenType::FT => {
                let token_bytes: &mut Vec<u8> = &mut prefix_hex::decode(&*prefix_hex::encode(self.token.token_addr.clone().unwrap().as_bytes())).unwrap();
                data.append(token_bytes);
            }
        }

        let amount = match self.amount {
            Some(amount) => amount,
            None => self.token.fee.clone(),
        };

        data.append(&mut vector_from_32_bytes(u128_to_bytes(amount)));
        data
    }
}
