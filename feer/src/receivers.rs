use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_contract_standards::non_fungible_token::core::NonFungibleTokenReceiver;
use near_sdk::{AccountId, near_bindgen, PromiseOrValue, serde_json};
use near_sdk::json_types::U128;

use super::*;

#[near_bindgen]
impl NonFungibleTokenReceiver for Feer {
    #[allow(unused_variables)]
    fn nft_on_transfer(
        &mut self,
        sender_id: AccountId,
        previous_owner_id: AccountId,
        token_id: TokenId,
        msg: String,
    ) -> PromiseOrValue<bool> {
        let log: DepositLog = serde_json::from_str(&msg).unwrap();
        let is_valid = is_deposit_log_valid(log.clone(), TokenType::NFT, Some(env::predecessor_account_id()));

        if !is_valid {
            log!("Invalid log");
            return PromiseOrValue::Value(true);
        }

        let ok = self.handle_transfer_receiver(
            sender_id.clone(),
            log.clone(),
            None,
            Some(token_id.clone()),
        );

        if !ok {
            return PromiseOrValue::Value(true);
        }

        let op = self.deposit_operations.get_deposit_op(sender_id.clone()).unwrap().clone();

        if !(op.fee_charged && op.deposited) {
            return PromiseOrValue::Value(false);
        }


        self.handle_bridge_deposit(op.clone());
        self.deposit_operations.reset_deposit_op(sender_id.clone());

        return PromiseOrValue::Value(false);
    }
}

#[near_bindgen]
impl FungibleTokenReceiver for Feer {
    #[allow(unused_variables)]
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        let log: DepositLog = serde_json::from_str(&msg).unwrap();
        let is_valid = is_deposit_log_valid(log.clone(), TokenType::FT, Some(env::predecessor_account_id()));

        if !is_valid {
            log!("Invalid log");
            return PromiseOrValue::Value(amount.clone());
        }

        let ok = self.handle_transfer_receiver(
            sender_id.clone(),
            log.clone(),
            Some(amount.clone()),
            None,
        );

        if !ok {
            return PromiseOrValue::Value(amount.clone());
        }

        let op = self.deposit_operations.get_deposit_op(sender_id.clone()).unwrap().clone();

        if !(op.fee_charged && op.deposited) {
            return PromiseOrValue::Value(U128(0));
        }

        self.handle_bridge_deposit(op.clone());
        self.deposit_operations.reset_deposit_op(sender_id.clone());

        return PromiseOrValue::Value(U128(0));
    }
}

pub fn is_deposit_log_valid(log: DepositLog, token_type: TokenType, token: Option<AccountId>) -> bool {
    match log.transfer_type {
        TransferType::Fee => {
            let fee_token = log.fee_token_addr.clone();

            if token.clone().is_some() && fee_token.clone().is_none() || token.clone().is_none() && fee_token.clone().is_some() {
                log!("Invalid fee token address, some of the fee token addresses are empty");
                return false;
            }

            if token.clone().is_some() && fee_token.clone().is_some() && token.clone().unwrap() != fee_token.clone().unwrap() {
                log!("Fee token address is not equal to the received token address");
                return false;
            }
        }
        TransferType::Deposit => {
            let deposit_token = log.token_addr.clone();

            if log.token_type.clone() != token_type.clone() {
                log!("Deposit token type is not equal to the expected token type");
                return false;
            }

            let is_native_token = token_type.clone() == TokenType::Native;

            if is_native_token && deposit_token.clone().is_some() {
                log!("Deposit token address cannot be set for the native token");
                return false;
            }

            if !is_native_token && deposit_token.clone().is_none() {
                log!("Deposit token address cannot be empty for the non-native token");
                return false;
            }

            if !is_native_token && deposit_token.clone().unwrap() != token.unwrap() {
                log!("Deposit token address is not equal to the received token address");
                return false;
            }
        }
    }

    if log.receiver.clone() == "".to_string() {
        log!("Receiver is empty");
        return false;
    }

    if log.chain_to.clone() == "".to_string() {
        log!("Chain to is empty");
        return false;
    }

    return true;
}
