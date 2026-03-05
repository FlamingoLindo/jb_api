use crate::handlers::users as handler;
use actix_web::web::{self};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route(
                "/register",
                web::post().to(handler::create_user::create_user),
            )
            .route(
                "/delete/{id}",
                web::delete().to(handler::delete_user::delete_user),
            )
            .route(
                "/status/{id}",
                web::patch().to(handler::block_user::block_user),
            )
            .route(
                "/export",
                web::post().to(handler::export_users::export_users),
            )
            .route("/login", web::post().to(handler::login_user::login)),
    );
}
