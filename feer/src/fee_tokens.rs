use near_sdk::AccountId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use serde::{Deserialize, Serialize};
use shared::{TokenType};


#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct FeeToken {
    pub token_addr: Option<AccountId>,
    pub token_type: TokenType,
    pub fee: U128,
}

impl FeeToken {
    pub fn new(token_addr: Option<AccountId>, fee: U128, token_type: TokenType) -> Self {
        Self {
            token_addr,
            fee,
            token_type,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct FeeTokensManager {
    pub tokens: Vec<FeeToken>,
}

impl FeeTokensManager {
    pub fn new(tokens: Option<Vec<FeeToken>>) -> Self {
        let mut _tokens = Vec::new();

        if let Some(inner_tokens) = tokens {
            if !inner_tokens.is_empty() {
                _tokens = inner_tokens;
            }
        }

        Self {
            tokens: _tokens
        }
    }


    pub fn is_exists(&self, token_addr: Option<AccountId>) -> bool {
        self.get_fee_token(token_addr).is_some()
    }

    pub fn is_unique(&self, token_addr: Option<AccountId>) -> bool {
        self.get_fee_token(token_addr).is_none()
    }

    pub fn add_fee_token(&mut self, token: FeeToken) {
        assert!(self.is_unique(token.token_addr.clone()), "Token already exists");
        self.tokens.push(token.clone());
    }

    pub fn update_fee_token(&mut self, token: FeeToken) {
        for _token in self.tokens.iter_mut() {
            if _token.token_addr == token.token_addr {
                _token.token_type = token.token_type;
                _token.fee = token.fee;
                break;
            }
        }
    }

    pub fn remove_fee_token(&mut self, token_addr: Option<AccountId>) {
        self.tokens.retain(|s| s.token_addr != token_addr);
    }

    pub fn get_fee_token(&self, token_addr: Option<AccountId>) -> Option<FeeToken> {
        self.tokens.clone().into_iter().find(|s| s.token_addr == token_addr)
    }

    pub fn get_fee_tokens(&self) -> Vec<FeeToken> {
        self.tokens.clone()
    }
}
