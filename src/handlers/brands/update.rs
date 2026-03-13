use actix_web::{HttpResponse, Responder, web};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

use crate::{
    dto::brands::update::{UpdateBrandDTO, UpdateBrandResponse},
    entities::{brands, images},
};

pub async fn update_brand(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
    brand: web::Json<UpdateBrandDTO>,
) -> impl Responder {
    let id = id.into_inner();

    if let Err(errors) = brand.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    let (current_brand, existing_brand) = tokio::join!(
        brands::Entity::find_by_id(id).one(db.get_ref()),
        brands::Entity::find()
            .filter(brands::Column::Name.eq(&brand.name))
            .filter(brands::Column::Id.ne(id))
            .one(db.get_ref())
    );

    match current_brand {
        Ok(None) => {
            log::warn!("(update_brand) Brand not found: {}", id);
            return HttpResponse::NotFound().json(json!({
                "status": "Not Found",
                "message": "Brand not found"
            }));
        }
        Err(err) => {
            log::error!("(update_brand) Database error: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Error when finding brand, please try again later"
            }));
        }
        Ok(Some(_)) => {}
    }

    match existing_brand {
        Ok(Some(_)) => {
            log::warn!("(update_brand) Brand with same name already exists");
            return HttpResponse::Conflict().json(json!({
                "status": "Conflict",
                "message": "Brand name already in use"
            }));
        }
        Err(err) => {
            log::error!("(update_brand) Could not check brand name: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Error when finding brand, please try again later"
            }));
        }
        Ok(None) => {}
    }

    let updated = brands::ActiveModel {
        id: Set(id),
        name: Set(brand.name.clone()),
        image_id: Set(brand.image_id),
        updated_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    }
    .update(db.get_ref())
    .await;

    match updated {
        Ok(brand) => {
            let image = if let Some(image_id) = brand.image_id {
                match images::Entity::find_by_id(image_id).one(db.get_ref()).await {
                    Ok(img) => img,
                    Err(err) => {
                        log::warn!("(update_brand) Could not get image data: {:?}", err);
                        return HttpResponse::InternalServerError().json(serde_json::json!({
                            "status": "Internal Server Error",
                            "message": "Something went wrong when retrieving brand data"
                        }));
                    }
                }
            } else {
                None
            };

            let dto = UpdateBrandResponse::from((brand, image));
            HttpResponse::Ok().json(dto)
        }
        Err(err) => {
            log::error!("(update_brand) Could not update brand: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Error when updating brand, please try again later"
            }))
        }
    }
}
