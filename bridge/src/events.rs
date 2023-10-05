use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::AccountId;
use near_sdk::json_types::U128;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Serialize, Debug)]
#[serde(tag = "standard")]
#[serde(rename_all = "snake_case")]
pub enum NearEvent<'a> {
    #[serde(borrow)]
    Nep171(Nep171Event<'a>),
    Nep141(Nep141Event<'a>),
    Native(NativeEvent<'a>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Nep171Event<'a> {
    pub version: &'static str,
    #[serde(flatten)]
    #[serde(borrow)]
    pub event_kind: Nep171EventKind<'a>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Nep141Event<'a> {
    pub version: &'static str,
    #[serde(flatten)]
    #[serde(borrow)]
    pub event_kind: Nep141EventKind<'a>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NativeEvent<'a> {
    pub version: &'static str,
    #[serde(flatten)]
    #[serde(borrow)]
    pub event_kind: NativeEventKind<'a>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
pub enum Nep171EventKind<'a> {
    #[serde(borrow)]
    NftDeposited(Vec<NftDepositedData<'a>>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
pub enum Nep141EventKind<'a> {
    #[serde(borrow)]
    FtDeposited(Vec<FtDepositedData<'a>>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
pub enum NativeEventKind<'a> {
    #[serde(borrow)]
    NativeDeposited(Vec<NativeDepositedData<'a>>),
}

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

impl<'a> NearEvent<'a> {
    pub fn new_171(version: &'static str, event_kind: Nep171EventKind<'a>) -> Self {
        NearEvent::Nep171(Nep171Event {
            version,
            event_kind,
        })
    }

    pub fn new_141(version: &'static str, event_kind: Nep141EventKind<'a>) -> Self {
        NearEvent::Nep141(Nep141Event {
            version,
            event_kind,
        })
    }

    pub fn new_native(version: &'static str, event_kind: NativeEventKind<'a>) -> Self {
        NearEvent::Native(NativeEvent {
            version,
            event_kind,
        })
    }

    pub fn new_171_v1(event_kind: Nep171EventKind<'a>) -> Self {
        NearEvent::new_171("1.0.0", event_kind)
    }

    pub fn new_141_v1(event_kind: Nep141EventKind<'a>) -> Self {
        NearEvent::new_141("1.0.0", event_kind)
    }

    pub fn new_native_v1(event_kind: NativeEventKind<'a>) -> Self {
        NearEvent::new_native("1.0.0", event_kind)
    }

    #[must_use = "don't forget to .emit() the event"]
    pub fn nft_deposited(data: Vec<NftDepositedData<'a>>) -> Self {
        NearEvent::new_171_v1(Nep171EventKind::NftDeposited(data))
    }

    #[must_use = "don't forget to .emit() the event"]
    pub fn ft_deposited(data: Vec<FtDepositedData<'a>>) -> Self {
        NearEvent::new_141_v1(Nep141EventKind::FtDeposited(data))
    }

    #[must_use = "don't forget to .emit() the event"]
    pub fn native_deposited(data: Vec<NativeDepositedData<'a>>) -> Self {
        NearEvent::new_native_v1(NativeEventKind::NativeDeposited(data))
    }

    pub(crate) fn to_json_string(&self) -> String {
        near_sdk::serde_json::to_string(self).unwrap()
    }

    pub fn to_json_event_string(&self) -> String {
        format!("EVENT_JSON:{}", self.to_json_string())
    }

    /// Logs the event to the host. This is required to ensure that the event is triggered
    /// and to consume the event.
    pub fn emit(self) {
        near_sdk::env::log_str(&self.to_json_event_string());
    }
}
