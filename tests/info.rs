use ureq::{self, Error};
use actix_rt;
use tokio::{self, spawn};
use serde_json::Value;
use authenticator::{self, utils::settings::Settings};

#[tokio::main]
async fn spawn_app() {
    let _ = spawn(authenticator::run().await.expect("Error spawning the API."));
}

// repeat tests when in blockchain

#[actix_rt::test]
async fn valid_token_valid_accountname_not_in_blockchain() {
	let settings = Settings::new().unwrap();

    spawn_app();

    let authentication: Value = ureq::post("http://127.0.0.1:8080/api/v1/new")
        .send_json(ureq::json!({
            "account_name": settings.testing.account_name
        }))
        .unwrap()
        .into_json()
        .unwrap();
    
    let request_url = "http://127.0.0.1:8080/api/v1/info/".to_owned() + authentication.get("id").unwrap().as_str().unwrap();
    let request = ureq::post(request_url.as_str())
        .send_json(ureq::json!({
            "account_name": settings.testing.account_name,
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
async fn valid_token_invalid_accountname_not_in_blockchain() {
	let settings = Settings::new().unwrap();

    spawn_app();

    let authentication: Value = ureq::post("http://127.0.0.1:8080/api/v1/new")
        .send_json(ureq::json!({
            "account_name": settings.testing.account_name
        }))
        .unwrap()
        .into_json()
        .unwrap();
    
    let request_url = "http://127.0.0.1:8080/api/v1/info/".to_owned() + authentication.get("id").unwrap().as_str().unwrap();
    let request = ureq::post(request_url.as_str())
        .send_json(ureq::json!({
            "account_name": "just an invalid account_name",
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

    //assert_eq!(status_code, 200); => we need to check this
    assert_eq!(status_code, 404);
}

#[actix_rt::test]
async fn invalid_token_valid_accountname_not_in_blockchain() {
	let settings = Settings::new().unwrap();

    spawn_app();

    let authentication: Value = ureq::post("http://127.0.0.1:8080/api/v1/new")
        .send_json(ureq::json!({
            "account_name": settings.testing.account_name
        }))
        .unwrap()
        .into_json()
        .unwrap();
    
    let request_url = "http://127.0.0.1:8080/api/v1/info/".to_owned() + authentication.get("id").unwrap().as_str().unwrap();
    let request = ureq::post(request_url.as_str())
        .send_json(ureq::json!({
            "account_name": settings.testing.account_name,
            "token": "just an invalid token"
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

    // assert_eq!(status_code, 403); uncomment and delete next next line once we can integrate with blockchain
    assert_eq!(status_code, 404);
}
#[actix_rt::test]
async fn invalid_token_invalid_accountname_not_in_blockchain() {
	let settings = Settings::new().unwrap();

    spawn_app();

    let authentication: Value = ureq::post("http://127.0.0.1:8080/api/v1/new")
        .send_json(ureq::json!({
            "account_name": settings.testing.account_name
        }))
        .unwrap()
        .into_json()
        .unwrap();
    
    let request_url = "http://127.0.0.1:8080/api/v1/info/".to_owned() + authentication.get("id").unwrap().as_str().unwrap();
    let request = ureq::post(request_url.as_str())
        .send_json(ureq::json!({
            "account_name": "just an invalid account_name",
            "token": "just an invalid token"
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

    // assert_eq!(status_code, 403); uncomment and delete next next line once we can integrate with blockchain
    assert_eq!(status_code, 404);
}