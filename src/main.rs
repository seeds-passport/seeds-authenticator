use actix_web::{
    App,
    middleware,
    HttpServer
};
mod api;
mod router;
mod utils;
use utils::{
    settings::Settings
};
use r2d2_redis::{r2d2, RedisConnectionManager};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,actix_redis=info");
    env_logger::init();

    let settings = Settings::new().unwrap();

    let redis_host = settings.redis.host;
    let host = settings.authenticator.host;
    let manager = RedisConnectionManager::new(redis_host).unwrap();
    let pool = r2d2::Pool::builder()
        .build(manager)
        .unwrap();

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