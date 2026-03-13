use crate::entities::users;
use actix_web::{HttpResponse, Responder, web};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct BlockUserResponse {
    pub username: String,
    pub blocked: bool,
}

pub async fn block_user(db: web::Data<DatabaseConnection>, id: web::Path<Uuid>) -> impl Responder {
    let existing_user = users::Entity::find()
        .filter(users::Column::Id.eq(*id))
        .one(db.get_ref())
        .await;

    let user = match existing_user {
        Ok(Some(user)) => user,
        Ok(None) => {
            return HttpResponse::NotFound().json(serde_json::json!({
                "status": "Not Found",
                "message": "User not found"
            }));
        }
        Err(err) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "Internal Server Error",
                "message": err.to_string()
            }));
        }
    };

    let mut active_user: users::ActiveModel = user.into();
    active_user.blocked = Set(!active_user.blocked.unwrap());

    match active_user.update(db.get_ref()).await {
        Ok(updated_user) => HttpResponse::Ok().json(BlockUserResponse {
            username: updated_user.username,
            blocked: updated_user.blocked,
        }),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "Internal Server Error",
            "message": err.to_string()
        })),
    }
}
