use actix_web::{App, middleware, HttpServer, dev::Server};
use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};
use crate::utils::{settings::Settings};
use crate::router;

pub fn start_server (pool: Pool<ConnectionManager<PgConnection>>) -> Server {

    let settings = Settings::new().unwrap();

    let host = settings.authenticator.host;

    HttpServer::new(move || {
		App::new()
			.data(pool.clone())
			.data(Settings::new().unwrap())
			.wrap(middleware::Logger::default())
			.configure(router::init_routes)
    })
    .bind(host).unwrap()
    .run()
}