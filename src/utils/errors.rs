use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use std::fmt;

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

impl AuthenticatorErrors {
    fn error_response(&self) -> String {
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

impl error::ResponseError for AuthenticatorErrors {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthenticatorErrors::AccountNotFound => StatusCode::NOT_FOUND,
            AuthenticatorErrors::InvalidId => StatusCode::NOT_FOUND,
            AuthenticatorErrors::InvalidToken => StatusCode::FORBIDDEN,
            AuthenticatorErrors::BlockchainError => StatusCode::SERVICE_UNAVAILABLE,
            AuthenticatorErrors::NotStoredBlockchain => StatusCode::NOT_FOUND,
            AuthenticatorErrors::MismatchedPolicies => StatusCode::FORBIDDEN,
            AuthenticatorErrors::ExpiredPolicy => StatusCode::FORBIDDEN,
            AuthenticatorErrors::InvalidSignature => StatusCode::FORBIDDEN,
            AuthenticatorErrors::TooManyUserAccesses => StatusCode::TOO_MANY_REQUESTS,
            AuthenticatorErrors::InvalidAccountName => StatusCode::FORBIDDEN,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl fmt::Display for AuthenticatorErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}