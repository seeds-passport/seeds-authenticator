use rocket::serde::json::{Json, Value};
use serde_json::json;
use rocket::serde::{Serialize, Deserialize};
use rocket::response::status;
use crate::database::{Database, get_authentication_entry};
use rocket::http::Status;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct InvalidateRequest {
    token: String
}


#[post("/<id>", format = "json", data = "<invalidate_request>")]
async fn invalidate(db: Database, invalidate_request: Json<InvalidateRequest>, id: &str) -> status::Custom<Value> {
    match get_authentication_entry(db.clone(), &id.to_string(), &invalidate_request.token) {
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
        rocket.mount("/api/v1/invalidate", routes![invalidate])
    })
}
