use near_sdk::AccountId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::env::panic_str;

use crate::deposit_operation::DepositOperation;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct DepositsManager {
    pub deposit_operations: LookupMap<AccountId, DepositOperation>,
}

impl DepositsManager {
    pub fn new() -> Self {
        Self {
            deposit_operations: LookupMap::new(b"deposit_operations".to_vec()),
        }
    }

    pub fn is_exists(&self, owner: &AccountId) -> bool {
        self.deposit_operations.contains_key(&owner)
    }

    pub fn add_deposit_op(&mut self, operation: DepositOperation) {
        self.deposit_operations.insert(&operation.owner.clone(), &operation);
    }

    pub fn get_deposit_op(&self, owner: AccountId) -> Option<DepositOperation> {
        self.deposit_operations.get(&owner)
    }

    pub fn update_deposit_op(&mut self, operation: DepositOperation) {
        self.deposit_operations.insert(&operation.owner.clone(), &operation);
    }

    pub fn reset_deposit_op(&mut self, owner: AccountId) {
        if !self.is_exists(&owner.clone()) {
            panic_str("Deposits: User doesn't exist");
        }

        let mut op = self.deposit_operations.get(&owner).unwrap().clone();

        op.deposited = false;
        op.fee_charged = false;
        op.token_addr = None;
        op.token_type = None;
        op.fee_token_addr = None;
        op.receiver = None;
        op.msg = None;
        op.amount = None;
        op.token_id = None;

        self.deposit_operations.insert(&owner, &op);
    }

    pub fn remove_deposit_op(&mut self, owner: AccountId) {
        self.deposit_operations.remove(&owner);
    }
}

