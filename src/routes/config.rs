use actix_web::web;

use crate::routes;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(routes::users::routes::config)
            .configure(routes::brands::routes::config)
            .configure(routes::classes::routes::config)
            .configure(routes::images::routes::config)
            .configure(routes::types::routes::config)
            .configure(routes::products::routes::config)
            .configure(routes::budget::routes::config),
    );
}
