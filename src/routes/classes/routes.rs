use crate::middlewares::role::RoleGuard;
use crate::{handlers::classes as handler, middlewares::auth::validator};
use actix_web::web::{self};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config(cfg: &mut web::ServiceConfig) {
    let auth = || HttpAuthentication::bearer(validator);

    cfg.service(
        web::scope("/classes")
            // Master-only routes (empty for now)
            // .service(
            //     web::resource("/some-master-route")
            //         .wrap(RoleGuard(&["Master"]))
            //         .wrap(auth())
            //         .route(web::post().to(handler::...)),
            // )
            .service(
                web::resource("/create")
                    .wrap(RoleGuard(&["User", "Master"]))
                    .wrap(auth())
                    .route(web::post().to(handler::create::create_class)),
            )
            .service(
                web::resource("/status/{id}")
                    .wrap(RoleGuard(&["User", "Master"]))
                    .wrap(auth())
                    .route(web::patch().to(handler::block::block_class)),
            )
            .service(
                web::resource("/{id}")
                    .wrap(RoleGuard(&["User", "Master"]))
                    .wrap(auth())
                    .route(web::get().to(handler::get::get_class))
                    .route(web::patch().to(handler::update::update_class))
                    .route(web::delete().to(handler::delete::delete_class)),
            )
            .service(
                web::resource("")
                    .wrap(RoleGuard(&["User", "Master"]))
                    .wrap(auth())
                    .route(web::get().to(handler::get_all::get_classes)),
            ),
    );
}
