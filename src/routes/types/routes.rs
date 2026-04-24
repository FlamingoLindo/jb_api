use crate::middlewares::role::RoleGuard;
use crate::{handlers::types as handler, middlewares::auth::validator};
use actix_web::web::{self};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config(cfg: &mut web::ServiceConfig) {
    let auth = || HttpAuthentication::bearer(validator);

    cfg.service(
        web::scope("/types")
            // Master-only routes (empty for now)
            // .service(
            //     web::resource("/some-master-route")
            //         .wrap(RoleGuard(&["Master"]))
            //         .wrap(auth())
            //         .route(web::post().to(handler::...)),
            // )
            .service(
                web::resource("/block/{id}")
                    .wrap(RoleGuard(&["User", "Master"]))
                    .wrap(auth())
                    .route(web::patch().to(handler::block::block_type)),
            )
            .service(
                web::resource("/create")
                    .wrap(RoleGuard(&["User", "Master"]))
                    .wrap(auth())
                    .route(web::post().to(handler::create::create_type)),
            )
            .service(
                web::resource("/{id}")
                    .wrap(RoleGuard(&["User", "Master"]))
                    .wrap(auth())
                    .route(web::get().to(handler::get::get_type))
                    .route(web::patch().to(handler::update::update_type))
                    .route(web::delete().to(handler::delete::delete_type)),
            )
            .service(
                web::resource("")
                    .wrap(RoleGuard(&["User", "Master"]))
                    .wrap(auth())
                    .route(web::get().to(handler::get_all::get_types)),
            ),
    );
}
