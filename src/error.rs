use ic_types::messages::HttpRequestError;
use ic_validator_ingress_message::RequestValidationError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("candid ser/de error {0}")]
    Candid(#[from] candid::Error),
    #[error("signature missing")]
    MissingSignature,
    #[error("signing failed {0}")]
    Signing(String),
    #[error("signature verification failed {0}")]
    SignatureVerification(RequestValidationError),
    #[error("invalid message {0}")]
    InvalidMessage(#[from] HttpRequestError),
    #[error("signature does not match identity")]
    IdentityMismatch,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
