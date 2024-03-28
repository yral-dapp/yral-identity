mod error;
mod glue;
pub mod msg_builder;

use candid::Principal;
pub use error::*;

use glue::into_ic_signed_delegation;
use ic_agent::Identity;
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

impl From<ic_agent::Signature> for Signature {
    fn from(value: ic_agent::Signature) -> Self {
        Self {
            sig: value.signature,
            public_key: value.public_key,
            delegations: value
                .delegations
                .map(|v| v.into_iter().map(into_ic_signed_delegation).collect()),
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

/// An identity interoperable with ic-agent & yral-identity
#[derive(Clone)]
pub struct WrappedIdentity<I: Identity> {
    inner: I,
    delegations: Option<Vec<IcSignedDelegation>>,
}

impl<I: Identity> From<I> for WrappedIdentity<I> {
    fn from(value: I) -> Self {
        let delegations: Vec<_> = value
            .delegation_chain()
            .into_iter()
            .map(into_ic_signed_delegation)
            .collect();
        Self {
            inner: value,
            delegations: if delegations.is_empty() {
                None
            } else {
                Some(delegations)
            },
        }
    }
}

impl<I: Identity> WrappedIdentity<I> {
    pub fn sign_message(&self, msg: Message) -> Result<Signature> {
        let sig_agent = self.inner.sign(&msg.into()).map_err(Error::Signing)?;
        Ok(Signature {
            sig: sig_agent.signature,
            public_key: sig_agent.public_key,
            delegations: self.delegations.clone(),
        })
    }
}
