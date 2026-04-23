use actix_web::{HttpResponse, Responder, web};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Serialize;
use serde_json::json;
use uuid::Uuid;

use log::{error, warn};

use crate::entities::products;

#[derive(Serialize)]
pub struct BlockProductResponse {
    pub code: String,
    pub blocked: bool,
}

pub async fn block_product(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let existing_product = products::Entity::find()
        .filter(products::Column::Id.eq(*id))
        .one(db.get_ref())
        .await;

    let blocked_product = match existing_product {
        Ok(Some(blocked_product)) => blocked_product,
        Ok(None) => {
            warn!("(block_product) Product not found");
            return HttpResponse::NotFound().json(json!({
                "status": "Not Found",
                "message": "Product not found"
            }));
        }
        Err(err) => {
            error!("(block_product) Could not find product: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when finding product, please try again"
            }));
        }
    };

    let mut active_product: products::ActiveModel = blocked_product.into();
    active_product.blocked = Set(!active_product.blocked.unwrap());

    match active_product.update(db.get_ref()).await {
        Ok(updated_product) => HttpResponse::Ok().json(BlockProductResponse {
            code: updated_product.code,
            blocked: updated_product.blocked,
        }),
        Err(err) => {
            error!("(block_product) Could not block product: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when updating product, please try again"
            }))
        }
    }
}
