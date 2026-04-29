use actix_web::{HttpResponse, Responder, web};
use log::{error, warn};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use serde_json::json;
use uuid::Uuid;

use crate::entities::{images, products, products_images};

#[utoipa::path(
    delete,
    path = "/api/v1/products/{id}",
    tag = "Products",
    params(("id" = Uuid, Path, description = "Product ID")),
    responses(
        (status = 200, description = "Product deleted successfully", body = serde_json::Value),
        (status = 404, description = "Product not found", body = serde_json::Value),
        (status = 500, description = "Internal server error", body = serde_json::Value)
    )
)]
pub async fn delete_product(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let id = id.into_inner();

    let existing_product = products::Entity::find_by_id(id).one(db.get_ref()).await;

    let delete_product = match existing_product {
        Ok(Some(found_product)) => found_product,
        Ok(None) => {
            warn!("(delete_product) Product not found");
            return HttpResponse::NotFound().json(json!({
                "status": "Not Found",
                "message": "Product not found"
            }));
        }
        Err(err) => {
            error!("(delete_product) Could not find Product: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when finding the provided product, please try again later"
            }));
        }
    };
    let bind_imgs = products_images::Entity::find()
        .filter(products_images::Column::ProductId.eq(id))
        .all(db.get_ref())
        .await;

    match bind_imgs {
        Ok(binds) => {
            for bind in binds {
                let image = images::Entity::find()
                    .filter(images::Column::Id.eq(bind.image_id))
                    .one(db.get_ref())
                    .await;

                match image {
                    Ok(Some(img)) => {
                        if let Err(err) = std::fs::remove_file(&img.path) {
                            error!("Could not delete file from disk: {:?}", err);
                            return HttpResponse::InternalServerError().json(json!({
                                "status": "Internal Server Error",
                                "message": "Something went wrong when deleting image"
                            }));
                        }

                        let _ = img.delete(db.get_ref()).await;
                    }
                    Ok(None) => {
                        warn!("(delete_product) Image not found");
                        return HttpResponse::NotFound().json(json!({
                            "status": "Not Found",
                            "message": "Image not found"
                        }));
                    }
                    Err(err) => {
                        error!("(delete_product) Could not find Product: {:?}", err);
                        return HttpResponse::InternalServerError().json(json!({
                        "status": "Internal Server Error",
                        "message": "There has been an error when deleting product image, please try again later"
                    }));
                    }
                }

                let _ = bind.delete(db.get_ref()).await;
            }
        }
        Err(err) => {
            error!("(delete_product) Could not product's images: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "An error occurred when deleting the product's images, please try again later"
            }));
        }
    }

    match delete_product.delete(db.get_ref()).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "Ok",
            "message": "Product deleted successfully"
        })),

        Err(err) => {
            error!("(delete_product) Could not delete product: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "An error occurred when deleting the product, please try again later"
            }));
        }
    }
}
