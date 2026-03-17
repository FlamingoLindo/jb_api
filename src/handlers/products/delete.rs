use actix_web::{HttpResponse, Responder, web};
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait};
use serde_json::json;
use uuid::Uuid;

use crate::entities::products;

use log::error;

pub async fn delete_product(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let id = id.into_inner();

    let existing_product = products::Entity::find_by_id(id).one(db.get_ref()).await;

    let delete_product = match existing_product {
        Ok(Some(found_product)) => found_product,
        Ok(None) => {
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
    // TODO also delete product image
    match delete_product.delete(db.get_ref()).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "status": "Ok",
            "message": "Product deleted successfully"
        })),

        Err(err) => {
            error!("(delete_product) Could not delete product: {:?}", err);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "Internal Server Error",
                "message": "An error occurred when deleting the product, please try again later"
            }));
        }
    }
}
