use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

use crate::constants::{HASH_LENGTH, SECP256K1_PUBLIC_KEY_LENGTH, SECP256K1_SIGNATURE_LENGTH};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SignerPublicKey(String);

impl SignerPublicKey {
    pub fn new(public_key: String) -> Self {
        Self(public_key)
    }
    pub fn validate(&self) -> bool {
        bs58::decode(&self.0).into_vec().unwrap().len() == SECP256K1_PUBLIC_KEY_LENGTH
    }
    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct RecoveryID(u8);

impl RecoveryID {
    pub fn unwrap(&self) -> u8 {
        self.0
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Hash([u8; HASH_LENGTH]);

impl Hash {
    pub fn new() -> Self {
        Self([0; HASH_LENGTH])
    }

    pub fn to_slice(&self) -> &[u8] {
        &self.0
    }

    pub fn from_hex(str: String) -> Self {
        Self::from_slice(decode_hex_to_vec(str).as_slice())
    }

    pub fn from_slice(slice: &[u8]) -> Self {
        let mut hash = [0u8; HASH_LENGTH];
        hash.copy_from_slice(slice);
        Hash(hash)
    }

    pub fn unwrap(&self) -> [u8; HASH_LENGTH] {
        self.0
    }

    pub fn as_str(&self) -> String {
        prefix_hex::encode(&self.0).as_str().to_string()
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Secp256K1Signature {
    #[serde(with = "BigArray")]
    arr: [u8; SECP256K1_SIGNATURE_LENGTH],
}

impl Secp256K1Signature {
    pub fn new() -> Self {
        Self { arr: [0; SECP256K1_SIGNATURE_LENGTH] }
    }

    pub fn from_hex(str: String) -> Self {
        Self::from_slice(decode_hex_to_vec(str).as_slice())
    }


    pub fn from_slice(slice: &[u8]) -> Self {
        let mut arr = [0u8; SECP256K1_SIGNATURE_LENGTH];
        arr.copy_from_slice(slice);
        Self { arr }
    }


    pub fn as_slice(&self) -> &[u8] {
        &self.arr
    }

    pub fn unwrap(&self) -> [u8; SECP256K1_SIGNATURE_LENGTH] {
        self.arr
    }
}

pub fn decode_hex_to_vec(str: String) -> Vec<u8> {
    return prefix_hex::decode::<Vec<u8>>(&*str).unwrap();
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum TokenType {
    Native,
    FT,
    NFT,
}
