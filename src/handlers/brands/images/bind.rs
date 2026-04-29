use actix_web::{HttpResponse, Responder, web};
use log::{error, warn};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait};
use serde_json::json;
use uuid::Uuid;

use crate::{
    dto::brands::images::bind::BindImageBrandDTO,
    entities::{brands, brands_images, images},
};

#[utoipa::path(
    post,
    path = "/api/v1/brands/image-bind",
    tag = "Brand",
    request_body = BindImageBrandDTO,
    responses(
        (status = 201, description = "Brand/image bind created", body = serde_json::Value),
        (status = 404, description = "Brand or image not found"),
        (status = 500, description = "Internal server error"),
    )
)]

pub async fn bind_brand_to_image(
    db: web::Data<DatabaseConnection>,
    data: web::Json<BindImageBrandDTO>,
) -> impl Responder {
    let data = data.into_inner();

    let found_brand = brands::Entity::find_by_id(data.brand_id)
        .one(db.get_ref())
        .await;

    let brand = match found_brand {
        Ok(Some(brand)) => brand,
        Ok(None) => {
            warn!("(bind_brand_to_image) Brand not found");
            return HttpResponse::NotFound().json(json!({
                "status": "Not Found",
                "message": "Brand not found"
            }));
        }
        Err(err) => {
            error!("(bind_brand_to_image) Could not find brand: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Something went wrong when trying to find brand"
            }));
        }
    };

    let found_image = images::Entity::find_by_id(data.img_id)
        .one(db.get_ref())
        .await;

    let image = match found_image {
        Ok(Some(image)) => image,
        Ok(None) => {
            warn!("(bind_brand_to_image) Image not found");
            return HttpResponse::NotFound().json(json!({
                "status": "Not Found",
                "message": "Image not found"
            }));
        }
        Err(err) => {
            error!("(bind_brand_to_image) Could not find image: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Something went wrong when trying to find image"
            }));
        }
    };

    let bind = brands_images::ActiveModel {
        id: Set(Uuid::new_v4()),
        brand_id: Set(brand.id),
        image_id: Set(image.id),
        ..Default::default()
    }
    .insert(db.get_ref())
    .await;

    match bind {
        Ok(_) => HttpResponse::Created().json(json!({
            "status": "Created",
            "message": "Brand/Image bind created"
        })),
        Err(err) => {
            error!("(bind_brand_to_image) Could not create bind: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Something went wrong when creating bind"
            }))
        }
    }
}
