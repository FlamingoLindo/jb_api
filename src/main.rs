mod config;
mod database;
mod dto;
mod entities;
mod handlers;
mod jobs;
mod mailer;
mod middlewares;
mod routes;
use std::sync::Arc;

use actix_cors::Cors;
use actix_multipart::form::tempfile::TempFileConfig;
use actix_web::{
    App, HttpServer,
    http::header,
    middleware::{self, Logger},
    web,
};
use routes::config::config;

use crate::database::connect_to_db::connect_to_db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    config::load_env();
    env_logger::init();

    let port = std::env::var("PORT").expect("PORT must be set in env");

    let db_conn = connect_to_db()
        .await
        .expect("Failed to connect to database");

    let db_arc = Arc::new(db_conn.clone());
    tokio::spawn(jobs::scheduler::start(db_arc));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(db_conn.clone()))
            .app_data(TempFileConfig::default().directory("./uploads/.temp"))
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Trim,
            ))
            .configure(config)
    })
    .bind(port)?
    .run()
    .await
}
