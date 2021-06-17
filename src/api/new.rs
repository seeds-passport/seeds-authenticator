
use serde::{Deserialize, Serialize};
use serde_json::json;
use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;
use chrono::{Duration, Utc};
use crate::utils::{
    blockchain::get_account,
    settings::Settings,
    errors::AuthenticatorErrors,
    signature::{Policy, sign, Signature}
};
use crate::models::authentication_entries::{AuthenticationEntry};
use diesel::{
    PgConnection,
    prelude::*,
    r2d2::{ConnectionManager, self}
};

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

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn new(
    pool: web::Data<Pool>,
    params: web::Json<AuthenticationRequest>,
    settings: web::Data<Settings>
) -> Result<HttpResponse, AuthenticatorErrors> {
    use crate::schema::authentication_entries;

    let account_name = params.account_name.as_ref().unwrap().to_string();

    match get_account(&account_name, &settings.blockchain).await {
        Ok(_) => {

			let data = generate_data(&account_name);

            let conn = pool.get().unwrap();
            let _ = diesel::insert_into(authentication_entries::table)
                .values(&data.authentication_entry)
                .execute(&conn);
			
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

	let valid_until = Utc::now().naive_utc() + Duration::days(30);

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
		valid_until: valid_until.into(), 
		policy: json!(policy),
		policy_base64: signature.base64_policy.clone(),
		blockchain_index: None
	};

	NewAuthenticationDataSet {
		authentication_entry: new_authentication_entry,
		policy: policy,
		token: token,
		signature: signature
	}  
}
