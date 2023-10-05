use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::{AccountId, env, log, near_bindgen, ONE_YOCTO, Promise, PromiseError, PromiseOrValue, serde_json};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::panic_str;
use near_sdk::json_types::U128;
use serde::{Deserialize, Serialize};

use shared::{ContentNode, Data, GAS_FOR_TX, get_merkle_root, Hash, Secp256K1Signature, SignerPublicKey, TransferLog, TokenType, verify_ecdsa_signature};

use crate::fee_tokens::{FeeToken};
use crate::merkle::OperationData;
use crate::types::{DepositLog, FeeManageOperation, OperationType, TransferType};

use super::*;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Clone)]
pub struct DepositOperation {
    pub owner: AccountId,
    pub deposited: bool,
    pub fee_charged: bool,
    pub token_addr: Option<AccountId>,
    pub token_type: Option<TokenType>,
    pub fee_token_addr: Option<AccountId>,
    pub receiver: Option<String>,
    pub msg: Option<String>,
    pub amount: Option<U128>,
    pub token_id: Option<TokenId>,
}

impl DepositOperation {
    pub fn populate_from_raw(&mut self, log: DepositLog, amount: Option<U128>, token_id: Option<TokenId>) {
        if self.token_addr.is_none() {
            self.token_addr = log.token_addr.clone();
        }
        if self.token_type.is_none() {
            self.token_type = Some(log.token_type.clone());
        }
        if self.fee_token_addr.is_none() {
            self.fee_token_addr = log.fee_token_addr.clone();
        }
        if self.receiver.is_none() {
            self.receiver = Some(log.receiver.clone());
        }
        if self.msg.is_none() {
            self.msg = Some(msg_from_deposit_log(self.owner.clone(), log.clone()));
        }

        if log.transfer_type == TransferType::Deposit && amount.is_some() {
            self.amount = amount.clone();
        }

        if self.token_id.is_none() && token_id.is_some() {
            self.token_id = token_id.clone();
        }
    }

    pub fn is_equals(&self, log: DepositLog) -> bool {
        if self.token_addr.is_some() {
            if let Some(token_addr) = self.token_addr.clone() {
                if token_addr != log.token_addr.clone().unwrap() {
                    log!("Token address is not equal to deposit token address");
                    return false;
                }
            }
        }

        if (self.fee_token_addr.is_none() && log.fee_token_addr.is_some()) || (self.fee_token_addr.is_some() && log.fee_token_addr.is_none()) {
            log!("Fee token address is not equal to deposit fee token address");
            return false;
        }

        if let Some(fee_token_addr) = self.fee_token_addr.clone() {
            if fee_token_addr != log.fee_token_addr.clone().unwrap() {
                log!("Fee token address is not equal to deposit fee token address");
                return false;
            }
        }

        if let Some(receiver) = self.receiver.clone() {
            if receiver != log.receiver.clone() {
                log!("Receiver is not equal to deposit receiver");
                return false;
            }
        }

        if let Some(msg) = self.msg.clone() {
            if msg != msg_from_deposit_log(self.owner.clone(), log.clone()) {
                log!("Msg is not equal to deposit msg");
                return false;
            }
        }

        if self.token_type.is_some() {
            if self.token_type.clone().unwrap() != log.token_type.clone() {
                log!("Token type is not equal to deposit token type");
                return false;
            }
        }

        return true;
    }
}

#[near_bindgen]
impl Feer {
    pub fn get_deposit_op(&mut self, owner: AccountId) -> Option<DepositOperation> {
        let op = self.deposit_operations.get_deposit_op(owner).unwrap().clone();
        Some(op)
    }

    pub fn add_fee_token(
        &mut self,
        op: FeeManageOperation,
    ) -> Promise {
        ext_bridge::ext(self.bridge_addr.clone()).get_signer()
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(GAS_FOR_TX)
                    .add_fee_token_callback(op.clone())
            )
    }

    pub fn update_fee_token(
        &mut self,
        op: FeeManageOperation,
    ) -> Promise {
        ext_bridge::ext(self.bridge_addr.clone()).get_signer()
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(GAS_FOR_TX)
                    .update_fee_token_callback(op.clone())
            )
    }

    pub fn remove_fee_token(&mut self, op: FeeManageOperation) -> Promise {
        ext_bridge::ext(self.bridge_addr.clone()).get_signer()
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(GAS_FOR_TX)
                    .remove_fee_token_callback(op.clone())
            )
    }

    pub fn withdraw(&mut self, op: FeeManageOperation, amount: U128, receiver: AccountId) -> Promise {
        ext_bridge::ext(self.bridge_addr.clone()).get_signer()
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(GAS_FOR_TX)
                    .withdraw_callback(op.clone(), amount.clone(), receiver.clone())
            )
    }

    pub fn get_fee_token(&self, token_addr: Option<AccountId>) -> Option<FeeToken> {
        self.tokens.get_fee_token(token_addr)
    }

    pub fn get_fee_tokens(&self) -> Vec<FeeToken> {
        self.tokens.get_fee_tokens()
    }


    #[private]
    pub fn get_signer_public_key(&self) -> PromiseOrValue<SignerPublicKey> {
        PromiseOrValue::Promise(ext_bridge::ext(self.bridge_addr.clone()).get_signer())
    }

    #[private]
    pub fn add_fee_token_callback(
        &mut self,
        #[callback_result] call_result: Result<SignerPublicKey, PromiseError>,
        op: FeeManageOperation,
    ) {
        self.handle_operation(
            unwrap_signer_public_key(call_result),
            OperationType::AddFeeToken,
            op.clone(),
            None,
        );
        self.tokens.add_fee_token(op.token);
    }

    #[private]
    pub fn update_fee_token_callback(
        &mut self,
        #[callback_result] call_result: Result<SignerPublicKey, PromiseError>,
        op: FeeManageOperation,
    ) {
        self.handle_operation(
            unwrap_signer_public_key(call_result),
            OperationType::UpdateFeeToken,
            op.clone(),
            None,
        );
        self.tokens.update_fee_token(op.token);
    }

    #[private]
    pub fn remove_fee_token_callback(
        &mut self,
        #[callback_result] call_result: Result<SignerPublicKey, PromiseError>,
        op: FeeManageOperation,
    ) {
        self.handle_operation(
            unwrap_signer_public_key(call_result),
            OperationType::RemoveFeeToken,
            op.clone(),
            None,
        );
        self.tokens.remove_fee_token(op.token.token_addr);
    }

    #[private]
    pub fn withdraw_callback(
        &mut self,
        #[callback_result] call_result: Result<SignerPublicKey, PromiseError>,
        op: FeeManageOperation,
        amount: U128,
        receiver: AccountId,
    ) {
        self.handle_operation(
            unwrap_signer_public_key(call_result),
            OperationType::Withdraw,
            op.clone(),
            Some(amount.clone()),
        );
        self.internal_withdraw(op.token.clone(), amount.clone(), receiver.clone());
    }

    #[private]
    pub fn internal_withdraw(&mut self, fee_token: FeeToken, amount: U128, receiver: AccountId) {
        match fee_token.token_type {
            TokenType::Native => {
                Promise::new(receiver).transfer(amount.into());
            }
            TokenType::FT => {
                ext_fungible_token::ext(fee_token.token_addr.unwrap().clone())
                    .with_static_gas(GAS_FOR_TX)
                    .with_attached_deposit(ONE_YOCTO)
                    .ft_transfer(receiver.clone(), amount.into(), None);
            }
            _ => {
                panic_str("Unsupported token type");
            }
        }
    }

    #[private]
    pub fn handle_operation(&mut self, signer: SignerPublicKey, op_type: OperationType, op: FeeManageOperation, amount: Option<U128>) {
        let signature = Secp256K1Signature::from_hex(op.clone().signature);
        let origin = Hash::from_hex(op.origin.clone());
        let receiver: Option<AccountId> = match op_type.clone() {
            OperationType::Withdraw => Some(env::current_account_id().clone()),
            _ => None,
        };

        let data = OperationData::new(op_type.clone(), op.clone().token, amount).get_data();

        let content = ContentNode::new(
            origin,
            self.bridge_addr.clone(),
            self.chain.clone(),
            data,
            receiver,
        );

        verify_ecdsa_signature(signer, get_merkle_root(content, &op.path), signature, op.recovery_id);
        self.hashes.check_hash(origin.clone());
    }
}

fn msg_from_deposit_log(sender: AccountId, log: DepositLog) -> String {
    let transfer_log = TransferLog {
        sender,
        receiver: log.receiver.clone(),
        chain_to: log.chain_to.clone(),
        is_wrapped: log.is_wrapped,
        bundle_data: log.bundle_data.clone(),
        bundle_salt: log.bundle_salt.clone(),
    };

    serde_json::to_string(&transfer_log).unwrap()
}

fn unwrap_signer_public_key(call_result: Result<SignerPublicKey, PromiseError>) -> SignerPublicKey {
    match call_result {
        Ok(signer) => signer,
        Err(_) => panic_str("failed to get bridge signer public key"),
    }
}
