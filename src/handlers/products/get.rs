use actix_web::{HttpResponse, Responder, web};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde_json::json;
use uuid::Uuid;

use crate::{
    dto::products::get::ProductResponse,
    entities::{brands, brands_images, classes, images, products, products_images, types},
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
                        return HttpResponse::InternalServerError().json(json!({
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
                        return HttpResponse::InternalServerError().json(json!({
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
                                        warn!("(get_product) Could not get brand image: {:?}", err);
                                        return HttpResponse::InternalServerError().json(json!({
                                "status": "Internal Server Error",
                                "message": "Something went wrong when retrieving product"
                            }));
                                    }
                                }
                            }
                            Ok(None) => None,
                            Err(err) => {
                                warn!("(get_product) Could not get brand/image bind: {:?}", err);
                                return HttpResponse::InternalServerError().json(json!({
                                    "status": "Internal Server Error",
                                    "message": "Something went wrong when retrieving product"
                                }));
                            }
                        };

                        (Some(brand), image)
                    }
                    Ok(None) => (None, None),
                    Err(err) => {
                        warn!("(get_product) Could not get brand data: {:?}", err);
                        return HttpResponse::InternalServerError().json(json!({
                            "status": "Internal Server Error",
                            "message": "Something went wrong when retrieving product"
                        }));
                    }
                }
            } else {
                (None, None)
            };

            // Get Product image
            let product_images: Vec<images::Model> = {
                match products_images::Entity::find()
                    .filter(products_images::Column::ProductId.eq(id))
                    .all(db.get_ref())
                    .await
                {
                    Ok(rows) => {
                        let mut imgs = vec![];
                        for pi in rows {
                            match images::Entity::find_by_id(pi.image_id)
                                .one(db.get_ref())
                                .await
                            {
                                Ok(Some(img)) => imgs.push(img),
                                Ok(None) => {}
                                Err(err) => {
                                    warn!("(get_product) Could not get product image: {:?}", err);
                                    return HttpResponse::InternalServerError().json(json!({
                                        "status": "Internal Server Error",
                                        "message": "Something went wrong when retrieving product"
                                    }));
                                }
                            }
                        }
                        imgs
                    }
                    Err(err) => {
                        warn!("(get_product) Could not get product images: {:?}", err);
                        return HttpResponse::InternalServerError().json(json!({
                            "status": "Internal Server Error",
                            "message": "Something went wrong when retrieving product"
                        }));
                    }
                }
            };

            HttpResponse::Ok().json(ProductResponse::from((
                product,
                type_data,
                class_data,
                brand_data,
                brand_image,
                product_images,
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
