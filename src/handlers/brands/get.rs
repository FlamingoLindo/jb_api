use actix_web::{HttpResponse, Responder, web};
use log::warn;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde_json::json;
use uuid::Uuid;

use crate::{
    dto::brands::get::BrandResponse,
    entities::{brands, brands_images, images},
};

pub async fn get_brand(db: web::Data<DatabaseConnection>, id: web::Path<Uuid>) -> impl Responder {
    let found_brand = brands::Entity::find_by_id(*id).one(db.get_ref()).await;

    match found_brand {
        Ok(Some(found_brand)) => {
            let bind = brands_images::Entity::find()
                .filter(brands_images::Column::BrandId.eq(*id))
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
                            warn!("(get_brand) Could not get image data: {:?}", err);
                            return HttpResponse::InternalServerError().json(json!({
                                "status": "Internal Server Error",
                                "message": "Something went wrong when retrieving brand data"
                            }));
                        }
                    }
                }
                Ok(None) => None,
                Err(err) => {
                    warn!("(get_brand) Could not get brand/image bind: {:?}", err);
                    return HttpResponse::InternalServerError().json(json!({
                        "status": "Internal Server Error",
                        "message": "Something went wrong when retrieving brand data"
                    }));
                }
            };

            let dto = BrandResponse::from((found_brand, image));
            HttpResponse::Ok().json(dto)
        }
        Ok(None) => HttpResponse::NotFound().json(json!({
            "status": "Not Found",
            "message": "Brand not found"
        })),
        Err(err) => {
            warn!("(get_brand) Could not get brand data: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Something went wrong when retrieving brand data"
            }))
        }
    }
}
