use actix_web::{web, web::ServiceConfig};
use crate::api;

pub fn init_routes(cfg: &mut ServiceConfig) {

    cfg.service(
        web::resource("/api/v1/new").route(web::post().to(api::new::new)),
	);

    cfg.service(
        web::resource("/api/v1/invalidate/{id}").route(web::post().to(api::invalidate::invalidate)),
	);

}