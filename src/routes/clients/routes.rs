use crate::middlewares::role::RoleGuard;
use crate::{handlers::clients as handler, middlewares::auth::validator};
use actix_web::web::{self};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(validator);

    cfg.service(
        web::scope("/clients").service(
            web::scope("").wrap(auth).service(
                web::scope("")
                    .wrap(RoleGuard("user"))
                    .route("/register", web::post().to(handler::create::create_client))
                    .route("", web::get().to(handler::get_all::get_clients))
                    .route(
                        "/available",
                        web::get().to(handler::available::available_clients),
                    )
                    .route("/{id}", web::get().to(handler::get::get_client))
                    .route("/{id}", web::patch().to(handler::update::update_client))
                    .route("/block/{id}", web::patch().to(handler::block::block_client))
                    .route("/{id}", web::delete().to(handler::delete::delete_client)),
            ),
        ),
    );
}
