#[macro_use]
extern crate diesel;
extern crate dotenv;

mod api;
mod models;
mod router;
mod schema;
mod utils;
mod database;
mod web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,actix_redis=info");
    env_logger::init();

    // Creating the database pool
    let pool = database::get_pool();

    // Let's start the long polling updater
    utils::blockchain_updater::start(pool.clone());
     
    // Let's start the web server
    web::start_server(pool.clone()).await
}