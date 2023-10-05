use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_contract_standards::non_fungible_token::core::NonFungibleTokenReceiver;
use near_sdk::{AccountId, env, PromiseOrValue};
use near_sdk::json_types::U128;

use shared::{BURN_ADDRESS, TRANSFER_DEPOSIT, TransferLog};

use super::*;

#[near_bindgen]
impl NonFungibleTokenReceiver for Bridge {
    #[allow(unused_variables)]
    fn nft_on_transfer(
        &mut self,
        sender_id: AccountId,
        previous_owner_id: AccountId,
        token_id: TokenId,
        msg: String,
    ) -> PromiseOrValue<bool> {
        Bridge::require_unpaused();

        assert_eq!(sender_id.clone(), self.fee_contract.clone(), "Sender must be fee contract");

        let log: TransferLog = serde_json::from_str(&msg).unwrap();

        if log.is_wrapped {
            self.internal_nft_transfer(
                env::predecessor_account_id(),
                token_id.clone(),
                AccountId::new_unchecked(BURN_ADDRESS.to_string()),
                Balance::from(TRANSFER_DEPOSIT),
            );
        }

        NearEvent::nft_deposited(vec![NftDepositedData::new(
            &env::predecessor_account_id(),
            &token_id.clone(),
            &log.sender.clone(),
            &log.receiver.clone(),
            log.chain_to.as_str(),
            log.is_wrapped,
            log.bundle_data.clone(),
            log.bundle_salt.clone(),
        )]).emit();

        PromiseOrValue::Value(false)
    }
}

#[near_bindgen]
impl FungibleTokenReceiver for Bridge {
    #[allow(unused_variables)]
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        Bridge::require_unpaused();

        assert_eq!(sender_id.clone(), self.fee_contract.clone(), "Sender must be fee contract");

        let log: TransferLog = serde_json::from_str(&msg).unwrap();

        if log.is_wrapped {
            self.internal_ft_transfer(
                env::predecessor_account_id(),
                amount,
                AccountId::new_unchecked(BURN_ADDRESS.to_string()),
                Balance::from(TRANSFER_DEPOSIT),
            );
        }

        NearEvent::ft_deposited(vec![FtDepositedData::new(
            &env::predecessor_account_id(),
            amount,
            &log.sender.clone(),
            &log.receiver.clone(),
            log.chain_to.as_str(),
            log.is_wrapped,
            log.bundle_data.clone(),
            log.bundle_salt.clone(),
        )]).emit();

        PromiseOrValue::Value(U128(0))
    }
}
