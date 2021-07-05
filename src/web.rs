use actix_web::{App, middleware, HttpServer, dev::Server};
use crate::utils::{settings::Settings};
use crate::router;

pub fn start_server (db: crate::database::Database) -> Server {

    let settings = Settings::new().unwrap();

    let host = settings.authenticator.host;

    HttpServer::new(move || {
		App::new()
			.data(db.clone())
			.data(Settings::new().unwrap())
			.wrap(middleware::Logger::default())
			.configure(router::init_routes)
    })
    .bind(host).unwrap()
    .run()
}