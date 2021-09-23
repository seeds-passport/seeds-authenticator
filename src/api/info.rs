use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};
use rocket::response::{content, status, Responder, Response};
use rocket::request::Request;
use rocket::State;
use crate::{
    utils::{
        errors::AuthenticatorErrors,
        throttling,
        blockchain::get_account,
        signature::{Policy, sign, Signature, hash_token},
        validate::{
            validate_token_and_fetch_from_blockchain,
            verify_credentials,
            CheckRequest
        }
    },
    database::{self, AuthenticationEntry, Database, get_authentication_entry}
};
use base64::{encode};
use std::io::Cursor;
use rocket::http::Status;
use std::net::{SocketAddr, IpAddr};
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};

#[post("/<id>", format = "json", data = "<check_request>")]
async fn info(db: Database, check_request: Json<CheckRequest>, id: &str) -> status::Custom<Value> {
    match get_authentication_entry(db.clone(), &id.to_string(), &check_request.token) {
        Ok(_) => {
            db.authentication_entries.remove(id).unwrap();
            return status::Custom(
                Status::Accepted,
                json!({ "message": {"status": "ok"} }));
        }
        Err(error) => {
            return status::Custom(
                error.status_code(),
                json!({ "message": error.get_error() }));
        }
    }
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("JSON", |rocket| async {
        rocket.mount("/info", routes![info])
    })
}
