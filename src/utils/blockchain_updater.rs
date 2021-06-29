use crate::utils::{settings::Settings};
use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use tokio::time::{sleep, Duration};
use ureq;
use std::thread;
use diesel;
use diesel::prelude::*;
use crate::schema::authentication_entries;
use crate::schema::authentication_entries::dsl::*;
use crate::models::authentication_entries::AuthenticationEntry;
use serde_json::Value;
use uuid::Uuid;

pub fn start(pool: Pool<ConnectionManager<PgConnection>>) {
	thread::spawn(|| {
		use tokio::runtime::Runtime;
		let rt = Runtime::new().unwrap();
		rt.spawn(async move {
			// Retrieve the last blockchain index
			let mut last_blockchain_index = get_next_blockchain_id(&pool);
			
			let mut is_last: bool = false;
			while !is_last {
				// Get the authentication entries
				let response = load_authentication_entries(last_blockchain_index).await;
				let length = response["rows"].as_array().unwrap().len();
				
				// If the length of the response is below 50, it means
				// that it is the last request we need to make (since it comes in batches of 50)
				is_last = length < 50;
				if !is_last {
					last_blockchain_index = last_blockchain_index + 50;
				}

				// Update records for this batch
				update_records(&pool, response, length);
			}
			// Once the previous steps are done we get into an infinite loop:
			loop {
				let response = load_authentication_entries(last_blockchain_index).await;
				let length = response["rows"].as_array().unwrap().len();
				// Here we need to take the result of the previous function and store it on the database
				// Once we have that done we need to set the last_blockchain_id variable to the new
				// maximum index
				update_records(&pool, response, length);
				sleep(Duration::from_millis(1000)).await;
			}
		});

		// This next loop is needed to keep this thread alive.
		loop{};
   });

}

fn get_next_blockchain_id(pool: &Pool<ConnectionManager<PgConnection>>) -> i64 {
	// The default blockchain_index is 0
	let mut last_blockchain_index = 0;

	// Checks if there are records with blockchain_index filled, if so
	let result = authentication_entries
		.filter(authentication_entries::blockchain_index.is_not_null())
		.load::<AuthenticationEntry>(&pool.clone().get().unwrap());

	match result {
		Ok(entries) => {
			// Iterates through the records do get the latest blockchain_index
			entries.iter().for_each(|entry| {
				let db_index = entry.blockchain_index.unwrap();
				if last_blockchain_index < db_index {
					last_blockchain_index = db_index;
				}
			});
		},
		Err(error) => {
			// If any error occured, print it in the terminal
			println!("Error: {:?}", error);
		}
	}

	// If there are already values, the search must continue from the next value
	if last_blockchain_index != 0 {
		last_blockchain_index = last_blockchain_index + 1;
	}

	return last_blockchain_index;
}

fn update_records(pool: &Pool<ConnectionManager<PgConnection>>, response: Value, length: usize) {
	let mut i = 0;
	while i < length {
		// For each record in the response, we want to check the database for entries, and update its blockchain_index
		let backend_user_id: Uuid = Uuid::parse_str(&response["rows"][i]["backend_user_id"].as_str().unwrap()).unwrap();
		let index = &response["rows"][i]["id"].as_i64().unwrap();
		diesel::update(authentication_entries.filter(authentication_entries::id.eq(backend_user_id)))
			.set(authentication_entries::blockchain_index.eq(index))
			.execute(&pool.clone().get().unwrap()) 
			.unwrap();
		i += 1;
	}
}

async fn load_authentication_entries(lower_bound: i64) -> Value {
	let settings = Settings::new().unwrap();

	let resp = ureq::post(&format!("{}/v1/chain/get_table_rows", settings.blockchain.host))
	.send_json(ureq::json!({
	    "json": true,
	    "code": "policy.seeds",
	    "scope": "policy.seeds",
	    "table": "devicepolicy",
	    "lower_bound": lower_bound.to_string(),
	    "upper_bound": "",
	    "index_position": 1,
	    "key_type": "",
	    "limit": 50,
	    "reverse": false,
	    "show_payer": false
	})).unwrap();

	return serde_json::from_str(&resp.into_string().unwrap()).unwrap();
}	