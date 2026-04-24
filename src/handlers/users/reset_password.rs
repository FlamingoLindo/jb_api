use actix_web::{HttpResponse, Responder, web};
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use log::{error, warn};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

use crate::{dto::users::reset_password::ResetPasswordDTO, entities::users};

pub async fn reset_password(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
    data: web::Json<ResetPasswordDTO>,
) -> impl Responder {
    if let Err(errors) = data.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    let existing_user = match users::Entity::find()
        .filter(users::Column::Id.eq(*id))
        .one(db.get_ref())
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            warn!("(reset_password) User not found");
            return HttpResponse::NotFound().json(json!({
                "status": "Not Found",
                "message": "User not found"
            }));
        }
        Err(err) => {
            error!("(reset_password) Could not find user: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when finding user, please try again"
            }));
        }
    };

    let current_password = match PasswordHash::new(&existing_user.password) {
        Ok(hash) => hash,
        Err(err) => {
            error!("(reset_password) Could not parse password hash: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when processing password, please try again"
            }));
        }
    };

    if Argon2::default()
        .verify_password(data.new_password.as_bytes(), &current_password)
        .is_ok()
    {
        warn!("(reset_password) Same password");
        return HttpResponse::BadRequest().json(json!({
            "status": "Bad Request",
            "message": "Invalid data"
        }));
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = match argon2.hash_password(data.new_password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(err) => {
            error!("(reset_password) Could not hash new_password: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when processing new password, please try again"
            }));
        }
    };

    let updated = users::ActiveModel {
        id: Set(existing_user.id),
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
            error!("(reset_password) Could not update user: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Error when updating user, please try again later"
            }))
        }
    }
}
