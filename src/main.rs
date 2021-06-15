#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{App, middleware, HttpServer};
use diesel::{r2d2::{self, ConnectionManager}, PgConnection};
use utils::{settings::Settings};

mod api;
mod models;
mod router;
mod schema;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,actix_redis=info");
    env_logger::init();

    let settings = Settings::new().unwrap();

    let host = settings.authenticator.host;

    dotenv::dotenv().ok();

    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(Settings::new().unwrap())
            .wrap(middleware::Logger::default())
            .configure(router::init_routes)
    })
    .bind(host)?
    .run()
    .await
}