use ureq;
use actix_rt;
use tokio::{self, spawn};
use authenticator;
use authenticator::utils::{
	settings::Settings
};

#[actix_rt::test]
async fn new_works() {
	let settings = Settings::new().unwrap();

    spawn_app();

    let _resp = ureq::post("http://127.0.0.1:8080/api/v1/new")
        .send_json(ureq::json!({
            "account_name": settings.testing.account_name
        })).unwrap();

    assert_eq!(_resp.status(), 200);
}

#[tokio::main]
async fn spawn_app() {
    let _ = spawn(authenticator::run().await.expect("Error spawning the API."));
}