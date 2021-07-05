use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum AuthenticatorErrors {
    AccountNotFound,
    InvalidId,
    InvalidToken
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
                "Account Not Found".into()
            }
            AuthenticatorErrors::InvalidId => {
                "Invalid id".into()
            }
            AuthenticatorErrors::InvalidToken => {
                "Invalid token".into()
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