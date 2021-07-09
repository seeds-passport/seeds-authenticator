use serde::{Deserialize, Serialize};
use chrono::prelude::*;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::utils::{
	settings::Settings
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AccessStatistic {
	pub account_name: String,
	pub ip_address: std::net::IpAddr,
	pub timestamp: DateTime<Utc>
}

/*
 * Mutex vector that holds the accesses statistics
 */
static ACCESS_STATISTICS: Lazy<Mutex<Vec<AccessStatistic>>> = Lazy::new(|| Mutex::new(vec![]));

/**
 * Params:
 * 	account_name: account_name sent in the request' params
 * 	ip: the ip of the request
 * Returns:
 * 	0 => the user can do the request
 * 	1 => if the API reached the maximum number of requests
 * 	2 => if the user reached the maximum number of requests allowed
 */
pub fn permission(account_name: &String, ip: std::net::IpAddr) -> i8 {
	let settings = Settings::new().unwrap();
	let current_time: DateTime<Utc> = Utc::now();

	// Add the access to the accesses mutex
	ACCESS_STATISTICS.lock().unwrap().push(
		AccessStatistic {
			account_name: account_name.clone(),
			ip_address: ip,
			timestamp: current_time
		}
	);
	
	let mut amount_user_requests = 0;
	let mut amount_total_requests = 0;
	for access in ACCESS_STATISTICS.lock().unwrap().iter() {
		let timestamp: DateTime<Utc> = access.timestamp;
		let entry_age = current_time.signed_duration_since(timestamp).num_seconds();

		// If the request was done by the same account_name, the same IP and within the time boundary
		// increase the ammount of requests
		if entry_age < settings.blockchain.request_time_limit && account_name == &access.account_name && ip == access.ip_address {
			amount_user_requests += 1;
		}

		// If the request was done within the time boundary
		// increase the ammount of requests
		if entry_age < settings.blockchain.request_time_limit {
			amount_total_requests += 1
		}
		
	}
	if amount_total_requests > settings.blockchain.request_total_amount_limit {
		return 1;
	}
	if amount_user_requests > settings.blockchain.request_amount_limit  {
		return 2;
	}
	return 0;
}
