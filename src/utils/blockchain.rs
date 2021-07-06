use actix_web::client::{Client};
use crate::utils::settings::Blockchain;
use serde::{Deserialize, Serialize};
use crate::utils::{settings::Settings};
use ureq;
use std::str;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
	account_name: String
}

pub async fn get_account(account_name: &String, settings: &Blockchain) -> Result<Account, &'static str> {
	let client = Client::default();

	let response = client.post(format!("{}/v1/chain/get_account", &settings.host))   // <- Create request builder
		.send_body(format!("{{\"account_name\": \"{}\"}}", &account_name))
		.await;

	match response {
		Ok(mut resp) => {

			let body = resp.body().await.unwrap();
			let json: Result<Account, _> = serde_json::from_str(str::from_utf8(&body).unwrap());

			match json {
				Ok(account) => {
					Ok(account)
				},
				Err(_) => {
					Err("Account not found")
				}
			}

		},
		Err(_) => Err("Invalid response")
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