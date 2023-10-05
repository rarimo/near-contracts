use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::{AccountId, Balance, env, log, near_bindgen, ONE_YOCTO, PanicOnDefault, serde_json, StorageUsage};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::panic_str;
use near_sdk::json_types::U128;

use deposit_operation::DepositOperation;
use shared::{GAS_FOR_TX, Hashes, TransferLog};

use crate::deposits::DepositsManager;
use crate::external::*;
use crate::fee_tokens::{FeeToken, FeeTokensManager, TokenType};
use crate::types::{DepositLog, TransferType};
use crate::receivers::{is_deposit_log_valid};

mod fee_tokens;
mod external;
mod merkle;
mod types;
mod deposits;
mod receivers;
mod storage;
mod deposit_operation;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Feer {
    pub chain: String,
    pub bridge_addr: AccountId,
    pub hashes: Hashes,
    pub tokens: FeeTokensManager,
    pub deposit_operations: DepositsManager,
    /// The storage size in bytes for one operation.
    pub operations_storage_usage: StorageUsage,
}

#[near_bindgen]
impl Feer {
    #[init]
    pub fn new(
        chain: String,
        bridge_addr: AccountId,
        tokens: Option<Vec<FeeToken>>,
    ) -> Self {
        let mut this = Self {
            chain,
            bridge_addr,
            hashes: Hashes::new(),
            tokens: FeeTokensManager::new(tokens),
            deposit_operations: DepositsManager::new(),
            operations_storage_usage: 0,
        };

        this.measure_operation_storage_usage();
        this
    }

    #[init(ignore_state)]
    #[private]
    pub fn migrate() -> Self {
        let contract: Feer = env::state_read().expect("ERROR, NOT INITIALIZED");
        contract
    }

    fn measure_operation_storage_usage(&mut self) {
        let initial_storage_usage = env::storage_usage();
        let tmp_account_id = AccountId::new_unchecked("a".repeat(64));

        let transfer_log = TransferLog {
            sender: AccountId::new_unchecked("x".repeat(64)),
            receiver: "b".repeat(64),
            chain_to: "Near".to_string(),
            is_wrapped: false,
            bundle_data: None,
            bundle_salt: None,
        };

        let msg = serde_json::to_string(&transfer_log).unwrap();

        self.deposit_operations.add_deposit_op(DepositOperation {
            owner: tmp_account_id.clone(),
            deposited: false,
            fee_charged: false,
            token_addr: Some(AccountId::new_unchecked("b".repeat(64))),
            token_type: Some(TokenType::Native),
            fee_token_addr: Some(AccountId::new_unchecked("c".repeat(64))),
            receiver: Some(String::from(AccountId::new_unchecked("b".repeat(64)))),
            msg: Some(msg),
            amount: Some(U128::from(10000000000000000)),
            token_id: Some(TokenId::from("1")),
        });

        self.operations_storage_usage = env::storage_usage() - initial_storage_usage;
        self.deposit_operations.remove_deposit_op(tmp_account_id);
    }

    #[payable]
    pub fn charge_native(
        &mut self,
        deposit: DepositLog,
    ) {
        let deposit_amount = env::attached_deposit();
        let sender = env::predecessor_account_id();
        let is_valid = is_deposit_log_valid(deposit.clone(), TokenType::Native, None);

        if !is_valid {
            panic_str("Invalid deposit log");
        }

        let ok = self.handle_transfer_receiver(
            sender.clone(),
            deposit.clone(),
            Some(U128::from(deposit_amount)),
            None,
        );

        if !ok {
            panic_str("Failed to charge native");
        }

        let op = self.deposit_operations.get_deposit_op(sender.clone()).unwrap().clone();

        if !(op.fee_charged && op.deposited) {
            return;
        }

        self.handle_bridge_deposit(op.clone());
        self.deposit_operations.reset_deposit_op(sender.clone());
    }

    #[private]
    pub fn handle_transfer_receiver(
        &mut self,
        sender_id: AccountId,
        log: DepositLog,
        amount: Option<U128>,
        token_id: Option<TokenId>,
    ) -> bool {
        if !self.deposit_operations.is_exists(&sender_id.clone()) {
            log!("Deposits: User with id {} not found", sender_id.clone());
            return false;
        }

        let mut op = self.deposit_operations.get_deposit_op(sender_id.clone()).unwrap().clone();

        if !self.tokens.is_exists(log.fee_token_addr.clone()) {
            log!("Fee token with address {} not found", log.fee_token_addr.clone().unwrap());
            return false;
        }

        let fee_token = self.tokens.get_fee_token(log.fee_token_addr.clone()).unwrap();
        op.populate_from_raw(log.clone(), amount.clone(), token_id.clone());

        // If operation's `deposited` or `fee_charged` fields was changed to the `true` with different
        // transaction before the current one it's required to check that `DepositLog` equals to the
        // previous one.
        if op.deposited || op.fee_charged {
            let is_equals = op.is_equals(log.clone());

            if !is_equals {
                log!("Deposit operation is not equals to log");
                return false;
            }
        }

        match log.transfer_type {
            TransferType::Fee => {
                if op.fee_charged {
                    log!("Fee already charged");
                    return false;
                }

                if amount.is_none() {
                    log!("Amount or token id is empty");
                    return false;
                }

                if amount.unwrap() != fee_token.fee {
                    log!("Fee amount is not equal to fee token fee");
                    return false;
                }

                op.fee_charged = true;
                log!("Fee charged");
            }
            TransferType::Deposit => {
                if op.deposited {
                    log!("Deposit already deposited");
                    return false;
                }

                op.deposited = true;
                log!("Deposited");
            }
        }

        self.deposit_operations.update_deposit_op(op.clone());

        return true;
    }

    #[private]
    pub fn handle_bridge_deposit(&mut self, op: DepositOperation) {
        if op.token_type.is_none() {
            panic_str("Token type is empty");
        }

        let token_type = op.token_type.clone().unwrap();

        match token_type {
            TokenType::Native => {
                let log: TransferLog = serde_json::from_str(&op.msg.unwrap()).unwrap();

                ext_bridge::ext(self.bridge_addr.clone())
                    .with_static_gas(GAS_FOR_TX)
                    .with_attached_deposit(Balance::from(op.amount.clone().unwrap()))
                    .native_deposit(
                        env::current_account_id(),
                        op.receiver.clone().unwrap(),
                        log.chain_to.clone(),
                        log.bundle_data.clone(),
                        log.bundle_salt.clone(),
                    );
            }
            TokenType::FT => {
                ext_fungible_token::ext(op.token_addr.clone().unwrap())
                    .with_static_gas(GAS_FOR_TX)
                    .with_attached_deposit(ONE_YOCTO)
                    .ft_transfer_call(self.bridge_addr.clone(), op.amount.clone().unwrap(), None, op.msg.clone().unwrap());
            }
            TokenType::NFT => {
                ext_non_fungible_token::ext(op.token_addr.clone().unwrap())
                    .with_static_gas(GAS_FOR_TX)
                    .with_attached_deposit(ONE_YOCTO)
                    .nft_transfer_call(self.bridge_addr.clone(), op.token_id.clone().unwrap(), None, None, op.msg.clone().unwrap());
            }
        }
    }
}
