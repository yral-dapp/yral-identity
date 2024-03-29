mod error;
#[cfg(feature = "ic-agent")]
mod glue;
#[cfg(feature = "ic-agent")]
pub mod identity;
pub mod msg_builder;

use candid::Principal;
pub use error::*;

use ic_types::messages::{
    HttpCallContent, HttpRequest, HttpRequestEnvelope, SignedDelegation as IcSignedDelegation,
    SignedIngressContent,
};
use ic_validator_ingress_message::{HttpRequestVerifier, IngressMessageVerifier};
use msg_builder::Message;
use serde::{Deserialize, Serialize};
use web_time::{Duration, SystemTime};

fn current_epoch() -> Duration {
    web_time::SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
}

/// A signature, interoperable with ic-agent & yral-identity
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub struct Signature {
    sig: Option<Vec<u8>>,
    public_key: Option<Vec<u8>>,
    delegations: Option<Vec<IcSignedDelegation>>,
}

#[cfg(feature = "ic-agent")]
impl From<ic_agent::Signature> for Signature {
    fn from(value: ic_agent::Signature) -> Self {
        Self {
            sig: value.signature,
            public_key: value.public_key,
            delegations: value
                .delegations
                .map(|v| v.into_iter().map(glue::into_ic_signed_delegation).collect()),
        }
    }
}

impl Signature {
    fn create_request(self, message: Message) -> Result<HttpRequest<SignedIngressContent>> {
        let envelope = HttpRequestEnvelope {
            content: HttpCallContent::Call {
                update: message.into(),
            },
            sender_pubkey: self.public_key.map(Into::into),
            sender_sig: self.sig.map(Into::into),
            sender_delegation: self.delegations.map(Into::into),
        };
        Ok(envelope.try_into()?)
    }

    fn verify_request(req: HttpRequest<SignedIngressContent>) -> Result<()> {
        let verifier = IngressMessageVerifier::default();
        verifier
            .validate_request(&req)
            .map_err(Error::SignatureVerification)
    }

    pub fn verify(self, msg: Message) -> Result<()> {
        let req = self.create_request(msg)?;
        Self::verify_request(req)
    }

    pub fn verify_identity(self, principal: Principal, msg: Message) -> Result<()> {
        let req = self.create_request(msg)?;
        if req.sender().get().0 != principal {
            return Err(Error::IdentityMismatch);
        }
        Self::verify_request(req)
    }
}
