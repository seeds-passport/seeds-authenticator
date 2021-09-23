#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

pub mod api;
pub mod utils;
mod database;
use std::thread;

use crate::utils::throttling;

#[launch]
fn rocket() -> _ {
    let db = database::get_db();
    spawn_services(db.clone());
    rocket::build()
        .manage(db)
        .attach(api::new::stage())
        .attach(api::invalidate::stage())
        .attach(api::info::stage())
        .attach(api::check::stage())
}

fn spawn_services(db: database::Database) {
    thread::spawn(move || {
        utils::blockchain_updater::start(db.clone());
    });
    thread::spawn(move || {
        throttling::clean();
    });
}
