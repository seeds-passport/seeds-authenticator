use ureq::{self, Error};
use actix_rt;
use tokio::{self, spawn};
use authenticator::{self, utils::settings::Settings};


#[tokio::main]
async fn spawn_app() {
    let _ = spawn(authenticator::run().await.expect("Error spawning the API."));
}

#[actix_rt::test]
async fn positive_correct_account_name() {
	let settings: Settings = Settings::new().unwrap();

    spawn_app();

    let _resp = ureq::post("http://127.0.0.1:8080/api/v1/new")
        .send_json(ureq::json!({
            "account_name": settings.testing.account_name
        })).unwrap();

    assert_eq!(_resp.status(), 200);
}

#[actix_rt::test]
async fn negative_wrong_account_name() {

    spawn_app();

    let _resp = ureq::post("http://127.0.0.1:8080/api/v1/new")
        .send_json(ureq::json!({
            "account_name": "wrong"
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
    assert_eq!(status_code, 404);
}

#[actix_rt::test]
async fn negative_account_name_too_long() {
    spawn_app();
    let mut status_code: u16 = 0;
    let _resp = ureq::post("http://127.0.0.1:8080/api/v1/new")
        .send_json(ureq::json!({
            "account_name": "iamwaaaaaaytoooolong"
        }));
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

#[actix_rt::test]
async fn negative_request_limit_surpassed() {
    let settings: Settings = Settings::new().unwrap();
    spawn_app();
    let request_amount_limit: i64 = settings.blockchain.request_amount_limit + 5;
    let mut status_code: u16 = 0;
    for _ in 0..request_amount_limit {
        let _resp = ureq::post("http://127.0.0.1:8080/api/v1/new")
            .send_json(ureq::json!({
            "account_name": settings.testing.account_name
        }));
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
    }

    assert_eq!(status_code, 429);
}