use actix_web::{HttpResponse, Responder, web};
use log::{error, warn};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

use crate::{
    dto::products::update::{UpdateProductDTO, UpdateProductResponse},
    entities::{brands, brands_images, classes, images, products, types},
};

#[utoipa::path(
    patch,
    path = "/api/v1/products/{id}",
    tag = "Products",
    params(("id" = Uuid, Path, description = "Product ID")),
    request_body = UpdateProductDTO,
    responses(
        (status = 200, description = "Product updated successfully", body = UpdateProductResponse),
        (status = 400, description = "Validation error", body = serde_json::Value),
        (status = 404, description = "Product not found", body = serde_json::Value),
        (status = 409, description = "Product code already in use", body = serde_json::Value),
        (status = 500, description = "Internal server error", body = serde_json::Value)
    )
)]
pub async fn update_product(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
    product_data: web::Json<UpdateProductDTO>,
) -> impl Responder {
    let id = id.into_inner();

    if let Err(errors) = product_data.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    let (current_product, existing_product) = tokio::join!(
        products::Entity::find_by_id(id).one(db.get_ref()),
        products::Entity::find()
            .filter(products::Column::Code.eq(&product_data.code))
            .filter(products::Column::Id.ne(id))
            .one(db.get_ref())
    );

    match current_product {
        Ok(None) => {
            warn!("(update_product) Product not found: {}", id);
            return HttpResponse::NotFound().json(json!({
                "status": "Not Found",
                "message": "Product not found"
            }));
        }
        Err(err) => {
            error!("(update_product) Database error: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Error when finding product, please try again later"
            }));
        }
        Ok(Some(_)) => {}
    }

    match existing_product {
        Ok(Some(_)) => {
            warn!("(update_product) Product with same name already exists");
            return HttpResponse::Conflict().json(json!({
                "status": "Conflict",
                "message": "Product name already in use"
            }));
        }
        Err(err) => {
            error!("(update_product) Could not check product name: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Error when finding product, please try again later"
            }));
        }
        Ok(None) => {}
    }

    let product = product_data.into_inner();

    let updated = products::ActiveModel {
        id: Set(id),
        code: Set(product.code),
        description: Set(product.description),
        brand_id: Set(product.brand_id),
        type_id: Set(product.type_id),
        class_id: Set(product.class_id),
        price_kg: Set(product.price_kg),
        price_kg_no_cut: Set(product.price_kg_no_cut),
        price_kg_cut: Set(product.price_kg_cut),
        price_3mt: Set(product.price_3mt),
        price_br: Set(product.price_br),
        price_rod: Set(product.price_rod),
        weight_3mts: Set(product.weight_3mts),

        price_p_mt: Set(product.price_p_mt),
        cut_percentage: Set(product.cut_percentage),
        weight_p_mm: Set(product.weight_p_mm),
        weight: Set(product.weight),
        weight_esp: Set(product.weight_esp),
        weight_p_br: Set(product.weight_p_br),
        br_price: Set(product.br_price),

        updated_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    }
    .update(db.get_ref())
    .await;

    match updated {
        Ok(product) => {
            // Get Type data
            let type_data: Option<types::Model> = if let Some(type_id) = product.type_id {
                match types::Entity::find_by_id(type_id).one(db.get_ref()).await {
                    Ok(t) => t,
                    Err(err) => {
                        error!("(update_product) Could not get type data: {:?}", err);
                        return HttpResponse::InternalServerError().json(json!({
                            "status": "Internal Server Error",
                            "message": "Something went wrong when updating product"
                        }));
                    }
                }
            } else {
                None
            };

            // Get Class data
            let class_data: Option<classes::Model> = if let Some(class_id) = product.class_id {
                match classes::Entity::find_by_id(class_id)
                    .one(db.get_ref())
                    .await
                {
                    Ok(c) => c,
                    Err(err) => {
                        error!("(update_product) Could not get class data: {:?}", err);
                        return HttpResponse::InternalServerError().json(json!({
                            "status": "Internal Server Error",
                            "message": "Something went wrong when updating product"
                        }));
                    }
                }
            } else {
                None
            };

            // Get Brand data and image path
            let (brand_data, brand_image) = if let Some(brand_id) = product.brand_id {
                match brands::Entity::find_by_id(brand_id).one(db.get_ref()).await {
                    Ok(Some(brand)) => {
                        let bind = brands_images::Entity::find()
                            .filter(brands_images::Column::BrandId.eq(brand.id))
                            .one(db.get_ref())
                            .await;

                        let image = match bind {
                            Ok(Some(bind)) => {
                                match images::Entity::find_by_id(bind.image_id)
                                    .one(db.get_ref())
                                    .await
                                {
                                    Ok(img) => img,
                                    Err(err) => {
                                        error!(
                                            "(update_product) Could not get brand image: {:?}",
                                            err
                                        );
                                        return HttpResponse::InternalServerError().json(json!({
                                            "status": "Internal Server Error",
                                            "message": "Something went wrong when updating product"
                                        }));
                                    }
                                }
                            }
                            Ok(None) => None,
                            Err(err) => {
                                error!(
                                    "(update_product) Could not get brand/image bind: {:?}",
                                    err
                                );
                                return HttpResponse::InternalServerError().json(json!({
                                    "status": "Internal Server Error",
                                    "message": "Something went wrong when updating product"
                                }));
                            }
                        };

                        (Some(brand), image)
                    }
                    Ok(None) => (None, None),
                    Err(err) => {
                        error!("(update_product) Could not get brand data: {:?}", err);
                        return HttpResponse::InternalServerError().json(json!({
                            "status": "Internal Server Error",
                            "message": "Something went wrong when updating product"
                        }));
                    }
                }
            } else {
                (None, None)
            };

            HttpResponse::Ok().json(UpdateProductResponse::from((
                product,
                type_data,
                class_data,
                brand_data,
                brand_image,
            )))
        }
        Err(err) => {
            error!("(update_product) Could not update product: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Error when updating product, please try again later"
            }))
        }
    }
}
