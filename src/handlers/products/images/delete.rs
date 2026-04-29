use actix_web::{HttpResponse, Responder, web};
use log::error;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use serde_json::json;
use uuid::Uuid;

use crate::entities::products_images;

#[utoipa::path(
    delete,
    path = "/api/v1/products/image-bind/{id}",
    tag = "Products",
    params(("id" = Uuid, Path, description = "Product ID")),
    responses(
        (status = 200, description = "Product images unbound successfully", body = serde_json::Value),
        (status = 500, description = "Internal server error", body = serde_json::Value)
    )
)]
pub async fn delete_product_bind(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>, // Product ID
) -> impl Responder {
    let imgs = products_images::Entity::find()
        .filter(products_images::Column::ProductId.eq(*id))
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
                "(delete_product_bind) Could not image with this product ID: {:?}",
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
