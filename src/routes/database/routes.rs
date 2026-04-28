use crate::middlewares::role::RoleGuard;
use crate::{handlers::database as handler, middlewares::auth::validator};
use actix_web::web::{self};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config(cfg: &mut web::ServiceConfig) {
    let auth = || HttpAuthentication::bearer(validator);

    cfg.service(
        web::scope("/database").service(
            web::resource("/dump")
                .wrap(RoleGuard(&["Master", "DPO"]))
                .wrap(auth())
                .route(web::post().to(handler::dump::create_db_dump)),
        ),
    );
}
