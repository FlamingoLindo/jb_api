use crate::middlewares::role::RoleGuard;
use crate::{handlers::budgets as handler, middlewares::auth::validator};
use actix_web::web::{self};
use actix_web_httpauth::middleware::HttpAuthentication;

pub fn config(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(validator);

    cfg.service(
        web::scope("/budgets").service(
            web::scope("").wrap(auth).service(
                web::scope("")
                    .wrap(RoleGuard("master"))
                    .route("/create", web::post().to(handler::create::create_budget))
                    .route("/{id}", web::delete().to(handler::delete::delete_budget))
                    .route(
                        "/count/{id}",
                        web::get().to(handler::count::count_client_budgets),
                    )
                    .route(
                        "/{id}",
                        web::get().to(handler::get_per_client::get_all_budgets_per_client),
                    ),
            ),
        ),
    );
}
