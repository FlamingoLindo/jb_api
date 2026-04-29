use actix_web::{HttpResponse, Responder, web};
use log::{error, warn};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, prelude::Decimal};
use serde_json::json;
use validator::Validate;

use crate::{dto::products::readjust_price::ReadjustPriceDTO, entities::products};

#[utoipa::path(
    patch,
    path = "/api/v1/products/readjust-prices",
    tag = "Products",
    request_body = ReadjustPriceDTO,
    responses(
        (status = 200, description = "Prices adjusted successfully", body = serde_json::Value),
        (status = 400, description = "Validation error", body = serde_json::Value),
        (status = 404, description = "Product not found", body = serde_json::Value),
        (status = 500, description = "Internal server error", body = serde_json::Value)
    )
)]
pub async fn readjust_price(
    db: web::Data<DatabaseConnection>,
    body: web::Json<ReadjustPriceDTO>,
) -> impl Responder {
    if let Err(errors) = body.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    let dto = body.into_inner();
    let multiplier = Decimal::ONE + (dto.percentage / Decimal::from(100));

    for id in dto.ids {
        let product = products::Entity::find_by_id(id).one(db.get_ref()).await;

        let product = match product {
            Ok(Some(p)) => p,
            Ok(None) => {
                warn!("(readjust_price) Product with id: {:?} not found", id);
                return HttpResponse::NotFound().json(json!({
                    "status": "Not Found",
                    "message": "Product not found"
                }));
            }
            Err(err) => {
                error!("Could not get product data: {:?}", err);
                return HttpResponse::InternalServerError().json(json!({
                    "status": "Internal Server Error",
                    "message": "Something went wrong when retrieving product data"
                }));
            }
        };

        let update = products::ActiveModel {
            id: Set(product.id),
            price_kg: Set(product.price_kg.map(|p| p * multiplier)),
            price_kg_no_cut: Set(product.price_kg_no_cut.map(|p| p * multiplier)),
            price_kg_cut: Set(product.price_kg_cut.map(|p| p * multiplier)),
            price_3mt: Set(product.price_3mt.map(|p| p * multiplier)),
            price_br: Set(product.price_br.map(|p| p * multiplier)),
            price_rod: Set(product.price_rod.map(|p| p * multiplier)),
            weight_3mts: Set(product.weight_3mts.map(|p| p * multiplier)),
            price_p_mt: Set(product.price_p_mt.map(|p| p * multiplier)),
            br_price: Set(product.br_price.map(|p| p * multiplier)),
            updated_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        }
        .update(db.get_ref())
        .await;

        if let Err(err) = update {
            error!("Could not update product {}: {:?}", id, err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Something went wrong when updating product prices"
            }));
        }
    }

    HttpResponse::Ok().json(json!({
        "status": "Ok",
        "message": "Prices have been adjusted for the provided products"
    }))
}
