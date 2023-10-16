use serde_derive::{Deserialize, Serialize};
use crate::events_deposit::{FtDepositedData, NativeDepositedData, NftDepositedData};
use crate::events_withdraw::{FtWithdrawnData, NativeWithdrawnData, NftWithdrawnData};

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
    NftWithdrawn(Vec<NftWithdrawnData<'a>>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
pub enum Nep141EventKind<'a> {
    #[serde(borrow)]
    FtDeposited(Vec<FtDepositedData<'a>>),
    FtWithdrawn(Vec<FtWithdrawnData<'a>>),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event", content = "data")]
#[serde(rename_all = "snake_case")]
#[allow(clippy::enum_variant_names)]
pub enum NativeEventKind<'a> {
    #[serde(borrow)]
    NativeDeposited(Vec<NativeDepositedData<'a>>),
    NativeWithdrawn(Vec<NativeWithdrawnData<'a>>),
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
    pub fn nft_withdrawn(data: Vec<NftWithdrawnData<'a>>) -> Self {
        NearEvent::new_171_v1(Nep171EventKind::NftWithdrawn(data))
    }

    #[must_use = "don't forget to .emit() the event"]
    pub fn ft_deposited(data: Vec<FtDepositedData<'a>>) -> Self {
        NearEvent::new_141_v1(Nep141EventKind::FtDeposited(data))
    }

    #[must_use = "don't forget to .emit() the event"]
    pub fn ft_withdrawn(data: Vec<FtWithdrawnData<'a>>) -> Self {
        NearEvent::new_141_v1(Nep141EventKind::FtWithdrawn(data))
    }

    #[must_use = "don't forget to .emit() the event"]
    pub fn native_deposited(data: Vec<NativeDepositedData<'a>>) -> Self {
        NearEvent::new_native_v1(NativeEventKind::NativeDeposited(data))
    }

    #[must_use = "don't forget to .emit() the event"]
    pub fn native_withdrawn(data: Vec<NativeWithdrawnData<'a>>) -> Self {
        NearEvent::new_native_v1(NativeEventKind::NativeWithdrawn(data))
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
