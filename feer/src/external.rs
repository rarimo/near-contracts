use near_contract_standards::non_fungible_token::TokenId;
use shared::{SignerPublicKey};
use near_sdk::{AccountId, ext_contract, PromiseOrValue};
use near_sdk::json_types::U128;

#[ext_contract(ext_bridge)]
trait Bridge {
    fn get_signer(&self) -> SignerPublicKey;
    fn native_deposit(
        &mut self,
        sender: AccountId,
        receiver_id: String,
        chain: String,
        bundle_data: Option<String>,
        bundle_salt: Option<String>,
    );
}

#[ext_contract(ext_fungible_token)]
trait FtToken {
    fn ft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128>;
    fn ft_transfer(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
    );
}

#[ext_contract(ext_non_fungible_token)]
trait NftToken {
    fn nft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<bool>;
}
