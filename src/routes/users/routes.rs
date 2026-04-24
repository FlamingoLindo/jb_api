use crate::middlewares::role::RoleGuard;
use crate::{handlers::users as handler, middlewares::auth::validator};
use actix_web::web::{self};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(validator);

    cfg.service(
        web::scope("/users")
            .route("/login", web::post().to(handler::login::login))
            .service(
                web::scope("").wrap(auth).service(
                    web::scope("")
                        .wrap(RoleGuard("user"))
                        .route("/register", web::post().to(handler::create::create_user))
                        .route(
                            "/delete/{id}",
                            web::delete().to(handler::delete::delete_user),
                        )
                        .route("/status/{id}", web::patch().to(handler::block::block_user))
                        .route("/export", web::post().to(handler::export::export_users))
                        .route("/{id}", web::patch().to(handler::update::update_user)),
                ),
            ),
    );
}
