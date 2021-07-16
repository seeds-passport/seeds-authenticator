#![allow(unused)]
use crate::utils::{
    throttling
};
use std::{thread, time};
use tokio::{
    task,
    time::{sleep, Duration}
};

mod api;
mod database;
mod router;
mod utils;
mod web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,actix_redis=info");
    env_logger::init();

    let db = database::get_db();

    // Let's spawn the support services
    spawn_services(db.clone()); 

    // Let's start the web server
    web::start_server(db.clone()).await
}

fn spawn_services(db: crate::database::Database) {
    thread::spawn(move || {
        utils::blockchain_updater::start(db.clone());
    });
    thread::spawn(move || {
        throttling::clean();
    });
}