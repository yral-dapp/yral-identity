use web_time::Duration;

use candid::{encode_args, utils::ArgumentEncoder, Principal};

use crate::{current_epoch, Result};

#[derive(Clone, Debug)]
pub struct Message {
    pub(crate) canister_id: Principal,
    pub(crate) method_name: String,
    pub(crate) args: Vec<u8>,
    #[allow(dead_code)]
    pub(crate) sender: Principal,
    pub(crate) ingress_expiry: Duration,
    pub(crate) nonce: Option<Vec<u8>>,
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
