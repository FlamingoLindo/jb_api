use actix_web::{HttpResponse, Responder, web};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde_json::json;
use uuid::Uuid;

use crate::{
    dto::products::get::ProductResponse,
    entities::{brands, classes, images, products, types},
};
use log::warn;

pub async fn get_product(db: web::Data<DatabaseConnection>, id: web::Path<Uuid>) -> impl Responder {
    let id = id.into_inner();

    let found_product = products::Entity::find_by_id(id).one(db.get_ref()).await;

    match found_product {
        Ok(Some(product)) => {
            // Get Type data
            let type_data: Option<types::Model> = if let Some(type_id) = product.type_id {
                match types::Entity::find_by_id(type_id).one(db.get_ref()).await {
                    Ok(t) => t,
                    Err(err) => {
                        warn!("(get_product) Could not get type data: {:?}", err);
                        return HttpResponse::InternalServerError().json(serde_json::json!({
                            "status": "Internal Server Error",
                            "message": "Something went wrong when retrieving product"
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
                        warn!("(get_product) Could not get class data: {:?}", err);
                        return HttpResponse::InternalServerError().json(serde_json::json!({
                            "status": "Internal Server Error",
                            "message": "Something went wrong when retrieving product"
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
                        let image = if let Some(image_id) = brand.image_id {
                            match images::Entity::find_by_id(image_id).one(db.get_ref()).await {
                                Ok(img) => img,
                                Err(err) => {
                                    warn!("(get_product) Could not get brand image: {:?}", err);
                                    return HttpResponse::InternalServerError().json(serde_json::json!({
                                        "status": "Internal Server Error",
                                        "message": "Something went wrong when retrieving product"
                                    }));
                                }
                            }
                        } else {
                            None
                        };
                        (Some(brand), image)
                    }
                    Ok(None) => (None, None),
                    Err(err) => {
                        warn!("(get_product) Could not get brand data: {:?}", err);
                        return HttpResponse::InternalServerError().json(serde_json::json!({
                            "status": "Internal Server Error",
                            "message": "Something went wrong when retrieving product"
                        }));
                    }
                }
            } else {
                (None, None)
            };

            HttpResponse::Ok().json(ProductResponse::from((
                product,
                type_data,
                class_data,
                brand_data,
                brand_image,
            )))
        }
        Ok(None) => HttpResponse::NotFound().json(json!({
            "status": "Not Found",
            "message": "Product not found"
        })),
        Err(err) => {
            warn!("(get_product) Could not get product data: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Something went wrong when retrieving product data"
            }))
        }
    }
}
