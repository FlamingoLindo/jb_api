use crate::middlewares::role::RoleGuard;
use crate::{handlers::products as handler, middlewares::auth::validator};
use actix_web::web::{self};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(validator);

    cfg.service(
        web::scope("/products").service(
            web::scope("").wrap(auth).service(
                web::scope("")
                    .wrap(RoleGuard("user"))
                    .route("/create", web::post().to(handler::create::create_product))
                    .route("", web::get().to(handler::get_all::get_products))
                    .route(
                        "/image-bind",
                        web::post().to(handler::images::bind::bind_product_to_image),
                    )
                    .route(
                        "/readjust-prices",
                        web::patch().to(handler::readjust_price::readjust_price),
                    )
                    .route("/{id}", web::get().to(handler::get::get_product))
                    .route("/{id}", web::patch().to(handler::update::update_product))
                    .route("/{id}", web::delete().to(handler::delete::delete_product))
                    .route(
                        "/block/{id}",
                        web::patch().to(handler::block::block_product),
                    )
                    .route(
                        "/image-bind/{id}",
                        web::delete().to(handler::images::delete::delete_bind),
                    ),
            ),
        ),
    );
}
