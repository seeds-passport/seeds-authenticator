use crate::utils::{settings::Settings};
use tokio::time::{sleep, Duration};
use ureq;
use std::thread;
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::utils::logger::log;
use uuid::Uuid;

pub fn start(db: crate::database::Database) {
	thread::spawn(|| {
		let settings = Settings::new().unwrap();
		use tokio::runtime::Runtime;
		let rt = Runtime::new().unwrap();
		rt.spawn(async move {
			
			let mut last_blockchain_index = get_next_blockchain_id(&db);
			
			log(format!("Starting blockchain updater..."));

			let mut is_last: bool = false;
			while !is_last {

				let load_until = last_blockchain_index + u64::from(settings.blockchain.fetch_limit);
				log(format!("Loading indexes {} - {} from the blockchain...", last_blockchain_index, load_until));

				// // Get the authentication entries
				let response = load_authentication_entries(last_blockchain_index).await;
				let length = response["rows"].as_array().unwrap().len();
				
				// If the length of the response is below the specified number in the settings, it means
				// that it is the last request we need to make (since it comes in batches of of the specified number number in the settings)
				is_last = length < settings.blockchain.fetch_limit as usize;
				
				match response["rows"].as_array().unwrap().last() {
					Some(record) => {
						last_blockchain_index = record["id"].clone().as_u64().unwrap() + 1;
					},
					None => {}
				}

				update_last_blockchain_id(&db, &last_blockchain_index);

				// Update records for this batch
				update_records(&db, response);

			}

			// Once the previous steps are done we get into an infinite loop:
			sleep(Duration::from_millis(settings.blockchain.fetch_timeout)).await;
			loop {
				let load_until = last_blockchain_index + u64::from(settings.blockchain.fetch_limit);
				log(format!("Loading indexes {} - {} from the blockchain...", last_blockchain_index, load_until));
				let response = load_authentication_entries(last_blockchain_index).await;
				
				// Here we need to take the result of the previous function and store it on the database
				// Once we have that done we need to set the last_blockchain_id variable to the new
				// maximum index
				match response["rows"].as_array().unwrap().last() {
					Some(record) => {
						last_blockchain_index = record["id"].clone().as_u64().unwrap() + 1;
					},
					None => {}
				}

				update_last_blockchain_id(&db, &last_blockchain_index);
				update_records(&db, response);
				sleep(Duration::from_millis(settings.blockchain.fetch_timeout)).await;
			}
		});

		// This next loop is needed to keep this thread alive.
		loop{};
   });

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
		let backend_user_id: Uuid = Uuid::parse_str(value["backend_user_id"].as_str().unwrap()).unwrap();
		let index = value["id"].as_u64().unwrap();


		let _ = db.authentication_entries.fetch_and_update(backend_user_id.as_bytes(), |el| {
			match el {
				Some(mut element) => {
					println!("{:?}", element);
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

async fn load_authentication_entries(lower_bound: u64) -> Value {
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
	    "limit": i64::from(settings.blockchain.fetch_limit),
	    "reverse": false,
	    "show_payer": false
	})).unwrap();

	return serde_json::from_str(&resp.into_string().unwrap()).unwrap();
}	