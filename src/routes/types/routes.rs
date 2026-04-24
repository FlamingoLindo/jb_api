use crate::middlewares::role::RoleGuard;
use crate::{handlers::types as handler, middlewares::auth::validator};
use actix_web::web::{self};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(validator);

    cfg.service(
        web::scope("/types").service(
            web::scope("").wrap(auth).service(
                web::scope("")
                    .wrap(RoleGuard("user"))
                    .route("/create", web::post().to(handler::create::create_type))
                    .route("/{id}", web::get().to(handler::get::get_type))
                    .route("", web::get().to(handler::get_all::get_types))
                    .route("/{id}", web::patch().to(handler::update::update_type))
                    .route("/{id}", web::delete().to(handler::delete::delete_type))
                    .route("/block/{id}", web::patch().to(handler::block::block_type)),
            ),
        ),
    );
}
