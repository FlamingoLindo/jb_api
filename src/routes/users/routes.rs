use crate::middlewares::role::RoleGuard;
use crate::{handlers::users as handler, middlewares::auth::validator};
use actix_web::web::{self};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config(cfg: &mut web::ServiceConfig) {
    let auth = || HttpAuthentication::bearer(validator);

    cfg.service(
        web::scope("/users")
            .route("/login", web::post().to(handler::login::login))
            .route(
                "/send-password-email",
                web::post().to(handler::send_forgot_password::send_forgot_password),
            )
            // Master
            .service(
                web::resource("/reset-password/{id}")
                    .wrap(RoleGuard(&["Master"]))
                    .wrap(auth())
                    .route(web::patch().to(handler::reset_password::reset_password)),
            )
            .service(
                web::resource("/register")
                    .wrap(RoleGuard(&["Master"]))
                    .wrap(auth())
                    .route(web::post().to(handler::create::create_user)),
            )
            .service(
                web::resource("/status/{id}")
                    .wrap(RoleGuard(&["Master"]))
                    .wrap(auth())
                    .route(web::patch().to(handler::block::block_user)),
            )
            .service(
                web::resource("/export")
                    .wrap(RoleGuard(&["Master"]))
                    .wrap(auth())
                    .route(web::post().to(handler::export::export_users)),
            )
            .service(
                web::resource("/delete/{id}")
                    .wrap(RoleGuard(&["Master"]))
                    .wrap(auth())
                    .route(web::delete().to(handler::delete::delete_user)),
            )
            .service(
                web::resource("/{id}")
                    .wrap(RoleGuard(&["Master"]))
                    .wrap(auth())
                    .route(web::patch().to(handler::update::update_user)),
            ),
        // User
    );
}
