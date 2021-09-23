use serde::Serialize;
use std::fmt;
use rocket::Response;
use rocket::http::Status;
use rocket::response::Body;

#[derive(Debug, Serialize)]
pub enum AuthenticatorErrors {
    AccountNotFound,
    InvalidId,
    InvalidToken,
    BlockchainError,
    NotStoredBlockchain,
    MismatchedPolicies,
    ExpiredPolicy,
    InvalidSignature,
    TooManyUserAccesses,
    InvalidAccountName
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}

impl std::error::Error for AuthenticatorErrors {}
impl std::io::Read for MyErrorResponse {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        todo!()
    }
}

impl AuthenticatorErrors {
    pub fn status_code(&self) -> Status {
        match self {
            AuthenticatorErrors::AccountNotFound => Status::NotFound,
            AuthenticatorErrors::InvalidId => Status::NotFound,
            AuthenticatorErrors::InvalidToken => Status::Forbidden,
            AuthenticatorErrors::BlockchainError => Status::ServiceUnavailable,
            AuthenticatorErrors::NotStoredBlockchain => Status::NotFound,
            AuthenticatorErrors::MismatchedPolicies => Status::Forbidden,
            AuthenticatorErrors::ExpiredPolicy => Status::Forbidden,
            AuthenticatorErrors::InvalidSignature => Status::Forbidden,
            AuthenticatorErrors::TooManyUserAccesses => Status::TooManyRequests,
            AuthenticatorErrors::InvalidAccountName => Status::Forbidden,
        }
    }
    pub fn get_error(&self) -> String {
        match self {
            AuthenticatorErrors::AccountNotFound => {
                "Account not found.".into()
            }
            AuthenticatorErrors::InvalidId => {
                "Invalid id.".into()
            }
            AuthenticatorErrors::InvalidToken => {
                "Invalid token.".into()
            }
            AuthenticatorErrors::BlockchainError => {
                "Error accessing the blockchain.".into()
            },
            AuthenticatorErrors::NotStoredBlockchain => {
                "Authentication entry not found on blockchain.".into()
            }
            AuthenticatorErrors::MismatchedPolicies => {
                "The policy stored on the blockchain doesn't match the policy stored on the authenticator.".into()
            }
            AuthenticatorErrors::ExpiredPolicy => {
                "The policy expired.".into()
            }
            AuthenticatorErrors::InvalidSignature => {
                "The signatures didn't match.".into()
            }
            AuthenticatorErrors::TooManyUserAccesses => {
                "Too many requests. Try again soon.".into()
            }
            AuthenticatorErrors::InvalidAccountName => {
                "Your account name is invalid.".into()
            }
        }
    }
}
impl fmt::Display for AuthenticatorErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}