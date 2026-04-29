use actix_web::{HttpResponse, Responder, web};
use log::{error, warn};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Serialize;
use serde_json::json;
use uuid::Uuid;
use utoipa::ToSchema;

use crate::entities::types;

#[derive(Serialize, ToSchema)]
pub struct BlockTypeResponse {
    pub name: String,
    pub blocked: bool,
}

#[utoipa::path(
    patch,
    path = "/api/v1/types/block/{id}",
    tag = "Types",
    params(("id" = Uuid, Path, description = "Type ID")),
    responses(
        (status = 200, description = "Type block status toggled", body = BlockTypeResponse),
        (status = 404, description = "Type not found", body = serde_json::Value),
        (status = 500, description = "Internal server error", body = serde_json::Value)
    )
)]
pub async fn block_type(db: web::Data<DatabaseConnection>, id: web::Path<Uuid>) -> impl Responder {
    let existing_type = types::Entity::find()
        .filter(types::Column::Id.eq(*id))
        .one(db.get_ref())
        .await;

    let blocked_type = match existing_type {
        Ok(Some(blocked_type)) => blocked_type,
        Ok(None) => {
            warn!("(block_type) Type not found");
            return HttpResponse::NotFound().json(json!({
                "status": "Not Found",
                "message": "Type not found"
            }));
        }
        Err(err) => {
            error!("(block_type) Could not find type: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when finding type, please try again"
            }));
        }
    };

    let mut active_type: types::ActiveModel = blocked_type.into();
    active_type.blocked = Set(!active_type.blocked.unwrap());

    match active_type.update(db.get_ref()).await {
        Ok(updated_type) => HttpResponse::Ok().json(BlockTypeResponse {
            name: updated_type.name,
            blocked: updated_type.blocked,
        }),
        Err(err) => {
            error!("(block_type) Could not update type block status: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when updating type, please try again"
            }))
        }
    }
}
