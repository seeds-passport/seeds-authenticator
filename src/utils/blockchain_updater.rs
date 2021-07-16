use crate::utils::{
	blockchain::load_authentication_entries,
	logger::log,
	settings::Settings
};
use serde_json::Value;
use std::{
	thread,
	time::{SystemTime, UNIX_EPOCH},
	time
};
use tokio::{
	task,
	time::{sleep, Duration}
};

pub fn start(db: crate::database::Database) {
	let settings = Settings::new().unwrap();
	use tokio::runtime::Runtime;
	let rt = Runtime::new().unwrap();
	loop {
		rt.block_on(async {
			
			let mut last_blockchain_index = get_next_blockchain_id(&db);
			
			log(format!("Starting blockchain updater..."));

			let mut is_last: bool = false;
			while !is_last {

				let load_until = last_blockchain_index + u64::from(settings.blockchain.fetch_limit);
				log(format!("Loading indexes {} - {} from the blockchain...", last_blockchain_index, load_until));

				// // Get the authentication entries
				let response = load_authentication_entries(last_blockchain_index, load_until).await;
				match response {
					Ok(data) => {
						let length = data["rows"].as_array().unwrap().len();
				
						// If the length of the response is below the specified number in the settings, it means
						// that it is the last request we need to make (since it comes in batches of of the specified number number in the settings)
						is_last = length < settings.blockchain.fetch_limit as usize;
						
						match data["rows"].as_array().unwrap().last() {
							Some(record) => {
								last_blockchain_index = record["id"].clone().as_u64().unwrap() + 1;
							},
							None => {}
						}
		
						update_last_blockchain_id(&db, &last_blockchain_index);
		
						// Update records for this batch
						update_records(&db, data);
					},
					Err(_) => {
						log(format!("Blockchain call failed. Retrying..."));
					}
				}
			}

			// Once the previous steps are done we get into an infinite loop:
			sleep(Duration::from_millis(settings.blockchain.fetch_timeout)).await;
			loop {
				let load_until = last_blockchain_index + u64::from(settings.blockchain.fetch_limit);
				log(format!("Loading indexes {} - {} from the blockchain...", last_blockchain_index, load_until));
				let response = load_authentication_entries(last_blockchain_index, load_until).await;
				
				// Here we need to take the result of the previous function and store it on the database
				// Once we have that done we need to set the last_blockchain_id variable to the new
				// maximum index
				match response {
					Ok(data) => {
						match data["rows"].as_array().unwrap().last() {
							Some(record) => {
								last_blockchain_index = record["id"].clone().as_u64().unwrap() + 1;
							},
							None => {}
						}
						update_last_blockchain_id(&db, &last_blockchain_index);
						update_records(&db, data);
					},
					Err(_) => {
						log(format!("Blockchain call failed. Retrying..."));
					}
				}


				sleep(Duration::from_millis(settings.blockchain.fetch_timeout)).await;
			}
		});
	};
}

fn get_next_blockchain_id(db: &crate::database::Database) -> u64 {
	match db.state.get("state").unwrap() {
		Some(state) => {
			state.last_blockchain_id
		},
		None => 0
	}
}

fn update_records(db: &crate::database::Database, response: Value) {
	let response_iter = response["rows"].as_array().unwrap(); 
	
	for value in response_iter {
		// For each record in the response, we want to check the database for entries, and update its blockchain_index
		let backend_user_id = value["backend_user_id"].as_str().unwrap();
		let index = value["id"].as_u64().unwrap();

		let _ = db.authentication_entries.fetch_and_update(backend_user_id.as_bytes(), |el| {
			match el {
				Some(mut element) => {
					element.blockchain_index = Some(index);
					Some(element)
				},
				None => None
			}
		});
	}
}

fn update_last_blockchain_id(db: &crate::database::Database, last_blockchain_id: &u64) {
	match db.state.get("state").unwrap() {
		Some(mut state) => {
			state.last_blockchain_id = last_blockchain_id.clone();
			let _ = db.state.fetch_and_update("state", |el| {
				let mut element = el.unwrap();
				element.last_blockchain_id = last_blockchain_id.clone();
				Some(element)
			});
		},
		None => {
			let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

			db.state
				.insert("state", crate::database::State {
					last_blockchain_id: last_blockchain_id.clone(),
					last_updated_at: now
				}).unwrap();
		}
	}
}