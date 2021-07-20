use sled_extensions::{Config, bincode::Tree, DbExt};
use actix_web::{Result, web};
use chrono::prelude::*;

use crate::utils::{
    settings::Settings,
    signature::hash_token,
    errors::AuthenticatorErrors
};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Database {
    pub waiting_for_confirmation: Tree<AuthenticationEntry>,
    pub authentication_entries: Tree<AuthenticationEntry>,
    pub state: Tree<State>
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
pub struct State {
    pub last_blockchain_id: u64,
    pub last_updated_at: u64,
}

pub fn get_db() -> Database {
    let settings = Settings::new().unwrap();

    let db = Config::default()
        .path(settings.database.path.to_owned())
        .open()
        .unwrap();

    Database {
        waiting_for_confirmation: db.open_bincode_tree("waiting_for_confirmation").unwrap(),
        authentication_entries: db.open_bincode_tree("authentication_entries").unwrap(),
        state: db.open_bincode_tree("state").unwrap()
    }
}

pub fn get_authentication_entry(
    db: &web::Data<crate::database::Database>,
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
    db: &crate::database::Database,
    id: &String
) -> Result<AuthenticationEntry, AuthenticatorErrors> {

    match db.waiting_for_confirmation.get(&id).unwrap() {
        Some(record) => Ok(record), 
        None => Err(AuthenticatorErrors::InvalidId)
    }
}