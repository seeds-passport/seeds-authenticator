mod api;
mod router;
mod utils;
mod database;
mod web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,actix_redis=info");
    env_logger::init();

    let db = database::get_db();

    // Let's start the long polling updater
    utils::blockchain_updater::start(db.clone());
     
    // Let's start the web server
    web::start_server(db).await
}