use ureq::{self, Error};
use actix_rt;
use tokio::{self, spawn};
use serde_json::Value;
use authenticator::{self, utils::settings::Settings};

#[tokio::main]
async fn spawn_app() {
    let _ = spawn(authenticator::run(true).await.expect("Error spawning the API."));
}

#[actix_rt::test]
async fn valid_token_not_in_blockchain() {
	let settings = Settings::new().unwrap();

    spawn_app();

    let authentication: Value = ureq::post("http://127.0.0.1:8080/api/v1/new")
        .send_json(ureq::json!({
            "account_name": settings.testing.account_name
        }))
        .unwrap()
        .into_json()
        .unwrap();
    
    let request_url = "http://127.0.0.1:8080/api/v1/check/".to_owned() + settings.testing.invalid_backend_id.as_str();
    let request = ureq::post(request_url.as_str())
        .send_json(ureq::json!({
            "token": authentication.get("token").unwrap()
        }));
    let mut status_code: u16 = 0;
    match request {
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

    assert_eq!(status_code, 404);
}

#[actix_rt::test]
async fn valid_token_in_blockchain() {
    let settings = Settings::new().unwrap();

    spawn_app();

    let authentication: Value = ureq::post("http://127.0.0.1:8080/api/v1/new")
        .send_json(ureq::json!({
            "account_name": settings.testing.account_name
        }))
        .unwrap()
        .into_json()
        .unwrap();

    let request_url = "http://127.0.0.1:8080/api/v1/check/".to_owned() + authentication.get("id").unwrap().as_str().unwrap();
    let request = ureq::post(request_url.as_str())
        .send_json(ureq::json!({
            "token": authentication.get("token").unwrap()
        }));
    let mut status_code: u16 = 0;
    match request {
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

    assert_eq!(status_code, 200);
}

#[actix_rt::test]
async fn invalid_token() {
	let settings = Settings::new().unwrap();

    spawn_app();

    let authentication: Value = ureq::post("http://127.0.0.1:8080/api/v1/new")
        .send_json(ureq::json!({
            "account_name": settings.testing.account_name
        }))
        .unwrap()
        .into_json()
        .unwrap();
    
    let request_url = "http://127.0.0.1:8080/api/v1/check/".to_owned() + authentication.get("id").unwrap().as_str().unwrap();
    let request = ureq::post(request_url.as_str())
        .send_json(ureq::json!({
            "token": settings.testing.invalid_token
        }));
    let mut status_code: u16 = 0;
    match request {
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
