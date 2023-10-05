use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::env;

use crate::types::Hash;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Hashes {
    pub hashes: UnorderedMap<String, bool>,
}

impl Hashes {
    pub fn new() -> Self {
        Self {
            hashes: UnorderedMap::new(b"hashes".to_vec()),
        }
    }

    pub fn check_hash(&mut self, hash: Hash) {
        if self.hashes.get(&hash.as_str()) == Some(true) {
            env::panic_str("Hashes: hash already exists");
        }
        self.hashes.insert(&hash.as_str(), &true);
    }

    pub fn change_hash(&mut self, hash: Hash, value: bool) {
        self.hashes.insert(&hash.as_str(), &value);
    }
}
