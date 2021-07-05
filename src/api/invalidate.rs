
use serde::{Deserialize, Serialize};
use serde_json::json;
use actix_web::{web, HttpResponse, Result, HttpRequest};
use crate::utils::{
    signature::hash_token,
    errors::AuthenticatorErrors,
};

#[derive(Serialize, Deserialize)]
pub struct InvalidateRequest {
    token: String,
}

pub async fn invalidate(
    db: web::Data<crate::database::Database>,
    req: HttpRequest,
    params: web::Json<InvalidateRequest>,
) -> Result<HttpResponse, AuthenticatorErrors> {

    let authentication_entry_id = req.match_info().get("id").unwrap().to_string();
    let token = params.token.to_string();

    match db.authentication_entries.get(&authentication_entry_id).unwrap() {
        Some(record) => {
            if record.token_hash == hash_token(&token) {
                db.authentication_entries.remove(authentication_entry_id).unwrap();
                Ok(HttpResponse::Ok().json(json!({"status": "ok"})))
            } else {
                Err(AuthenticatorErrors::InvalidToken)
            }
        }, None => {
            Err(AuthenticatorErrors::InvalidId)
        }
    }
}
