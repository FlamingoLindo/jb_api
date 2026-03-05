mod dto;
mod entities;
mod handlers;
mod routes;
use actix_cors::Cors;
use actix_web::{App, HttpServer, http::header, middleware, web};
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::time::Duration;

use migration::{Migrator, MigratorTrait};

use routes::config::config;

// TODO add roles table
// TODO add this function into a mod of its own
pub async fn connect_to_db() -> Result<DatabaseConnection, DbErr> {
    dotenv::from_filename(".env").ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in env");

    let mut opt = ConnectOptions::new(database_url);

    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info);

    let db_conn = Database::connect(opt).await?;

    Migrator::up(&db_conn, None).await?;

    Ok(db_conn)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_filename(".env").ok();
    let port = std::env::var("PORT").expect("PORT must be set in env");

    let db_conn = connect_to_db()
        .await
        .expect("Failed to connect to database");

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
            .wrap(cors)
            .wrap(middleware::Logger::default())
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
