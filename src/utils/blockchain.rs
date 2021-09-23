use serde::{Deserialize, Serialize};
use crate::utils::{settings::Settings, settings::Blockchain, helpers::name_bytes_to_u64};
use ureq;
use std::str;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
	account_name: String
}

pub async fn get_account(account_name: &String) -> Result<Account, &'static str> {
	let settings = Settings::new().unwrap();

	let resp = ureq::post(&format!("{}/v1/chain/get_account", settings.blockchain.host))
		.send_json(ureq::json!({
	    "account_name": &account_name,
	}));

	match resp {
		Ok(response) => {
			let json: Result<Account, _> = serde_json::from_str(&response.into_string().unwrap());
			match json {
				Ok(account) => {
					Ok(account)
				},
				Err(_) => {
					Err("Account not found")
				}
			}
		},
		Err(_) => {
			Err("Invalid response")
		}
	}


}

pub async fn load_authentication_entries(lower_bound: u64, upper_bound: u64) -> Result<Value, &'static str> {
	let settings = Settings::new().unwrap();

	let resp = ureq::post(&format!("{}/v1/chain/get_table_rows", settings.blockchain.host))
	.send_json(ureq::json!({
	    "json": true,
	    "code": "policy.seeds",
	    "scope": "policy.seeds",
	    "table": "devicepolicy",
	    "lower_bound": lower_bound.to_string(),
	    "upper_bound": upper_bound.to_string(),
	    "index_position": 1,
	    "key_type": "",
	    "limit": i64::from(settings.blockchain.fetch_limit),
	    "reverse": false,
	    "show_payer": false
	}));

	match resp {
		Ok(response) => {
			Ok(serde_json::from_str(&response.into_string().unwrap()).unwrap())
		},
		Err(_) => {
			Err("Error loading from blockchain")
		}
	}

}

pub async fn load_user_data(account_name: &String) -> Result<Value, &'static str> {
	let settings = Settings::new().unwrap();
	let account_name_as_u64 = name_bytes_to_u64(account_name.bytes()).unwrap();
	let resp = ureq::post(&format!("{}/v1/chain/get_table_rows", settings.blockchain.host))
	.send_json(ureq::json!({
		"json": true,
		"code": "accts.seeds",
		"scope": "accts.seeds",
		"lower_bound": account_name_as_u64,
		"upper_bound": account_name_as_u64,
		"limit": 1,
		"table": "users"
	}));

	match resp {
		Ok(response) => {
			Ok(serde_json::from_str(&response.into_string().unwrap()).unwrap())
		},
		Err(_) => {
			Err("Error loading from blockchain")
		}
	}

}