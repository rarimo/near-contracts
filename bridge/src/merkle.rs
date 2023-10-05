use shared::{Data, to_32_bytes, u128_to_bytes, usize_to_bytes, vector_from_32_bytes};
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::{AccountId, Balance};
use near_sdk::json_types::U128;

pub struct TransferFullMetaOperation {
    // Empty line if is native
    pub token: Option<AccountId>,
    // Empty line if is native or fungible
    pub token_id_to: Option<TokenId>,
    pub amount: Option<U128>,
    pub title: Option<String>,
    pub media_url: Option<String>,
    pub media_hash: Option<String>,
}

impl TransferFullMetaOperation {
    pub fn new_ft_transfer(token: AccountId, amount: U128) -> Self {
        TransferFullMetaOperation {
            token: Some(token),
            token_id_to: None,
            amount: Some(amount),
            title: None,
            media_url: None,
            media_hash: None,
        }
    }

    pub fn new_nft_transfer(token_id: TokenId, token: Option<AccountId>, title: String, media_url: String, media_hash: String) -> Self {
        TransferFullMetaOperation {
            token,
            token_id_to: Some(token_id),
            title: Some(title),
            amount: None,
            media_url: Some(media_url),
            media_hash: Some(media_hash),
        }
    }
}

impl Data for TransferFullMetaOperation {
    fn get_data(&self) -> Vec<u8> {
        let mut data = Vec::new();
        let zero_length = vector_from_32_bytes(usize_to_bytes(0usize));

        if let Some(token) = self.token.clone() {
            let token_bytes: &mut Vec<u8> = &mut prefix_hex::decode(&*prefix_hex::encode(token.as_bytes())).unwrap();
            data.append(&mut vector_from_32_bytes(usize_to_bytes(token_bytes.clone().len())));
            data.append(token_bytes);
        } else {
            data.append(&mut zero_length.clone());
        }

        if let Some(title) = self.title.clone() {
            let title_bytes = &mut Vec::from(title.clone().into_bytes());
            data.append(&mut vector_from_32_bytes(usize_to_bytes(title_bytes.clone().len())));
            data.append(title_bytes);
        } else {
            data.append(&mut zero_length.clone());
        }

        if let Some(token_id_to) = self.token_id_to.clone() {
            let token_id_to_bytes: &mut Vec<u8> = &mut prefix_hex::decode(&*prefix_hex::encode(to_32_bytes(token_id_to.as_bytes()))).unwrap();
            data.append(&mut vector_from_32_bytes(usize_to_bytes(token_id_to_bytes.clone().len())));
            data.append(token_id_to_bytes);
        } else {
            data.append(&mut zero_length.clone());
        }

        if let Some(amount) = self.amount.clone() {
            let mut amount_bytes = vector_from_32_bytes(u128_to_bytes(amount.clone()));
            data.append(&mut vector_from_32_bytes(usize_to_bytes(amount_bytes.clone().len())));
            data.append(&mut amount_bytes);
        } else {
            data.append(&mut zero_length.clone());
        }

        if let Some(media_url) = self.media_url.clone() {
            let media_url_bytes = &mut Vec::from(media_url.into_bytes());
            data.append(&mut vector_from_32_bytes(usize_to_bytes(media_url_bytes.clone().len())));
            data.append(media_url_bytes);
        } else {
            data.append(&mut zero_length.clone());
        }

        if let Some(media_hash) = self.media_hash.clone() {
            let decoded_base64 = base64::decode(media_hash.as_str()).unwrap();
            let media_hash_bytes: &mut Vec<u8> = &mut prefix_hex::decode(&*prefix_hex::encode(decoded_base64.clone())).unwrap();
            data.append(&mut vector_from_32_bytes(usize_to_bytes(media_hash_bytes.clone().len())));
            data.append(media_hash_bytes);
        } else {
            data.append(&mut zero_length.clone());
        }

        data
    }
}

pub struct TransferOperation {
    pub amount: Balance,
}

impl TransferOperation {
    pub fn new_native_transfer(amount: Balance) -> Self {
        Self {
            amount,
        }
    }
}

impl Data for TransferOperation {
    fn get_data(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.append(&mut vector_from_32_bytes(u128_to_bytes(U128::from(self.amount.clone()))));
        data
    }
}
