use crate::entities::roles;
use crate::{
    dto::users::login_user::LoginDTO, entities::users, entities::users::Model as UsersModel,
};
use actix_web::{HttpResponse, error::ErrorInternalServerError, web};
use argon2::password_hash::PasswordHash;
use argon2::{Argon2, PasswordVerifier};
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use log::{error, warn};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    role: Option<String>,
    iat: usize,
    exp: usize,
}

fn gen_jwt(
    user: &UsersModel,
    role_name: Option<String>,
) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();
    let claims = Claims {
        sub: user.email.clone().expect("Missing user email on claim"),
        role: role_name,
        exp: exp as usize,
        iat: Utc::now().timestamp() as usize,
    };
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}

#[utoipa::path(
    post,
    path = "/api/v1/users/login",
    tag = "Users",
    request_body = LoginDTO,
    responses(
        (status = 200, description = "Login successful", body = serde_json::Value),
        (status = 400, description = "Validation error", body = serde_json::Value),
        (status = 401, description = "Invalid credentials", body = serde_json::Value),
        (status = 500, description = "Internal server error", body = serde_json::Value)
    )
)]
pub async fn login(
    db: web::Data<DatabaseConnection>,
    credential: web::Json<LoginDTO>,
) -> actix_web::Result<HttpResponse> {
    if let Err(errors) = credential.validate() {
        return Ok(HttpResponse::BadRequest().json(errors));
    }

    let existing_user = users::Entity::find()
        .filter(users::Column::Email.eq(&credential.email.to_lowercase()))
        .find_also_related(roles::Entity)
        .one(db.get_ref())
        .await;

    match existing_user {
        Ok(Some((user, role))) => {
            if user.blocked {
                warn!("(login) Unauthorized login attempt");
                return Ok(HttpResponse::Unauthorized().json(json!({
                    "status": "Unauthorized",
                    "message": "Your account has been blocked by an administrator"
                })));
            }

            let role_name = role.map(|r| r.title);

            let parsed_hash = PasswordHash::new(&user.password)
                .map_err(|e| ErrorInternalServerError(e.to_string()))?;

            match Argon2::default().verify_password(credential.password.as_bytes(), &parsed_hash) {
                Ok(()) => match gen_jwt(&user, role_name) {
                    Ok(token) => {
                        let mut active_user: users::ActiveModel = user.into();
                        active_user.token = Set(Some(token.clone()));
                        active_user
                            .update(db.get_ref())
                            .await
                            .map_err(|e| ErrorInternalServerError(e))?;

                        Ok(HttpResponse::Ok().json(json!({
                            "message": "Login successful",
                            "token": token
                        })))
                    }
                    Err(err) => {
                        error!("(login) Could not generate JWT token: {:?}", err);
                        Ok(HttpResponse::InternalServerError().json(json!({
                            "status": "Internal Server Error",
                            "message": "There has been an error when logging in, please try again"
                        })))
                    }
                },
                Err(_) => Ok(HttpResponse::Unauthorized().json(json!({
                    "status": "Unauthorized",
                    "message": "Invalid credentials"
                }))),
            }
        }
        Ok(None) => Ok(HttpResponse::Unauthorized().json(json!({
            "status": "Unauthorized",
            "message": "Invalid credentials"
        }))),
        Err(err) => Err(ErrorInternalServerError(err)),
    }
}
