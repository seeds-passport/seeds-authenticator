
use serde::{Deserialize, Serialize};
use serde_json::json;
use actix_web::{web, HttpResponse, Result, HttpRequest};
use crate::utils::{
    errors::AuthenticatorErrors,
    validate::CheckRequest,
    settings::Settings
};
use crate::database::get_authentication_entry;

pub async fn invalidate(
    db: web::Data<crate::database::Database>,
    req: HttpRequest,
    params: web::Json<CheckRequest>,
) -> Result<HttpResponse, AuthenticatorErrors> {
    if std::env::var("IS_TEST").unwrap().parse().unwrap() {
        let settings = Settings::new().unwrap();
        let token = params.token.to_string();
        if token == settings.testing.invalid_token {
            Err(AuthenticatorErrors::InvalidToken)
        } else if req.match_info().get("id").unwrap().to_string() == settings.testing.invalid_backend_id {
            Err(AuthenticatorErrors::InvalidId)
        } else {
            Ok(HttpResponse::Ok().json(json!({"status": "ok"})))
        }
    }
    else {
        let authentication_entry_id = req.match_info().get("id").unwrap().to_string();
        let token = params.token.to_string();

        match get_authentication_entry(&db, &authentication_entry_id, &token) {
            Ok(_) => {
                db.authentication_entries.remove(authentication_entry_id).unwrap();
                Ok(HttpResponse::Ok().json(json!({"status": "ok"})))
            },
            Err(error) => return Err(error)
        }
    }
}
