use crate::entities::users;
use actix_web::{HttpResponse, Responder, web};
use log::{error, warn};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Serialize;
use serde_json::json;
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct BlockUserResponse {
    pub username: String,
    pub blocked: bool,
}

#[utoipa::path(
    patch,
    path = "/api/v1/users/status/{id}",
    tag = "Users",
    params(("id" = Uuid, Path, description = "User ID")),
    responses(
        (status = 200, description = "User block status toggled", body = BlockUserResponse),
        (status = 404, description = "User not found", body = serde_json::Value),
        (status = 500, description = "Internal server error", body = serde_json::Value)
    )
)]
pub async fn block_user(db: web::Data<DatabaseConnection>, id: web::Path<Uuid>) -> impl Responder {
    let existing_user = users::Entity::find()
        .filter(users::Column::Id.eq(*id))
        .one(db.get_ref())
        .await;

    let user = match existing_user {
        Ok(Some(user)) => user,
        Ok(None) => {
            warn!("(block_user) User not found");
            return HttpResponse::NotFound().json(json!({
                "status": "Not Found",
                "message": "User not found"
            }));
        }
        Err(err) => {
            error!("(block_user) Could not find user: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when finding user, please try again"
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
        Err(err) => {
            error!("(block_user) Could not update user block status: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when updating user, please try again"
            }))
        }
    }
}
