use actix_web::{HttpResponse, Responder, web};
use log::{error, warn};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Serialize;
use serde_json::json;
use uuid::Uuid;
use utoipa::ToSchema;

use crate::entities::classes;

#[derive(Serialize, ToSchema)]
pub struct BlockClassResponse {
    pub name: String,
    pub blocked: bool,
}

#[utoipa::path(
    patch,
    path = "/api/v1/classes/status/{id}",
    tag = "Class",
    params(("id" = Uuid, Path, description = "Class id")),
    responses(
        (status = 200, description = "Class block status toggled", body = BlockClassResponse),
        (status = 404, description = "Class not found"),
        (status = 500, description = "Internal server error"),
    )
)]

pub async fn block_class(db: web::Data<DatabaseConnection>, id: web::Path<Uuid>) -> impl Responder {
    let existing_class = classes::Entity::find()
        .filter(classes::Column::Id.eq(*id))
        .one(db.get_ref())
        .await;

    let class = match existing_class {
        Ok(Some(class)) => class,
        Ok(None) => {
            warn!("(block_class) Class not found");
            return HttpResponse::NotFound().json(json!({
                "status": "Not Found",
                "message": "Class not found"
            }));
        }
        Err(err) => {
            error!("(block_class) Could not find class: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
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
            error!(
                "(block_class) Could not update class block status: {:?}",
                err
            );
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when updating class, please try again"
            }))
        }
    }
}
