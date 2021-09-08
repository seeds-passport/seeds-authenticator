use config::{Config, ConfigError, File};
use serde::{Deserialize};

#[derive(Deserialize,Clone,Debug)]
pub struct Authenticator {
    pub host: String,
    pub throttling_repeater_time: u64,
    pub throttling_old_entry: i64
}

#[derive(Deserialize,Clone,Debug)]
pub struct Blockchain {
    pub host: String,
    pub fetch_limit: u8,
    pub fetch_timeout: u64,
    pub request_amount_limit: i64,
    pub request_total_amount_limit: i64,
    pub request_time_limit: i64,
}

#[derive(Deserialize,Clone,Debug)]
pub struct Database {
    pub path: String
}

#[derive(Deserialize,Clone,Debug)]
pub struct Testing {
    pub account_name: String,
    pub invalid_token: String,
    pub invalid_backend_id: String
}

#[derive(Deserialize,Clone,Debug)]
pub struct Settings {
	pub authenticator: Authenticator,
	pub blockchain: Blockchain,
    pub database: Database,
    pub testing: Testing,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.merge(File::with_name("config/environment"))?;

        s.try_into()
    }
}