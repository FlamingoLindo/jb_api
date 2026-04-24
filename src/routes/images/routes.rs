use crate::middlewares::role::RoleGuard;
use crate::{handlers::images as handler, middlewares::auth::validator};
use actix_web::web::{self};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config(cfg: &mut web::ServiceConfig) {
    let auth = || HttpAuthentication::bearer(validator);

    cfg.service(
        web::scope("/images")
            // Master-only routes (empty for now)
            // .service(
            //     web::resource("/some-master-route")
            //         .wrap(RoleGuard(&["Master"]))
            //         .wrap(auth())
            //         .route(web::post().to(handler::...)),
            // )
            .service(
                web::resource("/delete/{id}")
                    .wrap(RoleGuard(&["User", "Master"]))
                    .wrap(auth())
                    .route(web::delete().to(handler::delete::delete_image)),
            )
            .service(
                web::resource("/upload/{entity}")
                    .wrap(RoleGuard(&["User", "Master"]))
                    .wrap(auth())
                    .route(web::post().to(handler::upload::save_file)),
            ),
    );
}
