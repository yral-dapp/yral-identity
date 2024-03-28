use ic_agent::agent::EnvelopeContent;
use ic_types::messages::HttpCanisterUpdate;
use web_time::Duration;

use candid::{encode_args, utils::ArgumentEncoder, Principal};

use crate::{current_epoch, Result};

#[derive(Clone, Debug)]
pub struct Message {
    canister_id: Principal,
    method_name: String,
    args: Vec<u8>,
    sender: Principal,
    ingress_expiry: Duration,
    nonce: Option<Vec<u8>>,
}

impl Message {
    pub fn canister_id(mut self, canister_id: Principal) -> Self {
        self.canister_id = canister_id;
        self
    }

    pub fn method_name(mut self, method_name: String) -> Self {
        self.method_name = method_name;
        self
    }

    pub fn args<Args: ArgumentEncoder>(mut self, args: Args) -> Result<Self> {
        self.args = encode_args(args)?;
        Ok(self)
    }

    pub fn ingress_expiry(mut self, expiry: Duration) -> Self {
        self.ingress_expiry = expiry;
        self
    }

    pub fn nonce(mut self, nonce: Vec<u8>) -> Self {
        self.nonce = Some(nonce);
        self
    }
}

impl Default for Message {
    fn default() -> Self {
        Self {
            canister_id: Principal::anonymous(),
            method_name: String::new(),
            args: vec![],
            nonce: None,
            sender: Principal::anonymous(),
            ingress_expiry: current_epoch() + Duration::from_secs(120),
        }
    }
}

impl From<Message> for EnvelopeContent {
    fn from(value: Message) -> Self {
        let ingress_expiry_ns = value
            .ingress_expiry
            .as_nanos()
            .try_into()
            .expect("Ingress expiry overflow");
        EnvelopeContent::Call {
            canister_id: value.canister_id,
            method_name: value.method_name,
            arg: value.args,
            sender: value.sender,
            nonce: value.nonce,
            ingress_expiry: ingress_expiry_ns,
        }
    }
}

impl From<Message> for HttpCanisterUpdate {
    fn from(value: Message) -> Self {
        let ingress_expiry_ns = value
            .ingress_expiry
            .as_nanos()
            .try_into()
            .expect("Ingress expiry overflow");
        Self {
            canister_id: value.canister_id.into(),
            method_name: value.method_name,
            arg: value.args.into(),
            sender: value.sender.into(),
            ingress_expiry: ingress_expiry_ns,
            nonce: value.nonce.map(|n| n.into()),
        }
    }
}
