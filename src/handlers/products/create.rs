use actix_web::{HttpResponse, Responder, error::ErrorInternalServerError, web};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde_json::json;
use uuid::Uuid;

use crate::{
    dto::products::create::{CreateProductDTO, CreateProductResponse},
    entities::{brands, classes, images, products, types},
};

use log::{error, warn};

pub async fn create_product(
    db: web::Data<DatabaseConnection>,
    product: web::Json<CreateProductDTO>,
) -> impl Responder {
    let existing_product = products::Entity::find()
        .filter(products::Column::Code.eq(&product.code))
        .one(db.get_ref())
        .await;

    match existing_product {
        Ok(Some(_)) => {
            return Ok(HttpResponse::Conflict().json(json!({
                "status": "Conflict",
                "message": "Product with same code already exists"
            })));
        }
        Err(err) => {
            error!("(create_product) Could not find type: {:?}", err);
            return Ok(HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when getting products data, please try again later"
            })));
        }
        Ok(None) => {}
    }

    let product = product.into_inner();

    let new_product = products::ActiveModel {
        id: Set(Uuid::new_v4()),
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

        blocked: Set(false),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    }
    .insert(db.get_ref())
    .await;

    match new_product {
        Ok(new_product) => {
            // Get Type data
            let type_data: Option<types::Model> = if let Some(type_id) = new_product.type_id {
                match types::Entity::find_by_id(type_id).one(db.get_ref()).await {
                    Ok(t) => t,
                    Err(err) => {
                        warn!("(create_product) Could not get type data: {:?}", err);
                        return Ok(HttpResponse::InternalServerError().json(json!({
                            "status": "Internal Server Error",
                            "message": "Something went wrong when creating product"
                        })));
                    }
                }
            } else {
                None
            };

            // Get Class data
            let class_data: Option<classes::Model> = if let Some(class_id) = new_product.class_id {
                match classes::Entity::find_by_id(class_id)
                    .one(db.get_ref())
                    .await
                {
                    Ok(c) => c,
                    Err(err) => {
                        warn!("(create_product) Could not get class data: {:?}", err);
                        return Ok(HttpResponse::InternalServerError().json(json!({
                            "status": "Internal Server Error",
                            "message": "Something went wrong when creating product"
                        })));
                    }
                }
            } else {
                None
            };

            // Get Brand data and image path
            let (brand_data, brand_image) = if let Some(brand_id) = new_product.brand_id {
                match brands::Entity::find_by_id(brand_id).one(db.get_ref()).await {
                    Ok(Some(brand)) => {
                        let image = if let Some(image_id) = brand.image_id {
                            match images::Entity::find_by_id(image_id).one(db.get_ref()).await {
                                Ok(img) => img,
                                Err(err) => {
                                    warn!("(create_product) Could not get brand image: {:?}", err);
                                    return Ok(HttpResponse::InternalServerError().json(json!({
                                        "status": "Internal Server Error",
                                        "message": "Something went wrong when creating product"
                                    })));
                                }
                            }
                        } else {
                            None
                        };
                        (Some(brand), image)
                    }
                    Ok(None) => (None, None),
                    Err(err) => {
                        warn!("(create_product) Could not get brand data: {:?}", err);
                        return Ok(HttpResponse::InternalServerError().json(json!({
                            "status": "Internal Server Error",
                            "message": "Something went wrong when creating product"
                        })));
                    }
                }
            } else {
                (None, None)
            };

            Ok(HttpResponse::Created().json(CreateProductResponse::from((
                new_product,
                type_data,
                class_data,
                brand_data,
                brand_image,
            ))))
        }
        Err(err) => Err(ErrorInternalServerError(err)),
    }
}
