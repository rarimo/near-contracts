use near_contract_standards::fungible_token::metadata::FungibleTokenMetadata;
use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
use near_sdk::AccountId;
use near_sdk::ext_contract;
use near_sdk::json_types::U128;

// Validator interfaces, for cross-contract calls

#[ext_contract(ext_non_fungible_token)]
trait NonFungibleToken {
    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    );
    fn nft_mint(
        &mut self,
        token_id: TokenId,
        receiver_id: AccountId,
        token_metadata: TokenMetadata,
        memo: Option<String>,
    );
    fn nft_token(
        &self,
        token_id: TokenId,
    ) -> Option<Token>;
}

#[ext_contract(ext_fungible_token)]
trait FungibleToken {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
    fn ft_mint(&mut self, receiver_id: AccountId, amount: U128);
    fn ft_metadata(&self) -> FungibleTokenMetadata;
}
