use actix_web::{HttpResponse, Responder, web};
use log::warn;
use sea_orm::{DatabaseConnection, EntityTrait};
use uuid::Uuid;

use crate::{dto::types::get::TypeResponse, entities::types};

pub async fn get_type(db: web::Data<DatabaseConnection>, id: web::Path<Uuid>) -> impl Responder {
    let id = id.into_inner();

    let found_type = types::Entity::find_by_id(id).one(db.get_ref()).await;

    match found_type {
        Ok(Some(found_type)) => {
            let dto = TypeResponse::from(found_type);
            HttpResponse::Ok().json(dto)
        }
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "status": "Not Found",
            "message": "Type not found"
        })),
        Err(err) => {
            warn!("(get_type) Could not get type data: {:?}", err);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "Internal Server Error",
                "message": "Something went wrong when retrieving type data"
            }))
        }
    }
}
