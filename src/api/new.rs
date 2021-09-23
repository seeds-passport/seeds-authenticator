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
    },
    database::{self, AuthenticationEntry, Database}
};
use base64::{encode};
use std::io::Cursor;
use rocket::http::Status;
use std::net::{SocketAddr, IpAddr};
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct AuthenticationRequest {
    account_name: String
}
#[derive(Serialize, Deserialize)]
struct NewAuthenticationDataSet {
    authentication_entry: AuthenticationEntry,
    policy: Policy,
    token: String,
    signature: Signature
}
#[derive(Serialize, Deserialize)]
pub struct AnswerNew {
    id: String,
    account_name: String,
    token: String,
    valid_until: String,
    policy: String,
    signature: String,
}

#[post("/", format = "json", data = "<authentication_request>")]
async fn new(db: Database, authentication_request: Json<AuthenticationRequest>, remote_address: SocketAddr) -> status::Custom<Value> {
    if authentication_request.account_name.len() > 12 {
        return status::Custom(
            AuthenticatorErrors::InvalidAccountName.status_code(),
            json!({ "message": AuthenticatorErrors::InvalidAccountName.get_error() }));
    }
    let ip: IpAddr = remote_address.ip();
    let is_allowed: bool = throttling::permission(&authentication_request.account_name, ip);

    if !is_allowed {
        return status::Custom(
            AuthenticatorErrors::TooManyUserAccesses.status_code(),
            json!({ "message": AuthenticatorErrors::TooManyUserAccesses.get_error() }));
    } else {
        match get_account(&authentication_request.account_name).await {
            Ok(_) => {

                let data = generate_data(&authentication_request.account_name);

                db.waiting_for_confirmation
                    .insert(data.policy.id.as_bytes(), data.authentication_entry).unwrap();

                let answer = AnswerNew {
                    id: data.policy.id,
                    account_name: data.policy.account_name,
                    token: data.token,
                    valid_until: data.policy.valid_until,
                    policy: data.signature.base64_policy,
                    signature: data.signature.signature
                };
                return status::Custom(
                    Status::Accepted,
                    json!({ "message": answer }));
            },
            Err(_) => {
                return status::Custom(
                    AuthenticatorErrors::AccountNotFound.status_code(),
                    json!({ "message": AuthenticatorErrors::AccountNotFound.get_error() }));
            }
        }
    }
}

fn generate_data(account_name: &String) -> NewAuthenticationDataSet {
    let id = Uuid::new_v4();
    let secret = Uuid::new_v4();

    let start = SystemTime::now();
    let valid_until: u64 =
        start.duration_since(UNIX_EPOCH).unwrap().as_secs() + (86400 * 30);

    let policy = Policy {
        valid_until: format!("{}", valid_until),
        account_name: account_name.to_string(),
        id: id.to_string(),
    };

    let token = Uuid::new_v4().to_string();
    let b64_policy = encode(serde_json::to_string(&policy).unwrap());
    let signature = sign(&b64_policy, &secret.to_string(), &token);

    let new_authentication_entry = AuthenticationEntry {
        id: id.clone(),
        account_name: policy.account_name.clone(),
        secret: secret,
        valid_until: valid_until,
        policy_base64: signature.base64_policy.clone(),
        blockchain_index: None,
        token_hash: hash_token(&token)
    };

    NewAuthenticationDataSet {
        authentication_entry: new_authentication_entry,
        policy: policy,
        token: token,
        signature: signature
    }
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("JSON", |rocket| async {
        rocket.mount("/new", routes![new])
    })
}
