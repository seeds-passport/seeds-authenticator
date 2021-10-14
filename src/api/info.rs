use rocket::serde::json::{Json, Value, json};
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
            match verify_credentials(db_entry, blockchain_entry, check_request.token.clone()).await {
                Ok(_) => {
                    let user = load_user_data(&check_request.account_name).await.unwrap();
                    return status::Custom(
                        Status::Accepted,
                        json!({ "message": user  }));
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
        rocket.mount("/info", routes![info])
    })
}
