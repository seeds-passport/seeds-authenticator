
use serde::{Deserialize, Serialize};
use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;
use r2d2_redis::{r2d2::Pool, RedisConnectionManager};
use r2d2_redis::redis::Commands;
use chrono::{Duration, Utc};
use crate::utils::{
    blockchain::get_account,
    settings::Settings,
    errors::AuthenticatorErrors,
    signature::{Policy, sign}
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

pub async fn new(
    pool: web::Data<Pool<RedisConnectionManager>>,
    params: web::Json<AuthenticationRequest>,
    settings: web::Data<Settings>
) -> Result<HttpResponse, AuthenticatorErrors> {

    let mut conn = pool.get().unwrap();
    let account_name = params.account_name.as_ref().unwrap().to_string();

    match get_account(&account_name, &settings.blockchain).await {
        Ok(_) => {

            let id = Uuid::new_v4().to_string();
            let secret = Uuid::new_v4().to_string();
            let token = Uuid::new_v4().to_string();
            let valid_until = Utc::now() + Duration::days(30);

            let policy = Policy {
                valid_until: format!("{}", valid_until),
                account_name: account_name,
                id: id
            };

            let signature = sign(&policy, &secret, &token);

            let db_record = DbRecordNew {
                id: policy.id.clone(),
                account_name: policy.account_name.clone(),
                secret: secret,
                valid_until: policy.valid_until.clone(), 
                policy: signature.base64_policy.clone()
            };

            let _ : () = conn.set(&db_record.id, serde_json::to_string(&db_record).unwrap()).unwrap();

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
