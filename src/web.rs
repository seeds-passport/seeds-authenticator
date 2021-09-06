use actix_web::{App, middleware, HttpServer, dev::Server};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use crate::utils::{settings::Settings};
use crate::router;


lazy_static! {
  static ref INITIATED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
  static ref SERVER: Arc<Mutex<Vec<Server>>> = Arc::new(Mutex::new(vec!()));
}

pub fn start_server(db: crate::database::Database) -> Server {
  let mut initiated = INITIATED.lock().unwrap();
  let mut server = SERVER.lock().unwrap();
  if *initiated == false {
    let settings = Settings::new().unwrap();
    let host = settings.authenticator.host;

    let svr: Server = HttpServer::new(move || {
      App::new()
        .data(db.clone())
        .data(Settings::new().unwrap())
        .wrap(middleware::Logger::default())
        .configure(router::init_routes)
      })
      .bind(host).unwrap()
      .run();

    server.push(svr);
    *initiated = true;
  }

  return server[0].clone();
}