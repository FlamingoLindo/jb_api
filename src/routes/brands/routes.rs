use crate::middlewares::role::RoleGuard;
use crate::{handlers::brands as handler, middlewares::auth::validator};
use actix_web::web::{self};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config(cfg: &mut web::ServiceConfig) {
    let auth = || HttpAuthentication::bearer(validator);

    cfg.service(
        web::scope("/brands")
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
                    .route(web::post().to(handler::create::create_brand)),
            )
            .service(
                web::resource("/block/{id}")
                    .wrap(RoleGuard(&["User", "Master"]))
                    .wrap(auth())
                    .route(web::patch().to(handler::block::block_brand)),
            )
            .service(
                web::resource("/image-bind")
                    .wrap(RoleGuard(&["User", "Master"]))
                    .wrap(auth())
                    .route(web::post().to(handler::images::bind::bind_brand_to_image)),
            )
            .service(
                web::resource("/image-bind/{id}")
                    .wrap(RoleGuard(&["User", "Master"]))
                    .wrap(auth())
                    .route(web::delete().to(handler::images::delete::delete_brand_bind)),
            )
            .service(
                web::resource("/{id}")
                    .wrap(RoleGuard(&["User", "Master"]))
                    .wrap(auth())
                    .route(web::get().to(handler::get::get_brand))
                    .route(web::patch().to(handler::update::update_brand))
                    .route(web::delete().to(handler::delete::delete_brand)),
            )
            .service(
                web::resource("")
                    .wrap(RoleGuard(&["User", "Master"]))
                    .wrap(auth())
                    .route(web::get().to(handler::get_all::get_brands)),
            ),
    );
}
