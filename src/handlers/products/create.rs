use actix_web::{HttpResponse, Responder, web};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde_json::json;
use uuid::Uuid;

use crate::{
    dto::products::create::{CreateProductDTO, CreateProductResponse},
    entities::{brands, brands_images, classes, images, products, types},
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
            warn!("(create_product) Product with same code");
            return HttpResponse::Conflict().json(json!({
                "status": "Conflict",
                "message": "Product with same code already exists"
            }));
        }
        Err(err) => {
            error!(
                "(create_product) Could not check existing product: {:?}",
                err
            );
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when getting products data, please try again later"
            }));
        }
        Ok(None) => {}
    }

    // Validate type_id exists
    if let Some(type_id) = product.type_id {
        match types::Entity::find_by_id(type_id).one(db.get_ref()).await {
            Ok(Some(_)) => {}
            Ok(None) => {
                warn!("(create_product) type_id not found: {:?}", type_id);
                return HttpResponse::BadRequest().json(json!({
                    "status": "Bad Request",
                    "message": "The provided type_id does not exist"
                }));
            }
            Err(err) => {
                error!("(create_product) Could not validate type_id: {:?}", err);
                return HttpResponse::InternalServerError().json(json!({
                    "status": "Internal Server Error",
                    "message": "There has been an error when validating type, please try again later"
                }));
            }
        }
    }

    // Validate class_id exists
    if let Some(class_id) = product.class_id {
        match classes::Entity::find_by_id(class_id)
            .one(db.get_ref())
            .await
        {
            Ok(Some(_)) => {}
            Ok(None) => {
                warn!("(create_product) class_id not found: {:?}", class_id);
                return HttpResponse::BadRequest().json(json!({
                    "status": "Bad Request",
                    "message": "The provided class_id does not exist"
                }));
            }
            Err(err) => {
                error!("(create_product) Could not validate class_id: {:?}", err);
                return HttpResponse::InternalServerError().json(json!({
                    "status": "Internal Server Error",
                    "message": "There has been an error when validating class, please try again later"
                }));
            }
        }
    }

    // Validate brand_id exists
    if let Some(brand_id) = product.brand_id {
        match brands::Entity::find_by_id(brand_id).one(db.get_ref()).await {
            Ok(Some(_)) => {}
            Ok(None) => {
                warn!("(create_product) brand_id not found: {:?}", brand_id);
                return HttpResponse::BadRequest().json(json!({
                    "status": "Bad Request",
                    "message": "The provided brand_id does not exist"
                }));
            }
            Err(err) => {
                error!("(create_product) Could not validate brand_id: {:?}", err);
                return HttpResponse::InternalServerError().json(json!({
                    "status": "Internal Server Error",
                    "message": "There has been an error when validating brand, please try again later"
                }));
            }
        }
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
                        error!("(create_product) Could not get type data: {:?}", err);
                        return HttpResponse::InternalServerError().json(json!({
                            "status": "Internal Server Error",
                            "message": "Something went wrong when creating product"
                        }));
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
                        error!("(create_product) Could not get class data: {:?}", err);
                        return HttpResponse::InternalServerError().json(json!({
                            "status": "Internal Server Error",
                            "message": "Something went wrong when creating product"
                        }));
                    }
                }
            } else {
                None
            };

            // Get Brand data and image path
            let (brand_data, brand_image) = if let Some(brand_id) = new_product.brand_id {
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
                                            "(create_product) Could not get brand image: {:?}",
                                            err
                                        );
                                        return HttpResponse::InternalServerError().json(json!({
                                            "status": "Internal Server Error",
                                            "message": "Something went wrong when creating product"
                                        }));
                                    }
                                }
                            }
                            Ok(None) => None,
                            Err(err) => {
                                error!(
                                    "(create_product) Could not get brand/image bind: {:?}",
                                    err
                                );
                                return HttpResponse::InternalServerError().json(json!({
                                    "status": "Internal Server Error",
                                    "message": "Something went wrong when creating product"
                                }));
                            }
                        };

                        (Some(brand), image)
                    }
                    Ok(None) => (None, None),
                    Err(err) => {
                        error!("(create_product) Could not get brand data: {:?}", err);
                        return HttpResponse::InternalServerError().json(json!({
                            "status": "Internal Server Error",
                            "message": "Something went wrong when creating product"
                        }));
                    }
                }
            } else {
                (None, None)
            };

            HttpResponse::Created().json(CreateProductResponse::from((
                new_product,
                type_data,
                class_data,
                brand_data,
                brand_image,
            )))
        }
        Err(err) => {
            error!("(create_product) Could not insert product: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when creating product, please try again later"
            }))
        }
    }
}
