use actix_web::web;

use crate::routes;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(routes::users::users::config)
            .configure(routes::brands::brands::config)
            .configure(routes::classes::classes::config)
            .configure(routes::images::images::config)
            .configure(routes::types::types::config)
            .configure(routes::products::products::config),
    );
}
