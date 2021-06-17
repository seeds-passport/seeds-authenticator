use actix_web::{App, middleware, HttpServer, dev::Server};
use diesel::{r2d2::{self, ConnectionManager}, PgConnection};
use crate::utils::{settings::Settings};
use crate::router;

pub fn start_server () -> Server {

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
    .bind(host).unwrap()
    .run()
}