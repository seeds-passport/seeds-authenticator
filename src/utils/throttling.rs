use crate::utils::{
	settings::Settings
};
use chrono::prelude::*;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{
	collections::HashMap,
	sync::Mutex,
	thread,
	time
};
use tokio::time::{sleep, Duration};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AccessStatistic {
	pub account_name: String,
	pub ip_address: std::net::IpAddr,
	pub timestamp: DateTime<Utc>
}

/*
 * Mutex vector that holds the accesses statistics
 */
static ACCESS_STATISTICS: Lazy<Mutex<HashMap<String, Vec<AccessStatistic>>>> = Lazy::new(|| Mutex::new(HashMap::new()));

/**
 * Params:
 * 	account_name: account_name sent in the request' params
 * 	ip: the ip of the request
 * Returns:
 * 	true => the user can do the request
 * 	false => if the user reached the maximum number of requests allowed
 */
pub fn permission(account_name: &String, ip: std::net::IpAddr) -> bool {
	let settings = Settings::new().unwrap();
	let current_time: DateTime<Utc> = Utc::now();
	let access_identifier: String = format!("{}-{}", account_name, &ip.to_string());
	let access_statistic: AccessStatistic = AccessStatistic {
		account_name: account_name.clone(),
		ip_address: ip,
		timestamp: current_time
	}; 

	let mut amount_user_requests = 0;

	// Create a statistic entry
	// Or push the access to an existing new
	ACCESS_STATISTICS.lock().unwrap().entry(access_identifier.clone())
		.or_insert_with(Vec::new)
		.push(access_statistic);
	

	// Iterates through a user statistics
	match ACCESS_STATISTICS.lock().unwrap().get(&access_identifier) {
		Some(entries) => {
			for access in entries.iter() {
				let timestamp: DateTime<Utc> = access.timestamp;
				let entry_age = current_time.signed_duration_since(timestamp).num_seconds();
			
				// If the request was done by the same account_name, the same IP and within the time boundary
				// increase the ammount of requests
				if entry_age < settings.blockchain.request_time_limit && account_name == &access.account_name && ip == access.ip_address {
					amount_user_requests += 1;
				}
					
			}
		},
		None => println!("No record found")
	}

	// The user has reached the usage limit
	if amount_user_requests > settings.blockchain.request_amount_limit  {
		return false;
	}

	// The user is free to go
	return true;
}

pub fn clean () {
	let settings = Settings::new().unwrap();
	use tokio::runtime::Runtime;
	let rt = Runtime::new().unwrap();
	loop {
		rt.block_on(async {
			for (identifier, accesses) in ACCESS_STATISTICS.lock().unwrap().iter_mut() {
				let current_time: DateTime<Utc> = Utc::now();
				let mut new_accesses = vec![];
				for access in accesses.iter() {
					let timestamp: DateTime<Utc> = access.timestamp;
					let entry_age = current_time.signed_duration_since(timestamp).num_seconds();
					if entry_age < settings.authenticator.throttling_old_entry {
						new_accesses.push(access.to_owned());
					}
						
				}
				*accesses = new_accesses;
			}
			sleep(Duration::from_millis(settings.authenticator.throttling_repeater_time)).await;
		});
	};
}