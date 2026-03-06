use crate::{handlers::users as handler, middlewares::auth::validator};
use actix_web::web::{self};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(validator);

    cfg.service(
        web::scope("/users")
            .route("/login", web::post().to(handler::login_user::login))
            .service(
                web::scope("")
                    .wrap(auth)
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
                    .route(
                        "/register",
                        web::post().to(handler::create_user::create_user),
                    ),
            ),
    );
}
