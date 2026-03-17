use actix_web::{HttpResponse, Responder, web};
use log::warn;
use sea_orm::{DatabaseConnection, EntityTrait};
use uuid::Uuid;

use crate::{dto::classes::get::ClassResponse, entities::classes};

pub async fn get_class(db: web::Data<DatabaseConnection>, id: web::Path<Uuid>) -> impl Responder {
    let found_class = classes::Entity::find_by_id(id.into_inner())
        .one(db.get_ref())
        .await;

    match found_class {
        Ok(Some(class)) => {
            let dto = ClassResponse::from(class);
            HttpResponse::Ok().json(dto)
        }
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "status": "Not Found",
            "message": "Class not found"
        })),
        Err(err) => {
            warn!("Could not get class data: {:?}", err);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "Internal Server Error",
                "message": "Something went wrong when retrieving class data"
            }))
        }
    }
}
