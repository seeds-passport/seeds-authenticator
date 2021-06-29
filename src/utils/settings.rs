use config::{Config, ConfigError, File};
use serde::{Deserialize};

#[derive(Deserialize,Clone,Debug)]
pub struct Authenticator {
    pub host: String
}

#[derive(Deserialize,Clone,Debug)]
pub struct Blockchain {
    pub host: String,
    pub fetch_limit: i8,
    pub fetch_timeout: u64
}

#[derive(Deserialize,Clone,Debug)]
pub struct Settings {
	pub authenticator: Authenticator,
	pub blockchain: Blockchain
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.merge(File::with_name("config/environment"))?;

        s.try_into()
    }
}
