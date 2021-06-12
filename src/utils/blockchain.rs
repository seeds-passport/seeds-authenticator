use actix_web::client::{Client};
use crate::utils::settings::Blockchain;
use serde::{Deserialize, Serialize};
use std::str;

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
					println!("{:?}", &account);
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