use crate::middlewares::role::RoleGuard;
use crate::{handlers::clients as handler, middlewares::auth::validator};
use actix_web::web::{self};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config(cfg: &mut web::ServiceConfig) {
    let auth = || HttpAuthentication::bearer(validator);

    cfg.service(
        web::scope("/clients")
            // Master-only routes (empty for now)
            // .service(
            //     web::resource("/some-master-route")
            //         .wrap(RoleGuard(&["Master"]))
            //         .wrap(auth())
            //         .route(web::post().to(handler::...)),
            // )
            .service(
                web::resource("/register")
                    .wrap(RoleGuard(&["User", "Master"]))
                    .wrap(auth())
                    .route(web::post().to(handler::create::create_client)),
            )
            .service(
                web::resource("/available")
                    .wrap(RoleGuard(&["User", "Master"]))
                    .wrap(auth())
                    .route(web::get().to(handler::available::available_clients)),
            )
            .service(
                web::resource("/block/{id}")
                    .wrap(RoleGuard(&["User", "Master"]))
                    .wrap(auth())
                    .route(web::patch().to(handler::block::block_client)),
            )
            .service(
                web::resource("/{id}")
                    .wrap(RoleGuard(&["User", "Master"]))
                    .wrap(auth())
                    .route(web::get().to(handler::get::get_client))
                    .route(web::patch().to(handler::update::update_client))
                    .route(web::delete().to(handler::delete::delete_client)),
            )
            .service(
                web::resource("")
                    .wrap(RoleGuard(&["User", "Master"]))
                    .wrap(auth())
                    .route(web::get().to(handler::get_all::get_clients)),
            ),
    );
}
