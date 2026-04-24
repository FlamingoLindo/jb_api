use actix_web::{HttpResponse, Responder, web};
use log::{error, warn};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

use crate::{
    dto::users::update::{UpdateUserDTO, UpdateUserResponse},
    entities::users,
};

pub async fn update_user(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
    user: web::Json<UpdateUserDTO>,
) -> impl Responder {
    if let Err(errors) = user.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    let (current_user, existing_user) = tokio::join!(
        users::Entity::find_by_id(*id).one(db.get_ref()),
        users::Entity::find()
            .filter(users::Column::Email.eq(&user.email))
            .filter(users::Column::Id.ne(*id))
            .one(db.get_ref())
    );

    match current_user {
        Ok(None) => {
            warn!("(update_user) User not found: {}", id);
            return HttpResponse::NotFound().json(json!({
                "status": "Not Found",
                "message": "User not found"
            }));
        }
        Err(err) => {
            error!("(update_user) Database error: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Error when finding User, please try again later"
            }));
        }
        Ok(Some(_)) => {}
    }

    match existing_user {
        Ok(Some(_)) => {
            warn!("(update_user) User with same email already exists");
            return HttpResponse::Conflict().json(json!({
                "status": "Conflict",
                "message": "User email already in use"
            }));
        }
        Err(err) => {
            error!("(update_user) Could not check User email: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Error when finding user, please try again later"
            }));
        }
        Ok(None) => {}
    }

    let user = user.into_inner();
    let updated = users::ActiveModel {
        id: Set(*id),
        username: Set(user.username),
        email: Set(Some(user.email)),
        updated_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    }
    .update(db.get_ref())
    .await;

    match updated {
        Ok(updated_user) => HttpResponse::Ok().json(UpdateUserResponse::from(updated_user)),
        Err(err) => {
            error!("(update_user) Could not update user: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Error when updating user, please try again later"
            }))
        }
    }
}
