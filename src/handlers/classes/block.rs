use actix_web::{HttpResponse, Responder, web};
use log::error;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Serialize;
use uuid::Uuid;

use crate::entities::classes;

#[derive(Serialize)]
pub struct BlockClassResponse {
    pub name: String,
    pub blocked: bool,
}

pub async fn block_class(db: web::Data<DatabaseConnection>, id: web::Path<Uuid>) -> impl Responder {
    let existing_class = classes::Entity::find()
        .filter(classes::Column::Id.eq(*id))
        .one(db.get_ref())
        .await;

    let class = match existing_class {
        Ok(Some(class)) => class,
        Ok(None) => {
            return HttpResponse::NotFound().json(serde_json::json!({
                "status": "Not Found",
                "message": "Class not found"
            }));
        }
        Err(err) => {
            error!("(block_class) Could not find class: {:?}", err);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "Internal Server Error",
                "message": "There has been an error when finding class, please try again"
            }));
        }
    };

    let mut active_class: classes::ActiveModel = class.into();
    active_class.blocked = Set(!active_class.blocked.unwrap());

    match active_class.update(db.get_ref()).await {
        Ok(updated_class) => HttpResponse::Ok().json(BlockClassResponse {
            name: updated_class.name,
            blocked: updated_class.blocked,
        }),
        Err(err) => {
            error!("(block_class) Could not update class block status: {:?}", err);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "Internal Server Error",
                "message": "There has been an error when updating class, please try again"
            }))
        }
    }
}
