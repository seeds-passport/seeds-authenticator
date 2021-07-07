
use serde::{Deserialize, Serialize};
use actix_web::{web, HttpResponse, Result};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use crate::utils::{
    blockchain::get_account,
    settings::Settings,
    errors::AuthenticatorErrors,
    signature::{Policy, sign, Signature, hash_token}
};
use crate::database::{AuthenticationEntry};

#[derive(Serialize, Deserialize)]
pub struct AnswerNew {
    id: String,
    account_name: String,
    token: String,
    valid_until: String,
    policy: String,
    signature: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthenticationRequest {
    account_name: Option<String>
}

#[derive(Serialize, Deserialize)]
struct NewAuthenticationDataSet {
    authentication_entry: AuthenticationEntry,
    policy: Policy,
	token: String,
	signature: Signature
}

pub async fn new(
    db: web::Data<crate::database::Database>,
    params: web::Json<AuthenticationRequest>,
    settings: web::Data<Settings>
) -> Result<HttpResponse, AuthenticatorErrors> {

    let account_name = params.account_name.as_ref().unwrap().to_string();

    match get_account(&account_name, &settings.blockchain).await {
        Ok(_) => {

			let data = generate_data(&account_name);

            db.authentication_entries
                .insert(data.policy.id.as_bytes(), data.authentication_entry).unwrap();

            let answer = AnswerNew {
                id: data.policy.id,
                account_name: data.policy.account_name,
                token: data.token,
                valid_until: data.policy.valid_until, 
                policy: data.signature.base64_policy,
                signature: data.signature.signature
            };
            Ok(HttpResponse::Ok().json(answer))
        },
        Err(_) => {
            Err(AuthenticatorErrors::AccountNotFound)
        }
    }
}

fn generate_data(account_name: &String) -> NewAuthenticationDataSet {
	let id = Uuid::new_v4();
	let secret = Uuid::new_v4();

    let start = SystemTime::now();
    let valid_until: u64 =
        start.duration_since(UNIX_EPOCH).unwrap().as_secs() + (86400 * 30);

	let policy = Policy {
		valid_until: format!("{}", valid_until),
		account_name: account_name.to_string(),
		id: id.to_string(),
	};

	let token = Uuid::new_v4().to_string();
	let signature = sign(&policy, &secret.to_string(), &token);

	let new_authentication_entry = AuthenticationEntry {
		id: id.clone(),
		account_name: policy.account_name.clone(),
		secret: secret,
		valid_until: valid_until, 
		policy_base64: signature.base64_policy.clone(),
		blockchain_index: None,
        token_hash: hash_token(&token)
	};

	NewAuthenticationDataSet {
		authentication_entry: new_authentication_entry,
		policy: policy,
		token: token,
		signature: signature
	}  
}
