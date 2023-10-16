use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::AccountId;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use shared::{Hash, RecoveryID};

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NftWithdrawnData<'a> {
    #[serde(borrow)]
    pub token: &'a str,
    #[serde(borrow)]
    pub token_id: &'a str,
    #[serde(borrow)]
    pub sender: &'a str,
    #[serde(borrow)]
    pub receiver: &'a str,
    #[serde(borrow)]
    pub origin: &'a str,
    #[serde(borrow)]
    pub signature: &'a str,
    pub path: Vec<String>,
    pub recovery_id: RecoveryID,
    pub is_wrapped: bool,
}

impl<'a> NftWithdrawnData<'a> {
    pub fn new(
        token: &'a AccountId,
        token_id: &'a TokenId,
        sender: &'a AccountId,
        receiver: &'a AccountId,
        origin: &'a str,
        signature: &'a str,
        path: Vec<Hash>,
        recovery_id: RecoveryID,
        is_wrapped: bool,
    ) -> NftWithdrawnData<'a> {
        Self {
            token: token.as_str(),
            token_id: token_id.as_str(),
            sender: sender.as_str(),
            receiver: receiver.as_str(),
            origin,
            signature,
            path: path_to_str_vec(path),
            recovery_id,
            is_wrapped,
        }
    }
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FtWithdrawnData<'a> {
    #[serde(borrow)]
    pub token: &'a str,
    #[serde(borrow)]
    pub sender: &'a str,
    #[serde(borrow)]
    pub receiver: &'a str,
    #[serde(borrow)]
    pub origin: &'a str,
    #[serde(borrow)]
    pub signature: &'a str,
    #[serde(borrow)]
    pub amount: &'a str,
    pub path: Vec<String>,
    pub recovery_id: RecoveryID,
    pub is_wrapped: bool,
}

impl<'a> FtWithdrawnData<'a> {
    pub fn new(
        token: &'a AccountId,
        amount: &'a str,
        sender: &'a AccountId,
        receiver: &'a AccountId,
        origin: &'a str,
        signature: &'a str,
        path: Vec<Hash>,
        recovery_id: RecoveryID,
        is_wrapped: bool,
    ) -> FtWithdrawnData<'a> {
        Self {
            token: token.as_str(),
            sender: sender.as_str(),
            receiver: receiver.as_str(),
            amount,
            origin,
            signature,
            path: path_to_str_vec(path),
            recovery_id,
            is_wrapped,
        }
    }
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NativeWithdrawnData<'a> {
    #[serde(borrow)]
    pub sender: &'a str,
    #[serde(borrow)]
    pub receiver: &'a str,
    #[serde(borrow)]
    pub amount: &'a str,
    #[serde(borrow)]
    pub origin: &'a str,
    #[serde(borrow)]
    pub signature: &'a str,
    pub path: Vec<String>,
    pub recovery_id: RecoveryID,
}

impl<'a> NativeWithdrawnData<'a> {
    pub fn new(
        amount: &'a str,
        sender: &'a AccountId,
        receiver: &'a AccountId,
        origin: &'a str,
        signature: &'a str,
        path: Vec<Hash>,
        recovery_id: RecoveryID,
    ) -> NativeWithdrawnData<'a> {
        Self {
            sender: sender.as_str(),
            receiver: receiver.as_str(),
            amount,
            origin,
            signature,
            path: path_to_str_vec(path),
            recovery_id,
        }
    }
}

fn path_to_str_vec<'a>(path: Vec<Hash>) -> Vec<String> {
    path.iter().map(|h| h.as_str()).collect()
}
