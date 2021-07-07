use sha2::{Sha256, Digest};
use hmac::{Hmac, Mac, NewMac};
use serde::{Deserialize, Serialize};
use base64::{encode};

#[derive(Serialize, Deserialize, Debug)]
pub struct Policy {
    pub valid_until: String,
    pub account_name: String,
    pub id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Signature {
    pub base64_policy: String,
    pub signature: String
}


pub fn sign(policy: &Policy, secret: &String, token: &String) -> Signature {
	type HmacSha256 = Hmac<Sha256>;
	let b64_policy = encode(serde_json::to_string(&policy).unwrap());

	let signing_key = format!("{}|{}", &secret, &token);

	let mut mac = HmacSha256::new_from_slice(signing_key.as_bytes()).unwrap();

	mac.update(b64_policy.as_bytes());

	let result = mac.finalize();
	let signature = encode(result.into_bytes());

	Signature {
		base64_policy: b64_policy,
		signature: signature
	}
}

pub fn hash_token(token: &String) -> String {
	let mut hasher = Sha256::new();

    hasher.update(&token.to_string());

	format!("{:x}", hasher.finalize()) 
}