use rocket::serde::json::{Json, Value};
use serde_json::json;
use rocket::response::status;
use crate::{
    utils::{
        validate::{
            CheckRequest, validate_token_and_fetch_from_blockchain, verify_credentials
        },
        blockchain::load_user_data
    },
    database::Database
};
use rocket::http::Status;

#[post("/<id>", format = "json", data = "<check_request>")]
async fn info(db: Database, check_request: Json<CheckRequest>, id: &str) -> status::Custom<Value> {
    match validate_token_and_fetch_from_blockchain(db, id, check_request.token.clone()).await {
        Ok((db_entry, blockchain_entry)) => {
            match verify_credentials(
                db_entry.clone(),
                blockchain_entry,
                check_request.token.clone()
            ).await {
                Ok(_) => {
                    let mut user = load_user_data(&db_entry.account_name.to_string()).await.unwrap()["rows"][0].clone();

                    user["token_valid_until"] = json!(&db_entry.valid_until.to_string());

                    return status::Custom(Status::Accepted, user);
                }
                Err(error) => {
                    return status::Custom(
                        error.status_code(),
                        json!({ "message": error.get_error() }));
                }
            }
        }
        Err(error) => {
            return status::Custom(
                error.status_code(),
                json!({ "message": error.get_error() }));
        }
    }
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("JSON", |rocket| async {
        rocket.mount("/api/v1/info", routes![info])
    })
}
