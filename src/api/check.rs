use actix_web::{web, HttpResponse, Result, HttpRequest};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::utils::errors::AuthenticatorErrors;
use crate::utils::validate::{validate_token_and_fetch_from_blockchain, verify_credentials, CheckRequest};

pub async fn check(
	db: web::Data<crate::database::Database>,
	req: HttpRequest,
	params: web::Json<CheckRequest>,
) -> Result<HttpResponse, AuthenticatorErrors> {
	match validate_token_and_fetch_from_blockchain(db, req, &params).await {
		Ok((db_entry, blockchain_entry)) => {
			match verify_credentials(db_entry, blockchain_entry,  params.token.to_string()).await {
				Ok(_) => return Ok(HttpResponse::Ok().json(json!({"status": "ok"}))),
				Err(error) => return Err(error)
			}
		}
		Err(error) => return Err(error)
	}
	 
}

