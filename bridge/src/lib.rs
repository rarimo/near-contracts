extern crate core;

use std::str::FromStr;

use near_contract_standards::non_fungible_token::{Token, TokenId};
use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
use near_contract_tools::{Pause, pause::Pause};
use near_contract_tools::pause::PauseExternal;
use near_sdk::{AccountId, assert_one_yocto, Balance, env, near_bindgen, PanicOnDefault, Promise, PromiseError, PromiseOrValue, require};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use serde::{Deserialize, Serialize};

use shared::{CALL_GAS, ContentNode, Data, FT_MINT_STORAGE_DEPOSIT, GAS_FOR_TX, get_merkle_root, Hash, Hashes, NFT_MINT_STORAGE_DEPOSIT, NO_ARGS, RecoveryID, Secp256K1Signature, SignerPublicKey, u128_to_bytes, verify_ecdsa_signature};

use crate::events::*;
use crate::external::*;
use crate::merkle::*;
use crate::nft::*;

mod events;
mod external;
mod receivers;
mod merkle;
mod nft;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault, Pause)]
pub struct Bridge {
    pub signer: SignerPublicKey,
    pub hashes: Hashes,
    pub chain: String,
    pub fee_contract: AccountId,
    pub nonce: u128,
}

#[derive(Deserialize, Serialize, Debug)]
struct ContractUpdateArgs {
    code: String,
    signature: String,
    recovery_id: RecoveryID,
}

#[near_bindgen]
impl Bridge {
    #[init]
    pub fn new(signer: SignerPublicKey, fee_contract: AccountId, chain: String) -> Self {
        assert_ne!(chain, "", "Chain argument is required");

        if !signer.validate() {
            env::panic_str("Invalid signer public key");
        }

        Self {
            signer: signer.clone(),
            hashes: Hashes::new(),
            nonce: 0,
            fee_contract,
            chain,
        }
    }

    pub fn get_chain(&self) -> String {
        self.chain.clone()
    }

    pub fn get_fee_contract(&self) -> AccountId {
        self.fee_contract.clone()
    }

    pub fn set_fee_contract(&mut self, fee_contract: AccountId, signature: String, recovery_id: RecoveryID) {
        let sign = Secp256K1Signature::from_hex(signature);
        let hash = self.get_authorized_operation_hash(fee_contract.clone().to_string());

        self.check_signature_and_increment_nonce(hash, sign.clone(), recovery_id.clone());

        self.fee_contract = fee_contract;
    }

    pub fn update_contract(&mut self) -> Promise {
        // Receive the arguments directly from the input to avoid the
        // GAS overhead of deserializing parameters
        let input = env::input().expect("Error: No input").to_vec();
        let args: ContractUpdateArgs = serde_json::from_slice(&input).unwrap();
        let code = base64::decode(&args.code).expect("Error: Invalid base64 string").to_vec();
        let sign = Secp256K1Signature::from_hex(args.signature);
        let hash = self.get_update_contract_hash(code.clone());

        self.check_signature_and_increment_nonce(hash, sign.clone(), args.recovery_id.clone());

        // Deploy the contract on self
        Promise::new(env::current_account_id())
            .deploy_contract(code)
            .function_call(
                "migrate".to_string(),
                NO_ARGS,
                0,
                CALL_GAS,
            )
            .as_return()
    }

    pub fn get_signer(&self) -> SignerPublicKey {
        self.signer.clone()
    }

    #[payable]
    pub fn set_signer(&mut self, signer: SignerPublicKey, signature: String, recovery_id: RecoveryID) {
        let sign = Secp256K1Signature::from_hex(signature);

        if !signer.validate() {
            env::panic_str("Invalid signer public key");
        }

        let hash = self.get_set_signer_hash(signer.to_string().clone());

        self.check_signature_and_increment_nonce(hash, sign.clone(), recovery_id.clone());
        self.signer = signer;
    }

    pub fn pause_bridge(&mut self, signature: String, recovery_id: RecoveryID) {
        let sign = Secp256K1Signature::from_hex(signature);
        let hash = self.get_authorized_operation_hash("pause".to_string());

        self.check_signature_and_increment_nonce(hash, sign.clone(), recovery_id.clone());

        Bridge::pause(self);
    }

    pub fn resume_bridge(&mut self, signature: String, recovery_id: RecoveryID) {
        let sign = Secp256K1Signature::from_hex(signature);
        let hash = self.get_authorized_operation_hash("resume".to_string());

        self.check_signature_and_increment_nonce(hash, sign.clone(), recovery_id.clone());

        Bridge::unpause(self);
    }

    #[payable]
    pub fn nft_withdraw(
        &mut self,
        token: AccountId,
        token_id: TokenId,
        receiver_id: AccountId,
        token_metadata: Option<NftMetadata>,
        is_wrapped: bool,
        origin: String,
        path: Vec<Hash>,
        signature: String,
        recovery_id: RecoveryID,
    ) -> PromiseOrValue<bool> {
        Bridge::require_unpaused();

        if is_wrapped {
            require!(
                env::attached_deposit() == NFT_MINT_STORAGE_DEPOSIT,
                "Requires attached deposit of 0.2 NEAR",
            );

            let _token_metadata = token_metadata.clone().unwrap();

            let metadata = TokenMetadata{
                title: _token_metadata.clone().title,
                description: _token_metadata.clone().description,
                media: _token_metadata.clone().media,
                media_hash: _token_metadata.clone().media_hash,
                copies: Some(1),
                issued_at: None,
                expires_at: None,
                starts_at: None,
                updated_at: None,
                extra: None,
                reference: None,
                reference_hash: None,
            };

            self.internal_nft_withdraw(
                token.clone(),
                token_id.clone(),
                receiver_id.clone(),
                metadata,
                is_wrapped.clone(),
                origin.clone(),
                path.clone(),
                signature.clone(),
                recovery_id.clone(),
            )
        } else {
            assert_one_yocto();
            let promise = ext_non_fungible_token::ext(token.clone())
                .with_static_gas(GAS_FOR_TX)
                .nft_token(token_id.clone())
                .then(
                    Self::ext(env::current_account_id())
                        .with_static_gas(GAS_FOR_TX)
                        .with_attached_deposit(env::attached_deposit())
                        .nft_get_callback(
                            token.clone(),
                            token_id.clone(),
                            receiver_id.clone(),
                            is_wrapped.clone(),
                            origin.clone(),
                            path.clone(),
                            signature.clone(),
                            recovery_id.clone(),
                        )
                );

            PromiseOrValue::from(promise)
        }
    }

    #[payable]
    pub fn ft_withdraw(
        &mut self,
        token: AccountId,
        amount: U128,
        receiver_id: AccountId,
        is_wrapped: bool,
        origin: String,
        path: Vec<Hash>,
        signature: String,
        recovery_id: RecoveryID,
    ) -> PromiseOrValue<U128> {
        Bridge::require_unpaused();

        if is_wrapped {
            require!(
                env::attached_deposit() == FT_MINT_STORAGE_DEPOSIT,
                "Requires attached deposit of 0.00125 NEAR",
            );
        } else {
            assert_one_yocto();
        }

        let sign = Secp256K1Signature::from_hex(signature);
        let origin_hash = Hash::from_hex(origin.clone());
        let content = ContentNode::new(
            origin_hash,
            env::current_account_id(),
            self.chain.clone(),
            TransferFullMetaOperation::new_ft_transfer(token.clone(), amount).get_data(),
            Some(receiver_id.clone()),
        );

        self.check_signature(get_merkle_root(content, &path), sign, recovery_id);
        self.hashes.check_hash(origin_hash.clone());

        if is_wrapped {
            let promise = ext_fungible_token::ext(token.clone())
                .with_static_gas(GAS_FOR_TX)
                .with_attached_deposit(env::attached_deposit())
                .ft_mint(receiver_id.clone(), amount)
                .then(
                    Self::ext(env::current_account_id())
                        .with_static_gas(GAS_FOR_TX)
                        .handle_hash_callback(origin_hash.clone())
                );

            PromiseOrValue::from(promise)
        } else {
            let promise = ext_fungible_token::ext(token.clone())
                .with_static_gas(GAS_FOR_TX)
                .with_attached_deposit(env::attached_deposit())
                .ft_transfer(receiver_id.clone(), amount.clone(), None)
                .then(
                    Self::ext(env::current_account_id())
                        .with_static_gas(GAS_FOR_TX)
                        .handle_hash_callback(origin_hash.clone())
                );

            PromiseOrValue::from(promise)
        }
    }

    #[payable]
    pub fn native_deposit(
        &mut self,
        sender: AccountId,
        receiver_id: String,
        chain: String,
        bundle_data: Option<String>,
        bundle_salt: Option<String>,
    ) {
        Bridge::require_unpaused();

        assert_eq!(env::predecessor_account_id(), self.fee_contract.clone(), "Sender must be fee contract");

        if env::attached_deposit() == 0 {
            env::panic_str("Attached deposit must be greater than zero");
        }

        NearEvent::native_deposited(vec![NativeDepositedData::new(
            env::attached_deposit().to_string().as_str(),
            &sender.clone(),
            &receiver_id.clone(),
            chain.as_str(),
            bundle_data.clone(),
            bundle_salt.clone(),
        )]).emit();
    }

    #[payable]
    pub fn native_withdraw(
        &mut self,
        receiver_id: AccountId,
        amount: String,
        origin: String,
        path: Vec<Hash>,
        signature: String,
        recovery_id: RecoveryID,
    ) {
        Bridge::require_unpaused();

        assert_one_yocto();

        let amnt = Balance::from_str(amount.as_str()).unwrap();
        let sign = Secp256K1Signature::from_hex(signature);
        let origin_hash = Hash::from_hex(origin.clone());

        let content = ContentNode::new(
            origin_hash,
            env::current_account_id(),
            self.chain.clone(),
            TransferOperation::new_native_transfer(
                amnt,
            ).get_data(),
            Some(receiver_id.clone()),
        );

        self.check_signature(get_merkle_root(content, &path), sign, recovery_id);
        self.hashes.check_hash(origin_hash.clone());

        Promise::new(receiver_id)
            .transfer(amnt)
            .then(Self::ext(env::current_account_id())
                .with_static_gas(GAS_FOR_TX)
                .handle_hash_callback(origin_hash.clone())
            );
    }

    #[init(ignore_state)]
    #[private]
    pub fn migrate() -> Self {
        let contract: Bridge = env::state_read().expect("ERROR, NOT INITIALIZED");
        contract
    }

    #[private]
    fn internal_nft_transfer(&self, token: AccountId, token_id: TokenId, receiver_id: AccountId, deposit: Balance) -> Promise {
        ext_non_fungible_token::ext(token.clone())
            .with_static_gas(GAS_FOR_TX)
            .with_attached_deposit(deposit)
            .nft_transfer(receiver_id, token_id, None, None)
    }

    #[private]
    fn internal_ft_transfer(&self, token: AccountId, amount: U128, receiver_id: AccountId, deposit: Balance) -> PromiseOrValue<U128> {
        let promise = ext_fungible_token::ext(token.clone())
            .with_static_gas(GAS_FOR_TX)
            .with_attached_deposit(deposit)
            .ft_transfer(receiver_id, amount, None);
        PromiseOrValue::from(promise)
    }

    #[private]
    pub fn get_authorized_operation_hash(&mut self, value: String) -> Hash {
        let mut data = Vec::new();
        let contract_data = self.get_contract_based_hash_data();

        data.append(&mut Vec::from(value.into_bytes()));
        data.append(&mut Vec::from(contract_data));

        Hash::from_slice(env::keccak256(data.as_slice()).as_slice())
    }

    #[private]
    fn get_update_contract_hash(&mut self, code: Vec<u8>) -> Hash {
        let mut data = Vec::new();
        let contract_data = self.get_contract_based_hash_data();

        data.append(&mut Vec::from(env::keccak256(code.as_slice()).as_slice()));
        data.append(&mut Vec::from(contract_data));

        Hash::from_slice(env::keccak256(data.as_slice()).as_slice())
    }

    #[private]
    fn get_set_signer_hash(&mut self, signer_public_key: String) -> Hash {
        Hash::from_slice(env::keccak256(Vec::from(signer_public_key.into_bytes()).as_slice()).as_slice())
    }

    #[private]
    fn get_contract_based_hash_data(&mut self) -> Vec<u8> {
        let mut data = Vec::new();
        data.append(&mut Vec::from(self.chain.to_string().into_bytes()));
        data.append(&mut Vec::from(u128_to_bytes(U128::from(self.nonce.clone())).as_slice()));
        data.append(&mut Vec::from(env::current_account_id().to_string().into_bytes()));
        data
    }

    #[private]
    pub fn handle_hash_callback(&mut self, #[callback_result] call_result: Result<(), PromiseError>, hash: Hash) {
        if call_result.is_err() {
            self.hashes.change_hash(hash, false);
        }
    }

    #[private]
    pub fn check_signature(&mut self, msg: Hash, signature: Secp256K1Signature, recovery_id: RecoveryID) {
        verify_ecdsa_signature(self.signer.clone(), msg, signature, recovery_id);
    }

    #[private]
    pub fn check_signature_and_increment_nonce(&mut self, msg: Hash, signature: Secp256K1Signature, recovery_id: RecoveryID) {
        self.check_signature(msg, signature, recovery_id);
        self.nonce += 1;
    }

    #[private]
    #[payable]
    pub fn nft_get_callback(
        &mut self,
        #[callback_result] call_result: Result<Option<Token>, PromiseError>,
        token: AccountId,
        token_id: TokenId,
        receiver_id: AccountId,
        is_wrapped: bool,
        origin: String,
        path: Vec<Hash>,
        signature: String,
        recovery_id: RecoveryID,
    ) -> PromiseOrValue<bool> {
        if call_result.is_err() {
            env::panic_str("failed to get non fungible token metadata");
        }

        let result = call_result.unwrap();
        if result.is_none() {
            env::panic_str("non fungible token not found");
        }

        let metadata = result.unwrap().metadata;
        if metadata.is_none() {
            env::panic_str("non fungible token metadata not found");
        }

        metadata.clone().unwrap().assert_valid();

        self.internal_nft_withdraw(
            token.clone(),
            token_id.clone(),
            receiver_id.clone(),
            metadata.clone().unwrap(),
            is_wrapped.clone(),
            origin.clone(),
            path.clone(),
            signature.clone(),
            recovery_id.clone(),
        )
    }

    #[private]
    #[payable]
    pub fn internal_nft_withdraw(
        &mut self,
        token: AccountId,
        token_id: TokenId,
        receiver_id: AccountId,
        token_metadata: TokenMetadata,
        is_wrapped: bool,
        origin: String,
        path: Vec<Hash>,
        signature: String,
        recovery_id: RecoveryID,
    ) -> PromiseOrValue<bool> {
        let sign = Secp256K1Signature::from_hex(signature);
        let origin_hash = Hash::from_hex(origin.clone());

        let content = ContentNode::new(
            origin_hash,
            env::current_account_id(),
            self.chain.clone(),
            TransferFullMetaOperation::new_nft_transfer(
                token_id.clone(),
                Some(token.clone()),
                token_metadata.title.as_ref().unwrap().to_string(),
                token_metadata.media.as_ref().unwrap().to_string(),
                base64::encode(token_metadata.media_hash.clone().unwrap().0),
            ).get_data(),
            Some(receiver_id.clone()),
        );

        self.check_signature(get_merkle_root(content, &path), sign, recovery_id);
        self.hashes.check_hash(origin_hash.clone());

        if is_wrapped {
            let promise = ext_non_fungible_token::ext(token.clone())
                .with_static_gas(GAS_FOR_TX)
                .with_attached_deposit(env::attached_deposit())
                .nft_mint(token_id, receiver_id, token_metadata.clone(), None)
                .then(
                    Self::ext(env::current_account_id())
                        .with_static_gas(GAS_FOR_TX)
                        .handle_hash_callback(origin_hash.clone())
                );

            PromiseOrValue::from(promise)
        } else {
            let promise = self.internal_nft_transfer(
                token,
                token_id,
                receiver_id,
                env::attached_deposit(),
            ).then(
                Self::ext(env::current_account_id())
                    .with_static_gas(GAS_FOR_TX)
                    .handle_hash_callback(origin_hash.clone())
            );

            PromiseOrValue::from(promise)
        }
    }
}

