mod error;
#[cfg(feature = "ic-agent")]
pub mod ic_agent;
#[cfg(feature = "ic-git")]
pub mod ic_git;
pub mod msg_builder;

use candid::Principal;
pub use error::*;

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
    delegations: Option<Vec<SignedDelegation>>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
struct Delegation {
    pub pubkey: Vec<u8>,
    pub expiration_ns: u64,
    pub targets: Option<Vec<Principal>>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
struct SignedDelegation {
    pub delegation: Delegation,
    pub signature: Vec<u8>,
}
