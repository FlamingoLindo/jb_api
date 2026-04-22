use crate::middlewares::role::RoleGuard;
use crate::{handlers::budgets as handler, middlewares::auth::validator};
use actix_web::web::{self};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(validator);

    cfg.service(
        web::scope("/budget").service(
            web::scope("").wrap(auth).service(
                web::scope("")
                    .wrap(RoleGuard("master"))
                    .route("/create", web::post().to(handler::create::create_budget))
                    .route("/{id}", web::delete().to(handler::delete::delete_budget)),
            ),
        ),
    );
}
