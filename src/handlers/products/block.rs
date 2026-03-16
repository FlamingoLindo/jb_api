use actix_web::{HttpResponse, Responder, web};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Serialize;
use uuid::Uuid;

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
            return HttpResponse::NotFound().json(serde_json::json!({
                "status": "Not Found",
                "message": "Product not found"
            }));
        }
        Err(err) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "Internal Server Error",
                "message": err.to_string()
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
            log::error!("(block_product) Could not block product: {:?}", err);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "Internal Server Error",
                "message": "err.to_string()"
            }))
        }
    }
}
