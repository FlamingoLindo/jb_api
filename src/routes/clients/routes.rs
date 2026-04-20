use crate::middlewares::role::RoleGuard;
use crate::{handlers::clients as handler, middlewares::auth::validator};
use actix_web::web::{self};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(validator);

    cfg.service(
        web::scope("/clients").service(
            web::scope("").wrap(auth).service(
                web::scope("")
                    .wrap(RoleGuard("master"))
                    .route("/register", web::post().to(handler::create::create_client))
                    .route("", web::get().to(handler::get_all::get_clients)),
            ),
        ),
    );
}
