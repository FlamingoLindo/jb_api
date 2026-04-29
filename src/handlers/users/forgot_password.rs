use actix_web::{HttpResponse, Responder, web};
use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use log::error;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter, Set,
};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

use crate::{
    dto::users::forgot_password::ForgotPasswordDTO,
    entities::{reset_password_tokens, users},
};

#[utoipa::path(
    post,
    path = "/api/v1/users/forgot-password/{token}",
    tag = "Users",
    params(("token" = String, Path, description = "Password reset token")),
    request_body = ForgotPasswordDTO,
    responses(
        (status = 200, description = "Password reset successfully", body = serde_json::Value),
        (status = 400, description = "Validation error", body = serde_json::Value),
        (status = 404, description = "Token not found", body = serde_json::Value),
        (status = 500, description = "Internal server error", body = serde_json::Value)
    )
)]
pub async fn forgot_password(
    db: web::Data<DatabaseConnection>,
    token: web::Path<Uuid>,
    data: web::Json<ForgotPasswordDTO>,
) -> impl Responder {
    if let Err(errors) = data.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    let reset_token = match reset_password_tokens::Entity::find()
        .filter(reset_password_tokens::Column::Token.eq(*token))
        .one(db.get_ref())
        .await
    {
        Ok(Some(reset_token)) => reset_token,
        Ok(None) => {
            return HttpResponse::NotFound().json(json!({
                "status": "Not Found",
                "message": "Reset token is invalid or has expired"
            }));
        }
        Err(err) => {
            error!("(forgot_password) Could not find reset token: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when finding reset token, please try again"
            }));
        }
    };

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = match argon2.hash_password(data.new_password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(err) => {
            error!("(forgot_password) Could not hash new_password: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when processing new password, please try again"
            }));
        }
    };

    let updated = users::ActiveModel {
        id: Set(reset_token.user_id),
        token: Set(None),
        password: Set(password_hash),
        ..Default::default()
    }
    .update(db.get_ref())
    .await;

    match updated {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "Updated",
            "message": "The new password has been set"
        })),
        Err(err) => {
            error!("(forgot_password) Could not update user: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Error when updating user, please try again later"
            }));
        }
    };

    match reset_token.delete(db.get_ref()).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "Ok",
            "message": "Your password has ben reset"
        })),
        Err(err) => {
            error!("(forgot_password) Could not delete token: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when deleting token, please try again"
            }));
        }
    }
}
