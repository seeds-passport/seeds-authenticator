#[macro_use]
extern crate diesel;
extern crate dotenv;

mod api;
mod models;
mod router;
mod schema;
mod utils;
mod web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,actix_redis=info");
    env_logger::init();

    web::start_server().await
}