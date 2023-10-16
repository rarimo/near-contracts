use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::AccountId;
use near_sdk::json_types::U128;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NftDepositedData<'a> {
    #[serde(borrow)]
    pub token: &'a str,
    #[serde(borrow)]
    pub token_id: &'a str,
    #[serde(borrow)]
    pub sender: &'a str,
    #[serde(borrow)]
    pub receiver: &'a str,
    #[serde(borrow)]
    pub chain_to: &'a str,
    pub is_wrapped: bool,
    pub bundle_data: Option<String>,
    pub bundle_salt: Option<String>,
}

impl<'a> NftDepositedData<'a> {
    pub fn new(
        token: &'a AccountId,
        token_id: &'a TokenId,
        sender: &'a AccountId,
        receiver: &'a String,
        chain_to: &'a str,
        is_wrapped: bool,
        bundle_data: Option<String>,
        bundle_salt: Option<String>,
    ) -> NftDepositedData<'a> {
        let data = Self {
            token: token.as_str(),
            token_id: token_id.as_str(),
            sender: sender.as_str(),
            receiver,
            chain_to,
            is_wrapped,
            bundle_data,
            bundle_salt,
        };
        data
    }
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FtDepositedData<'a> {
    #[serde(borrow)]
    pub token: &'a str,
    #[serde(borrow)]
    pub sender: &'a str,
    #[serde(borrow)]
    pub receiver: &'a str,
    #[serde(borrow)]
    pub chain_to: &'a str,
    pub amount: U128,
    pub is_wrapped: bool,
    pub bundle_data: Option<String>,
    pub bundle_salt: Option<String>,
}

impl<'a> FtDepositedData<'a> {
    pub fn new(
        token: &'a AccountId,
        amount: U128,
        sender: &'a AccountId,
        receiver: &'a String,
        chain_to: &'a str,
        is_wrapped: bool,
        bundle_data: Option<String>,
        bundle_salt: Option<String>,
    ) -> FtDepositedData<'a> {
        Self {
            token: token.as_str(),
            sender: sender.as_str(),
            receiver,
            amount,
            chain_to,
            is_wrapped,
            bundle_data,
            bundle_salt,
        }
    }
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NativeDepositedData<'a> {
    #[serde(borrow)]
    pub sender: &'a str,
    #[serde(borrow)]
    pub receiver: &'a str,
    #[serde(borrow)]
    pub chain_to: &'a str,
    #[serde(borrow)]
    pub amount: &'a str,
    pub bundle_data: Option<String>,
    pub bundle_salt: Option<String>,
}

impl<'a> NativeDepositedData<'a> {
    pub fn new(
        amount: &'a str,
        sender: &'a AccountId,
        receiver: &'a String,
        chain_to: &'a str,
        bundle_data: Option<String>,
        bundle_salt: Option<String>,
    ) -> NativeDepositedData<'a> {
        Self {
            sender: sender.as_str(),
            receiver,
            amount,
            chain_to,
            bundle_data,
            bundle_salt,
        }
    }
}
