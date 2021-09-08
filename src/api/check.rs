use actix_web::{web, HttpResponse, Result, HttpRequest};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::utils::{
	errors::AuthenticatorErrors,
	settings::Settings,
	validate::{
		validate_token_and_fetch_from_blockchain,
		verify_credentials,
		CheckRequest
	}
};

pub async fn check(
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
	} else {
		match validate_token_and_fetch_from_blockchain(db, req, &params).await {
			Ok((db_entry, blockchain_entry)) => {
				match verify_credentials(db_entry, blockchain_entry, params.token.to_string()).await {
					Ok(_) => return Ok(HttpResponse::Ok().json(json!({"status": "ok"}))),
					Err(error) => return Err(error)
				}
			}
			Err(error) => return Err(error)
		}
	}
	 
}

