use near_sdk::{AccountId, env};
use crate::{RecoveryID, Secp256K1Signature, SignerPublicKey};

use crate::types::Hash;

pub trait Data {
    fn get_data(&self) -> Vec<u8>;
}

#[derive(Clone)]
pub struct ContentNode {
    // Hash of deposit tx info. See spec in core for more information.
    pub origin: Hash,
    pub chain: String,
    pub receiver: Option<AccountId>,
    pub bridge: AccountId,
    pub data: Vec<u8>,
}

impl ContentNode {
    pub fn new(origin: Hash, bridge: AccountId, chain: String, data: Vec<u8>, receiver: Option<AccountId>) -> Self {
        ContentNode {
            origin,
            receiver,
            chain,
            bridge,
            data,
        }
    }

    pub fn hash(self) -> Vec<u8> {
        let mut data = Vec::new();
        data.append(&mut Vec::from(self.data));
        data.append(&mut Vec::from(self.origin.to_slice()));
        data.append(&mut Vec::from(self.chain.as_bytes()));
        if let Some(receiver) = self.receiver.clone() {
            data.append(&mut Vec::from(receiver.as_bytes()));
        }
        data.append(&mut Vec::from(self.bridge.as_bytes()));
        env::keccak256(data.as_slice())
    }
}

pub fn get_merkle_root(content: ContentNode, path: &Vec<Hash>) -> Hash {
    let mut hash = content.hash();

    for i in 0..path.len() {
        let leaf = Hash::from_slice(path[i].to_slice());

        if leaf >= Hash::from_slice(hash.clone().as_slice()) {
            hash = env::keccak256([leaf.unwrap().as_ref(), hash.as_ref()].concat().as_slice());
        } else {
            hash = env::keccak256([hash.as_ref(), leaf.unwrap().as_ref()].concat().as_slice());
        }
    }

    return Hash::from_slice(hash.as_slice());
}

pub fn verify_ecdsa_signature(signer: SignerPublicKey, msg: Hash, signature: Secp256K1Signature, recovery_id: RecoveryID) {
    let public_key = env::ecrecover(msg.to_slice(), signature.as_slice(), recovery_id.unwrap(), true);
    if public_key.is_none() {
        env::panic_str("Signer: invalid signature, public recovered key is none");
    }

    assert_eq!(signer, SignerPublicKey::new(bs58::encode(public_key.unwrap()).into_string()), "Signer: invalid signature, public recovered key is not equal to signer public key");
}

