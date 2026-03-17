use actix_web::{HttpResponse, Responder, web};
use log::warn;
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait};
use uuid::Uuid;

use crate::entities::images;

pub async fn delete_image(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let existing_image = images::Entity::find_by_id(id.into_inner())
        .one(db.get_ref())
        .await;

    let image = match existing_image {
        Ok(Some(image)) => image,
        Ok(None) => {
            return HttpResponse::NotFound().json(serde_json::json!({
                "status": "Not Found",
                "message": "Image not found"
            }));
        }
        Err(err) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "Internal Server Error",
                "message": err.to_string()
            }));
        }
    };

    if let Err(err) = std::fs::remove_file(&image.path) {
        warn!("Could not delete file from disk: {:?}", err);
        return HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "Internal Server Error",
            "message": "Something went wrong when deleting image"
        }));
    }

    match image.delete(db.get_ref()).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "status": "Ok",
            "message": "Image deleted successfully"
        })),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "Internal Server Error",
            "message": err.to_string()
        })),
    }
}
