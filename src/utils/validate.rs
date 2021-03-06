use serde::{Deserialize, Serialize};
use std::str;
use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::Value;
use crate::utils::{
	errors::AuthenticatorErrors,
	blockchain::load_authentication_entries,
	signature::sign
};
use crate::database::{
	Database,
	AuthenticationEntry,
	get_authentication_entry
};

#[derive(Serialize, Deserialize)]
pub struct CheckRequest {
	pub token: String,
	pub account_name: String
}

pub async fn validate_token_and_fetch_from_blockchain(
	db: Database,
	id: &str,
	token: String
) -> Result<(AuthenticationEntry, Value), AuthenticatorErrors> {
	let authentication_entry_id = id;

	match get_authentication_entry(db.clone(), &authentication_entry_id.to_string(), &token) {
		Ok(data) => {
			match data.blockchain_index {
				Some(blockchain_index) => {
					match load_authentication_entries(blockchain_index, blockchain_index).await {
						Ok(response) => return Ok(
							(
								data,
								response["rows"]
									.as_array()
									.unwrap()
									.first()
									.unwrap()
									.clone()
							)
						),
						Err(_) => return Err(AuthenticatorErrors::BlockchainError)
					}
				},
				None => return Err(AuthenticatorErrors::NotStoredBlockchain)
			}
		},
		Err(error) => return Err(error)
	}
}

pub async fn verify_credentials(
	db_entry: AuthenticationEntry,
	blockchain_entry: Value,
	token: String
) -> Result<(), AuthenticatorErrors> {
	let blockchain_signature = blockchain_entry["signature"].as_str().unwrap();
	let b64_policy = db_entry.policy_base64.to_string();
	let secret = db_entry.secret.to_string();

	if b64_policy == db_entry.policy_base64.as_str() {
 		let policy: Value = 
			serde_json::from_str(
				str::from_utf8(
					&base64::decode(&db_entry.policy_base64.as_str()).unwrap()[..]
				).unwrap()
			).unwrap();
		let valid_until = u64::from_str_radix(&policy["valid_until"].as_str().unwrap(), 10).unwrap(); 
		let now: u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

		if valid_until < now {
			Err(AuthenticatorErrors::ExpiredPolicy)
		} else if blockchain_signature == sign(&b64_policy, &secret, &token).signature {

			Ok(())
		} else {
			Err(AuthenticatorErrors::InvalidSignature)
		}
	} else {
		Err(AuthenticatorErrors::MismatchedPolicies)
	}
}