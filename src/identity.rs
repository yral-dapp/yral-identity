use std::ops::{Deref, DerefMut};

use ic_agent::Identity;
use ic_types::messages::SignedDelegation as IcSignedDelegation;

use crate::{glue::into_ic_signed_delegation, msg_builder::Message, Error, Result, Signature};

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
    pub fn sign_message(&self, mut msg: Message) -> Result<Signature> {
        msg.sender = self.inner.sender().map_err(|_| Error::SenderNotFound)?;
        let sig_agent = self.inner.sign(&msg.into()).map_err(Error::Signing)?;
        Ok(Signature {
            sig: sig_agent.signature,
            public_key: sig_agent.public_key,
            delegations: self.delegations.clone(),
        })
    }
}

impl<I: Identity> Deref for WrappedIdentity<I> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<I: Identity> DerefMut for WrappedIdentity<I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
