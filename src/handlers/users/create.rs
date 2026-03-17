use actix_web::{HttpResponse, Responder, error::ErrorInternalServerError, web};
use log::error;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;
use validator::Validate;

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};

use crate::{
    dto::users::create_user::{CreateUserDTO, CreateUserResponseDTO},
    entities::users,
};

pub async fn create_user(
    db: web::Data<DatabaseConnection>,
    user: web::Json<CreateUserDTO>,
) -> impl Responder {
    // Check if theres an existing user with the same username
    let existing_user = users::Entity::find()
        .filter(users::Column::Username.eq(&user.username))
        .one(db.get_ref())
        .await;

    match existing_user {
        Ok(Some(_)) => {
            return Ok(HttpResponse::Conflict().json(serde_json::json!({
                "status": "Conflict",
                "message": "Username already taken"
            })));
        }
        Err(err) => {
            error!("(create_user) Could not find user by username: {:?}", err);
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "Internal Server Error",
                "message": "There has been an error when finding user, please try again"
            })));
        }
        Ok(None) => {}
    }

    if let Err(errors) = user.validate() {
        return Ok(HttpResponse::BadRequest().json(errors));
    }

    // Hash user's password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = match argon2.hash_password(user.password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(err) => {
            error!("(create_user) Could not hash user password: {:?}", err);
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "Internal Server Error",
                "message": "There has been an error when processing password, please try again"
            })));
        }
    };

    // Remove case sensitivity for login
    let lower_username = user.username.to_lowercase();

    // Create user
    let new_user = users::ActiveModel {
        id: Set(Uuid::new_v4()),
        username: Set(lower_username),
        password: Set(password_hash),
        blocked: Set(user.blocked),
        ..Default::default()
    }
    .insert(db.get_ref())
    .await;

    match new_user {
        Ok(user) => Ok(HttpResponse::Created().json(CreateUserResponseDTO::from(user))),
        Err(err) => Err(ErrorInternalServerError(err)),
    }
}
