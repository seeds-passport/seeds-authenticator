use rocket::serde::json::{Json, Value, json};
use rocket::response::status;
use crate::{
    utils::validate::CheckRequest,
    database::{Database, get_authentication_entry}
};
use rocket::http::Status;

#[post("/<id>", format = "json", data = "<check_request>")]
async fn info(db: Database, check_request: Json<CheckRequest>, id: &str) -> status::Custom<Value> {
    match get_authentication_entry(db.clone(), &id.to_string(), &check_request.token) {
        Ok(_) => {
            db.authentication_entries.remove(id).unwrap();
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

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("JSON", |rocket| async {
        rocket.mount("/info", routes![info])
    })
}
