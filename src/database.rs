use sled_extensions::{Config, bincode::Tree, DbExt};
use crate::utils::{settings::Settings};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Database {
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
        authentication_entries: db.open_bincode_tree("authentication_entries").unwrap(),
        state: db.open_bincode_tree("state").unwrap()
    }
}