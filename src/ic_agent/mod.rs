use ic_agent::agent::EnvelopeContent;

use crate::{msg_builder::Message, Delegation, Signature, SignedDelegation};

impl From<ic_agent::Signature> for Signature {
    fn from(value: ic_agent::Signature) -> Self {
        Self {
            sig: value.signature,
            public_key: value.public_key,
            delegations: value
                .delegations
                .map(|v| v.into_iter().map(Into::into).collect()),
        }
    }
}

impl From<Delegation> for ic_agent::identity::Delegation {
    fn from(value: Delegation) -> Self {
        Self {
            pubkey: value.pubkey,
            expiration: value.expiration_ns,
            targets: value.targets,
        }
    }
}

impl From<SignedDelegation> for ic_agent::identity::SignedDelegation {
    fn from(value: SignedDelegation) -> Self {
        Self {
            delegation: value.delegation.into(),
            signature: value.signature,
        }
    }
}

impl From<ic_agent::identity::Delegation> for Delegation {
    fn from(value: ic_agent::identity::Delegation) -> Self {
        Self {
            pubkey: value.pubkey,
            expiration_ns: value.expiration,
            targets: value.targets,
        }
    }
}

impl From<ic_agent::identity::SignedDelegation> for SignedDelegation {
    fn from(value: ic_agent::identity::SignedDelegation) -> Self {
        Self {
            delegation: value.delegation.into(),
            signature: value.signature,
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
