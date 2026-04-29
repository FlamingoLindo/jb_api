use actix_web::{HttpResponse, Responder, web};
use log::{error, warn};
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait};
use serde_json::json;
use uuid::Uuid;

use crate::entities::images;

#[utoipa::path(
    delete,
    path = "/api/v1/images/delete/{id}",
    tag = "Images",
    params(("id" = Uuid, Path, description = "Image ID")),
    responses(
        (status = 200, description = "Image deleted successfully", body = serde_json::Value),
        (status = 404, description = "Image not found", body = serde_json::Value),
        (status = 500, description = "Internal server error", body = serde_json::Value)
    )
)]
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
            warn!("(delete_image) Could not delete image from database");
            return HttpResponse::NotFound().json(json!({
                "status": "Not Found",
                "message": "Image not found"
            }));
        }
        Err(err) => {
            error!("(delete_image) Could not find image: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when finding image, please try again"
            }));
        }
    };

    if let Err(err) = std::fs::remove_file(&image.path) {
        error!("Could not delete file from disk: {:?}", err);
        return HttpResponse::InternalServerError().json(json!({
            "status": "Internal Server Error",
            "message": "Something went wrong when deleting image"
        }));
    }

    match image.delete(db.get_ref()).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "Ok",
            "message": "Image deleted successfully"
        })),
        Err(err) => {
            error!(
                "(delete_image) Could not delete image from database: {:?}",
                err
            );
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when deleting image, please try again"
            }))
        }
    }
}
