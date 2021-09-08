#![allow(unused)]
use crate::utils::throttling;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use actix_web::dev::Server;
use std::{thread, time};
use tokio::{
    task,
    time::{sleep, Duration}
};

mod api;
mod database;
mod router;
pub mod utils;
mod web;

lazy_static! {
    static ref INITIATED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

pub async fn run(is_test: bool) -> Result<Server, std::io::Error> {
    let mut initiated = INITIATED.lock().unwrap();
    if *initiated == false {
        std::env::set_var("IS_TEST", is_test.to_string());
        std::env::set_var("RUST_LOG", "actix_web=info,actix_redis=info");
        env_logger::init();
        *initiated = true;
    }

    let db = database::get_db();

    // Let's spawn the support services
    spawn_services(db.clone()); 

    // Let's start the web server
    let server = web::start_server(db.clone());

    Ok(server)
}

fn spawn_services(db: crate::database::Database) {
    thread::spawn(move || {
        utils::blockchain_updater::start(db.clone());
    });
    thread::spawn(move || {
        throttling::clean();
    });
}