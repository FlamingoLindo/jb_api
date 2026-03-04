use actix_web::{HttpResponse, Responder, error::ErrorInternalServerError, web};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use uuid::Uuid;
use validator::Validate;

use crate::{
    dto::users::create_user::{CreateUserDTO, CreateUserResponseDTO},
    entities::users,
};

pub async fn create_user(
    db: web::Data<DatabaseConnection>,
    user: web::Json<CreateUserDTO>,
) -> impl Responder {
    if let Err(errors) = user.validate() {
        return Ok(HttpResponse::BadRequest().json(errors));
    }

    let new_user = users::ActiveModel {
        id: Set(Uuid::new_v4()),
        username: Set(user.username.to_owned()),
        password: Set(user.password.to_owned()),
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


// TODO Verify if user with the same "username" exists
// TODO hash password with argon2
// TODO add `createdAt` and `updatedAt`