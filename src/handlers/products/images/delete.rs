use actix_web::{HttpResponse, Responder, web};
use log::error;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use serde_json::json;
use uuid::Uuid;

use crate::entities::products_images;

pub async fn delete_bind(
    db: web::Data<DatabaseConnection>,
    id: web::Path<String>, // Product ID
) -> impl Responder {
    let id = id.into_inner();

    let uuid = match Uuid::parse_str(&id) {
        Ok(uuid) => uuid,
        Err(_) => {
            return HttpResponse::BadRequest().json(json!({
                "status": "Bad Request",
                "message": format!("Invalid UUID format: {}", id)
            }));
        }
    };

    let imgs = products_images::Entity::find()
        .filter(products_images::Column::ProductId.eq(uuid))
        .all(db.get_ref())
        .await;

    match imgs {
        Ok(imgs) => {
            for img in imgs {
                let _ = img.delete(db.get_ref()).await;
            }
        }
        Err(err) => {
            error!(
                "(delete_bind) Could not image with this product ID: {:?}",
                err
            );
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when finding product image, please try again"
            }));
        }
    }

    HttpResponse::Ok().json(json!({
        "status": "Ok",
        "message": "Deleted"
    }))
}
