use actix_web::{web, HttpResponse, Result, HttpRequest};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::utils::errors::AuthenticatorErrors;
use crate::utils::validate::{validate_token_and_fetch_from_blockchain, verify_credentials, CheckRequest};
use crate::utils::blockchain::load_user_data;

#[derive(Serialize, Deserialize)]
pub struct InfoRequest {
	token: String,
	account_name: String
}

pub async fn info(
	db: web::Data<crate::database::Database>,
	req: HttpRequest,
	params: web::Json<InfoRequest>,
) -> Result<HttpResponse, AuthenticatorErrors> {
	match validate_token_and_fetch_from_blockchain(
		db,
		req,
		&web::Json(CheckRequest {
			token: params.token.to_owned()
		})
	).await {
		Ok((db_entry, blockchain_entry)) => {
			match verify_credentials(db_entry, blockchain_entry,  params.token.to_string()).await {
				Ok(_) => {
					return Ok(HttpResponse::Ok().json(load_user_data(&params.account_name).await))
				},
				Err(error) => return Err(error)
			}
		}
		Err(error) => return Err(error)
	}
	 
}

