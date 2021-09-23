use sled_extensions::{Config, bincode::Tree, DbExt};
use serde::{Deserialize, Serialize};
use rocket::request::{self, FromRequest, Request};
use crate::utils::{
    errors::AuthenticatorErrors,
    settings::Settings,
    signature::hash_token,
};
use rocket::outcome::IntoOutcome;

#[derive(Clone)]
pub struct Database {
    pub waiting_for_confirmation: Tree<AuthenticationEntry>,
    pub authentication_entries: Tree<AuthenticationEntry>,
    pub database_state: Tree<DatabaseState>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthenticationEntry {
    pub id: uuid::Uuid,
    pub account_name: String,
    pub secret: uuid::Uuid,
    pub policy_base64: String,
    pub valid_until: u64,
    pub blockchain_index: Option<u64>,
    pub token_hash: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DatabaseState {
    pub last_blockchain_id: u64,
    pub last_updated_at: u64,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Database {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, ()> {
        let outcome = request.rocket().state::<Database>()
            .map(|database| database.to_owned())
            .or_forward(());
        outcome
    }
}


pub fn get_db() -> Database {
    let db = Config::default()
        .path(Settings::new().unwrap().database.path.to_owned())
        .open()
        .unwrap();
    Database {
        waiting_for_confirmation: db.open_bincode_tree("waiting_for_confirmation").unwrap(),
        authentication_entries: db.open_bincode_tree("authentication_entries").unwrap(),
        database_state: db.open_bincode_tree("database_state").unwrap()
    }
}
pub fn get_authentication_entry(
    db: Database,
    id: &String,
    token: &String
) -> Result<AuthenticationEntry, AuthenticatorErrors> {

    match db.authentication_entries.get(&id).unwrap() {
        Some(record) => {
            if record.token_hash == hash_token(&token) {
                Ok(record)
            } else {
                Err(AuthenticatorErrors::InvalidToken)
            }
        }, None => {
            Err(AuthenticatorErrors::InvalidId)
        }
    }
}
pub fn get_waiting_for_confirmation(
    db: Database,
    id: &String
) -> Result<AuthenticationEntry, AuthenticatorErrors> {
    match db.waiting_for_confirmation.get(&id).unwrap() {
        Some(record) => Ok(record),
        None => Err(AuthenticatorErrors::InvalidId)
    }
}