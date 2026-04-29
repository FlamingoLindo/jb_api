use actix_web::{HttpResponse, Responder, web};
use log::{error, warn};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait};
use serde_json::json;
use uuid::Uuid;

use crate::{
    dto::products::images::bind::BindImageProductDTO,
    entities::{images, products, products_images},
};

#[utoipa::path(
    post,
    path = "/api/v1/products/image-bind",
    tag = "Products",
    request_body = BindImageProductDTO,
    responses(
        (status = 201, description = "Image bound to product successfully", body = serde_json::Value),
        (status = 404, description = "Product or image not found", body = serde_json::Value),
        (status = 409, description = "Image already bound to product", body = serde_json::Value),
        (status = 500, description = "Internal server error", body = serde_json::Value)
    )
)]
pub async fn bind_product_to_image(
    db: web::Data<DatabaseConnection>,
    data: web::Json<BindImageProductDTO>,
) -> impl Responder {
    let data = data.into_inner();

    match products::Entity::find_by_id(data.prod_id)
        .one(db.get_ref())
        .await
    {
        Ok(Some(_)) => {}
        Ok(None) => {
            warn!("(bind_product_to_image) Product not found");
            return HttpResponse::NotFound().json(json!({
                "status": "Not Found",
                "message": "Product not found"
            }));
        }
        Err(err) => {
            error!("(bind_product_to_image) Could not find product: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Something went wrong when trying to find product"
            }));
        }
    }

    match images::Entity::find_by_id(data.img_id)
        .one(db.get_ref())
        .await
    {
        Ok(Some(_)) => {}
        Ok(None) => {
            warn!("(bind_product_to_image) Image not found");
            return HttpResponse::NotFound().json(json!({
                "status": "Not Found",
                "message": "Image not found"
            }));
        }
        Err(err) => {
            error!("(bind_product_to_image) Could not find image: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Something went wrong when trying to find image"
            }));
        }
    }

    let bind = products_images::ActiveModel {
        id: Set(Uuid::new_v4()),
        product_id: Set(data.prod_id),
        image_id: Set(data.img_id),
        ..Default::default()
    }
    .insert(db.get_ref())
    .await;

    match bind {
        Ok(_) => HttpResponse::Created().json(json!({
            "status": "Created",
            "message": "Product/Image bind created"
        })),
        Err(err) => {
            error!("(bind_product_to_image) Could not create bind: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Something went wrong when creating bind"
            }))
        }
    }
}
