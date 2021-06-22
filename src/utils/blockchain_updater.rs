use crate::utils::{settings::Settings};
use tokio::time::{sleep, Duration};
use ureq;
use std::thread;

pub fn start() {
	thread::spawn(|| {

		use tokio::runtime::Runtime;
		let rt = Runtime::new().unwrap();

		rt.spawn(async move {
			let mut last_blockchain_id = "0";
			// let's verify here if we have already got records in the database
			// with blockchain_id filled and let's get the latest value
			// ...
			// If we already have records with that if filled, we get the maximum
			// value and add one. We set the last_blockchain_id to that value.
			// ...
			// Then we need to do a loop to get records (without the 1 sec timer)
			// and we stop once we got all the new records.

			// Once the previous steps are done we get into an infinite loop:
			loop {
				load_authentication_entries(last_blockchain_id).await;
				// Here we need to take the result of the previous function and store it on the database
				// Once we have that done we need to set the last_blockchain_id variable to the new
				// maximum index
				last_blockchain_id = "50";
				sleep(Duration::from_millis(1000)).await;
			}
		});

		// This next loop is needed to keep this thread alive.
		loop {};
   });

}

async fn load_authentication_entries(lower_bound: &str) {
	let settings = Settings::new().unwrap();

	let resp = ureq::post(&format!("{}/v1/chain/get_table_rows", settings.blockchain.host))
	.send_json(ureq::json!({
	    "json": true,
	    "code": "policy.seeds",
	    "scope": "policy.seeds",
	    "table": "devicepolicy",
	    "lower_bound": lower_bound,
	    "upper_bound": "",
	    "index_position": 1,
	    "key_type": "",
	    "limit": 50,
	    "reverse": false,
	    "show_payer": false
	}));
	println!("{:?}", resp);
	// we should now process the result and return it.
}	