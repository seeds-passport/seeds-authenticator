use rocket::serde::json::{Json, Value, json};
use rocket::response::status;
use crate::{
    utils::validate::{
            validate_token_and_fetch_from_blockchain,
            verify_credentials,
            CheckRequest
    },
    database::Database
};
use rocket::http::Status;

#[post("/<id>", format = "json", data = "<check_request>")]
async fn check(db: Database, check_request: Json<CheckRequest>, id: &str) -> status::Custom<Value> {
    match validate_token_and_fetch_from_blockchain(db.clone(), id, check_request.token.clone()).await {
        Ok((db_entry, blockchain_entry)) => {
            match verify_credentials(db_entry, blockchain_entry, check_request.token.clone()).await {
                Ok(_) => {
                    return status::Custom(
                        Status::Accepted,
                        json!({ "message": {"status": "ok"} }));
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
        rocket.mount("/check", routes![check])
    })
}
