use config::{Config, ConfigError, File};
use serde::{Deserialize};

#[derive(Deserialize,Clone,Debug)]
pub struct Authenticator {
    pub host: String
}

#[derive(Deserialize,Clone,Debug)]
pub struct Redis {
    pub host: String
}

#[derive(Deserialize,Clone,Debug)]
pub struct Blockchain {
    pub host: String
}

#[derive(Deserialize,Clone,Debug)]
pub struct Settings {
	pub authenticator: Authenticator,
    pub redis: Redis,
	pub blockchain: Blockchain
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.merge(File::with_name("config/environment"))?;

        s.try_into()
    }
}
