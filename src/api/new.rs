
use serde::{Deserialize, Serialize};
use serde_json::json;
use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;
use chrono::{Duration, Utc};
use crate::utils::{
    blockchain::get_account,
    settings::Settings,
    errors::AuthenticatorErrors,
    signature::{Policy, sign}
};
use crate::models::authentication_entries::AuthenticationEntry;
use diesel::{
    PgConnection,
    prelude::*,
    r2d2::{ConnectionManager, self}
};


#[derive(Serialize, Deserialize, Debug)]
pub struct AnswerNew {
    id: String,
    account_name: String,
    token: String,
    valid_until: String,
    policy: String,
    signature: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DbRecordNew {
    id: String,
    account_name: String,
    secret: String,
    valid_until: String,
    policy: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthenticationRequest {
    account_name: Option<String>
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

            let id = Uuid::new_v4();
            let secret = Uuid::new_v4();
            let token = Uuid::new_v4().to_string();
            
            let valid_until = Utc::now().naive_utc() + Duration::days(30);

            let policy = Policy {
                valid_until: format!("{}", valid_until),
                account_name: account_name,
                id: id.to_string(),
            };

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

            let conn = pool.get().unwrap();
            let _ = diesel::insert_into(authentication_entries::table)
                .values(&new_authentication_entry)
                .execute(&conn);

            let answer = AnswerNew {
                id: policy.id,
                account_name: policy.account_name,
                token: token,
                valid_until: policy.valid_until, 
                policy: signature.base64_policy,
                signature: signature.signature
            };
            Ok(HttpResponse::Ok().json(answer))
        },
        Err(_) => {
            Err(AuthenticatorErrors::AccountNotFound)
        }
    }
}
