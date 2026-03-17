use actix_web::{HttpResponse, Responder, web};
use log::warn;
use sea_orm::{DatabaseConnection, EntityTrait};
use uuid::Uuid;

use crate::{
    dto::brands::get::BrandResponse,
    entities::{brands, images},
};

pub async fn get_brand(db: web::Data<DatabaseConnection>, id: web::Path<Uuid>) -> impl Responder {
    let id = id.into_inner();

    let found_brand = brands::Entity::find_by_id(id).one(db.get_ref()).await;

    match found_brand {
        Ok(Some(found_brand)) => {
            let image = if let Some(image_id) = found_brand.image_id {
                match images::Entity::find_by_id(image_id).one(db.get_ref()).await {
                    Ok(img) => img,
                    Err(err) => {
                        warn!("(get_brand) Could not get image data: {:?}", err);
                        return HttpResponse::InternalServerError().json(serde_json::json!({
                            "status": "Internal Server Error",
                            "message": "Something went wrong when retrieving brand data"
                        }));
                    }
                }
            } else {
                None
            };

            let dto = BrandResponse::from((found_brand, image));
            HttpResponse::Ok().json(dto)
        }
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "status": "Not Found",
            "message": "Brand not found"
        })),
        Err(err) => {
            warn!("(get_brand) Could not get brand data: {:?}", err);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "Internal Server Error",
                "message": "Something went wrong when retrieving brand data"
            }))
        }
    }
}
