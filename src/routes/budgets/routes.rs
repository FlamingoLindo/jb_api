use crate::middlewares::role::RoleGuard;
use crate::{handlers::budgets as handler, middlewares::auth::validator};
use actix_web::web::{self};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config(cfg: &mut web::ServiceConfig) {
    let auth = || HttpAuthentication::bearer(validator);

    cfg.service(
        web::scope("/budgets")
            // Master-only routes (empty for now)
            // .service(
            //     web::resource("/some-master-route")
            //         .wrap(RoleGuard(&["Master"]))
            //         .wrap(auth())
            //         .route(web::post().to(handler::...)),
            // )
            .service(
                web::resource("/create")
                    .wrap(RoleGuard(&["User", "Master"]))
                    .wrap(auth())
                    .route(web::post().to(handler::create::create_budget)),
            )
            .service(
                web::resource("/count/{id}")
                    .wrap(RoleGuard(&["User", "Master"]))
                    .wrap(auth())
                    .route(web::get().to(handler::count::count_client_budgets)),
            )
            .service(
                web::resource("/{id}")
                    .wrap(RoleGuard(&["User", "Master"]))
                    .wrap(auth())
                    .route(web::get().to(handler::get_per_client::get_all_budgets_per_client))
                    .route(web::delete().to(handler::delete::delete_budget)),
            ),
    );
}
