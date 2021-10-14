use rocket::serde::json::{Json, Value};
use rocket::serde::{Serialize, Deserialize};
use rocket::response::status;
use serde_json::json;
use crate::{
    utils::{
        errors::AuthenticatorErrors,
        throttling,
        blockchain::get_account,
        signature::{Policy, sign, Signature, hash_token},
    },
    database::{AuthenticationEntry, Database}
};
use base64::{encode};
use rocket::http::Status;
use std::net::{SocketAddr, IpAddr};
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct AuthenticationRequest {
    account_name: Option<String>
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
    let ip: IpAddr = remote_address.ip();
    let is_allowed: bool = throttling::permission(&authentication_request.account_name, ip);

    let account_name: String = authentication_request.account_name
        .as_ref()
        .map(|s| s.clone())
        .unwrap_or("".to_string())
        .to_string();

    if !is_allowed {
        return status::Custom(
            AuthenticatorErrors::TooManyUserAccesses.status_code(),
            json!({ "message": AuthenticatorErrors::TooManyUserAccesses.get_error() }));
    } else {
        match &authentication_request.account_name {
            Some(account_name) => {
                match get_account(&account_name).await {
                    Err(_) => {
                        return status::Custom(
                            AuthenticatorErrors::AccountNotFound.status_code(),
                            json!({ "message": AuthenticatorErrors::AccountNotFound.get_error() }));
                    }, Ok(_) => {}
                }
            }, None => {}
        }

        let data = generate_data(&authentication_request.account_name);

        db.waiting_for_confirmation
            .insert(data.policy.id.as_bytes(), data.authentication_entry).unwrap();

        let answer = AnswerNew {
            id: data.policy.id,
            account_name: account_name,
            token: data.token,
            valid_until: data.policy.valid_until,
            policy: data.signature.base64_policy,
            signature: data.signature.signature
        };
        return status::Custom(
            Status::Accepted,
            json!({ "message": answer }));
    }
}

fn generate_data(account_name: &Option<String>) -> NewAuthenticationDataSet {
    let id = Uuid::new_v4();
    let secret = Uuid::new_v4();

    let start = SystemTime::now();
    let valid_until: u64 =
        start.duration_since(UNIX_EPOCH).unwrap().as_secs() + (86400 * 30);

    let policy = Policy {
        valid_until: format!("{}", valid_until),
        account_name: account_name
            .as_ref()
            .map(|s| s.clone())
            .unwrap_or("".to_string())
            .to_string(),
        id: id.to_string(),
    };

    let token = Uuid::new_v4().to_string();
    let b64_policy = encode(serde_json::to_string(&policy).unwrap());
    let signature = sign(&b64_policy, &secret.to_string(), &token);

    let new_authentication_entry = AuthenticationEntry {
        id: id.clone(),
        account_name: policy.account_name.to_string().clone(),
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
        rocket.mount("/api/v1/new", routes![new])
    })
}
