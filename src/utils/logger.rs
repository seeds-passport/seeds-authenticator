use chrono::{DateTime, Utc};

pub fn log(message: String) {
	let now: DateTime<Utc> = Utc::now();
	println!("[{:?}] {}", now, message);
}