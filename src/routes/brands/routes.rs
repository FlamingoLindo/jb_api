use crate::middlewares::role::RoleGuard;
use crate::{handlers::brands as handler, middlewares::auth::validator};
use actix_web::web::{self};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(validator);

    cfg.service(
        web::scope("/brands").service(
            web::scope("").wrap(auth).service(
                web::scope("")
                    .wrap(RoleGuard("master"))
                    .route("/register", web::post().to(handler::create::create_brand))
                    .route("/{id}", web::get().to(handler::get::get_brand))
                    .route("", web::get().to(handler::get_all::get_brands))
                    .route("/{id}", web::patch().to(handler::update::update_brand))
                    .route("/{id}", web::delete().to(handler::delete::delete_brand))
                    .route("/block/{id}", web::patch().to(handler::block::block_brand)),
            ),
        ),
    );
}
