use actix_web::{web, HttpResponse, Result, HttpRequest};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::utils::{
	errors::AuthenticatorErrors,
	validate::{validate_token_and_fetch_from_blockchain, verify_credentials, CheckRequest},
	blockchain::load_user_data,
	settings::Settings
};

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
	if std::env::var("IS_TEST").unwrap().parse().unwrap() {
		let settings = Settings::new().unwrap();
		let token = params.token.to_string();
		if token == settings.testing.invalid_token {
			Err(AuthenticatorErrors::InvalidToken)
		} else if req.match_info().get("id").unwrap().to_string() == settings.testing.invalid_backend_id {
			Err(AuthenticatorErrors::InvalidId)
		} else if params.account_name.to_string() != settings.testing.account_name {
			Err(AuthenticatorErrors::AccountNotFound)
		} else {
			Ok(HttpResponse::Ok().json(json!(
				{
					"Ok": {
						"more": false,
						"next_key": "",
						"rows": [
							{
								"account": settings.testing.account_name,
								"image": "",
								"interests": "",
								"nickname": settings.testing.account_name,
								"reputation": 0,
								"roles": "",
								"skills": "",
								"status": "visitor",
								"story": "",
								"timestamp": 0,
								"type": "individual"
							}
						]
					}
				}
			)))
		}
	} else {
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
	 
}

