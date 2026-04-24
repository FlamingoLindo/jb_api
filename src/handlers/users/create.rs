use actix_web::{HttpResponse, Responder, error::ErrorInternalServerError, web};
use log::{error, warn};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};

use crate::{
    dto::users::create_user::{CreateUserDTO, CreateUserResponseDTO},
    entities::{roles, users},
};

pub async fn create_user(
    db: web::Data<DatabaseConnection>,
    user: web::Json<CreateUserDTO>,
) -> impl Responder {
    if let Err(errors) = user.validate() {
        return Ok(HttpResponse::BadRequest().json(errors));
    }

    // Check if theres an existing user with the same email
    let existing_user = users::Entity::find()
        .filter(users::Column::Email.eq(&user.email.to_lowercase()))
        .one(db.get_ref())
        .await;

    match existing_user {
        Ok(Some(_)) => {
            warn!("(create_user) User not found");
            return Ok(HttpResponse::Conflict().json(json!({
                "status": "Conflict",
                "message": "Invalid data"
            })));
        }
        Err(err) => {
            error!("(create_user) Could not find user by email: {:?}", err);
            return Ok(HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when finding user, please try again"
            })));
        }
        Ok(None) => {}
    }

    // Hash user's password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = match argon2.hash_password(user.password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(err) => {
            error!("(create_user) Could not hash user password: {:?}", err);
            return Ok(HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when processing password, please try again"
            })));
        }
    };

    // Get role
    let master_role = match roles::Entity::find()
        .filter(roles::Column::Title.eq("User"))
        .one(db.get_ref())
        .await
    {
        Ok(Some(role)) => role,
        Ok(None) => {
            warn!("(create_user) Role not found");
            return Ok(HttpResponse::NotFound().json(json!({
                "status": "Not Found",
                "message": "Role not found"
            })));
        }
        Err(err) => {
            error!("(create_user) Could not find master role: {:?}", err);
            return Ok(HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when getting user's role, please try again"
            })));
        }
    };

    // Create user
    let user_data = user.into_inner();
    let new_user = users::ActiveModel {
        id: Set(Uuid::new_v4()),
        username: Set(user_data.username),
        email: Set(Some(user_data.email.to_lowercase())), // This should not need Some(), but since the migration came out wrong...
        password: Set(password_hash),
        blocked: Set(user_data.blocked),
        role_id: Set(Some(master_role.id)),
        ..Default::default()
    }
    .insert(db.get_ref())
    .await;

    match new_user {
        Ok(user) => Ok(HttpResponse::Created().json(CreateUserResponseDTO::from(user))),
        Err(err) => Err(ErrorInternalServerError(err)),
    }
}
