use actix_web::web;

use crate::handlers::users as handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users").route(
        "/register",
        web::post().to(handler::create_user::create_user),
    ));
}
