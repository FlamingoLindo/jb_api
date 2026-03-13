use actix_web::{HttpResponse, Responder, web};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Serialize;
use uuid::Uuid;

use crate::entities::types;

#[derive(Serialize)]
pub struct BlockTypeResponse {
    pub name: String,
    pub blocked: bool,
}

pub async fn block_type(db: web::Data<DatabaseConnection>, id: web::Path<Uuid>) -> impl Responder {
    let existing_type = types::Entity::find()
        .filter(types::Column::Id.eq(*id))
        .one(db.get_ref())
        .await;

    let blocked_type = match existing_type {
        Ok(Some(blocked_type)) => blocked_type,
        Ok(None) => {
            return HttpResponse::NotFound().json(serde_json::json!({
                "status": "Not Found",
                "message": "Type not found"
            }));
        }
        Err(err) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "Internal Server Error",
                "message": err.to_string()
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
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "Internal Server Error",
            "message": err.to_string()
        })),
    }
}
