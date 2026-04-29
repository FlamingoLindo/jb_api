use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, web};
use chrono::Local;
use log::error;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait,
};
use serde_json::json;
use std::{fs, process::Command};

use crate::{
    entities::{roles, users},
    mailer::mailer::Mailer,
    middlewares::auth::Claims,
};

#[utoipa::path(
    post,
    path = "/api/v1/database/dump",
    tag = "Database",
    responses(
        (status = 200, description = "Database dump created successfully", body = serde_json::Value),
        (status = 401, description = "Unauthorized", body = serde_json::Value),
        (status = 500, description = "Internal server error", body = serde_json::Value)
    )
)]
pub async fn create_db_dump(req: HttpRequest, db: web::Data<DatabaseConnection>) -> impl Responder {
    let claims = match req.extensions().get::<Claims>().cloned() {
        Some(c) => c,
        None => {
            return HttpResponse::Unauthorized().json(json!({
                "status": "Unauthorized",
                "message": "Missing token claims"
            }));
        }
    };
    let requester_email = claims.sub;

    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let file_name = format!("db_dump_{}.sql", timestamp);
    let db_user = std::env::var("POSTGRES_USER").expect("POSTGRES_USER missing");
    let db_host = std::env::var("POSTGRES_HOST").expect("POSTGRES_HOST missing");
    let db_name = std::env::var("POSTGRES_DB").expect("POSTGRES_DB missing");
    let db_password = std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD missing");

    if let Err(err) = fs::create_dir_all("exports/dumps") {
        error!("(create_db_dump) Could not create exports dir: {:?}", err);
        return HttpResponse::InternalServerError().json(json!({
            "status": "Internal Server Error",
            "message": "Could not prepare export directory"
        }));
    }

    let path = format!("exports/dumps/{}", file_name);

    let output = Command::new("pg_dump")
        .env("PGPASSWORD", &db_password)
        .arg("-U")
        .arg(&db_user)
        .arg("-h")
        .arg(&db_host)
        .arg("-d")
        .arg(&db_name)
        .arg("-f")
        .arg(&path)
        .output();

    match output {
        Ok(result) if result.status.success() => {}
        Ok(result) => {
            let stderr = String::from_utf8_lossy(&result.stderr);
            error!("(create_db_dump) pg_dump failed: {}", stderr);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "pg_dump failed"
            }));
        }
        Err(err) => {
            error!("(create_db_dump) Failed to spawn pg_dump: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Could not run pg_dump"
            }));
        }
    }

    let auth_users = match users::Entity::find()
        .join(JoinType::InnerJoin, users::Relation::Roles.def())
        .filter(roles::Column::Title.is_in(["Master", "DPO"]))
        .all(db.get_ref())
        .await
    {
        Ok(users) => users,
        Err(err) => {
            error!(
                "(create_db_dump) Could not find authorized users: {:?}",
                err
            );
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Could not find authorized users"
            }));
        }
    };

    for user in &auth_users {
        let email = match user.email.as_deref() {
            Some(e) => e,
            None => {
                error!(
                    "(create_db_dump) User {:?} has no email, skipping",
                    user.username
                );
                continue;
            }
        };

        if let Err(err) =
            Mailer::send_dump(email, &user.username, &db_name, &requester_email, &path)
        {
            error!(
                "(create_db_dump) Could not send dump email to {}: {:?}",
                email, err
            );
        }
    }

    HttpResponse::Ok().json(json!({
        "status": "Ok",
        "message": "Database dump created!",
    }))
}
