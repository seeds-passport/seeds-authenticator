use ureq::{self, Error};
use actix_rt;
use tokio::{self, spawn};
use authenticator;
use authenticator::utils::{
	settings::Settings
};


#[tokio::main]
async fn spawn_app() {
    let _ = spawn(authenticator::run().await.expect("Error spawning the API."));
}

#[actix_rt::test]
async fn positive() {
	let settings = Settings::new().unwrap();

    spawn_app();

    let _resp = ureq::post("http://127.0.0.1:8080/api/v1/new")
        .send_json(ureq::json!({
            "account_name": settings.testing.account_name
        })).unwrap();

    assert_eq!(_resp.status(), 200);
}

#[actix_rt::test]
async fn wrong_account_name() {

    spawn_app();

    let _resp = ureq::post("http://127.0.0.1:8080/api/v1/new")
        .send_json(ureq::json!({
            "account_name": "wrong-account-name-for-testing"
        }));
    let mut status_code: u16 = 0;
    match _resp {
        Ok(response) => {
            status_code = response.status();
        },
        Err(Error::Status(code, _)) => {
            status_code = code;
        },
        Err(_) => {
            // Here for syntax purpose
            // Never reaches this endpoint
        }
    }
    assert_eq!(status_code, 403);
}