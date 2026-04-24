use crate::middlewares::role::RoleGuard;
use crate::{handlers::images as handler, middlewares::auth::validator};
use actix_web::web::{self};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(validator);

    cfg.service(
        web::scope("/images").service(
            web::scope("")
                .wrap(RoleGuard("user"))
                .wrap(auth)
                .route(
                    "/delete/{id}",
                    web::delete().to(handler::delete::delete_image),
                )
                .route(
                    "/upload/{entity}",
                    web::post().to(handler::upload::save_file),
                ),
        ),
    );
}
