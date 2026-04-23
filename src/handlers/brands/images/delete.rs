use actix_web::{HttpResponse, Responder, web};
use log::error;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use serde_json::json;
use uuid::Uuid;

use crate::entities::brands_images;

pub async fn delete_brand_bind(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>, // Brands Id
) -> impl Responder {
    let images = brands_images::Entity::find()
        .filter(brands_images::Column::BrandId.eq(*id))
        .all(db.get_ref())
        .await;

    match images {
        Ok(images) => {
            for img in images {
                let _ = img.delete(db.get_ref()).await;
            }
        }
        Err(err) => {
            error!(
                "(delete_brand_bind) Could not image with this brand Id: {:?}",
                err
            );
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when finding brand image, please try again"
            }));
        }
    }

    HttpResponse::Ok().json(json!({
        "status": "Ok",
        "message": "Deleted"
    }))
}
