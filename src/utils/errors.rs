use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum AuthenticatorErrors {
    // InternalServerError,
    AccountNotFound,
}
#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}
impl std::error::Error for AuthenticatorErrors {}

impl AuthenticatorErrors {
    fn error_response(&self) -> String {
        match self {
            // AuthenticatorErrors::InternalServerError => {
            //     "Internal Server Error".into()
            // }
            AuthenticatorErrors::AccountNotFound => {
                "Account Not Found".into()
            }
        }
    }
}

impl error::ResponseError for AuthenticatorErrors {
    fn status_code(&self) -> StatusCode {
        match self {
            // AuthenticatorErrors::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AuthenticatorErrors::AccountNotFound => StatusCode::NOT_FOUND,
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