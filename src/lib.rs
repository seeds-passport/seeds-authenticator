#![allow(unused)]
use crate::utils::{
    throttling
};
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

pub async fn run() -> Result<Server, std::io::Error> {
    std::env::set_var("RUST_LOG", "actix_web=info,actix_redis=info");
    env_logger::init();

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