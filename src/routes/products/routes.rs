use crate::middlewares::role::RoleGuard;
use crate::{handlers::products as handler, middlewares::auth::validator};
use actix_web::web::{self};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config(cfg: &mut web::ServiceConfig) {
    let auth = || HttpAuthentication::bearer(validator);

    cfg.service(
        web::scope("/products")
            // Master-only routes (empty for now)
            // .service(
            //     web::resource("/some-master-route")
            //         .wrap(RoleGuard(&["Master"]))
            //         .wrap(auth())
            //         .route(web::post().to(handler::...)),
            // )
            // Specific static routes first (before /{id} wildcard)
            .service(
                web::resource("/create")
                    .wrap(RoleGuard(&["User", "Master"]))
                    .wrap(auth())
                    .route(web::post().to(handler::create::create_product)),
            )
            .service(
                web::resource("/image-bind")
                    .wrap(RoleGuard(&["User", "Master"]))
                    .wrap(auth())
                    .route(web::post().to(handler::images::bind::bind_product_to_image)),
            )
            .service(
                web::resource("/readjust-prices")
                    .wrap(RoleGuard(&["User", "Master"]))
                    .wrap(auth())
                    .route(web::patch().to(handler::readjust_price::readjust_price)),
            )
            .service(
                web::resource("/block/{id}")
                    .wrap(RoleGuard(&["User", "Master"]))
                    .wrap(auth())
                    .route(web::patch().to(handler::block::block_product)),
            )
            .service(
                web::resource("/image-bind/{id}")
                    .wrap(RoleGuard(&["User", "Master"]))
                    .wrap(auth())
                    .route(web::delete().to(handler::images::delete::delete_bind)),
            )
            // Wildcard /{id} routes
            .service(
                web::resource("/{id}")
                    .wrap(RoleGuard(&["User", "Master"]))
                    .wrap(auth())
                    .route(web::get().to(handler::get::get_product))
                    .route(web::patch().to(handler::update::update_product))
                    .route(web::delete().to(handler::delete::delete_product)),
            )
            // Root route last
            .service(
                web::resource("")
                    .wrap(RoleGuard(&["User", "Master"]))
                    .wrap(auth())
                    .route(web::get().to(handler::get_all::get_products)),
            ),
    );
}
