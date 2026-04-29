use actix_web::{HttpResponse, Responder, web};
use log::{error, warn};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

use crate::{
    dto::brands::update::{UpdateBrandDTO, UpdateBrandResponse},
    entities::{brands, brands_images, images},
};

#[utoipa::path(
    patch,
    path = "/api/v1/brands/{id}",
    tag = "Brand",
    params(("id" = Uuid, Path, description = "Brand id")),
    request_body = UpdateBrandDTO,
    responses(
        (status = 200, description = "Brand updated successfully", body = UpdateBrandResponse),
        (status = 400, description = "Validation error"),
        (status = 404, description = "Brand not found"),
        (status = 409, description = "Brand name already in use"),
        (status = 500, description = "Internal server error"),
    )
)]

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
            warn!("(update_brand) Brand not found: {}", id);
            return HttpResponse::NotFound().json(json!({
                "status": "Not Found",
                "message": "Brand not found"
            }));
        }
        Err(err) => {
            error!("(update_brand) Database error: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Error when finding brand, please try again later"
            }));
        }
        Ok(Some(_)) => {}
    }

    match existing_brand {
        Ok(Some(_)) => {
            warn!("(update_brand) Brand with same name already exists");
            return HttpResponse::Conflict().json(json!({
                "status": "Conflict",
                "message": "Brand name already in use"
            }));
        }
        Err(err) => {
            error!("(update_brand) Could not check brand name: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Error when finding brand, please try again later"
            }));
        }
        Ok(None) => {}
    }

    let brand = brand.into_inner();
    let updated = brands::ActiveModel {
        id: Set(id),
        name: Set(brand.name),
        updated_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    }
    .update(db.get_ref())
    .await;

    match updated {
        Ok(brand) => {
            let bind = brands_images::Entity::find()
                .filter(brands_images::Column::BrandId.eq(id))
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
                            error!("(update_brand) Could not get image data: {:?}", err);
                            return HttpResponse::InternalServerError().json(json!({
                                "status": "Internal Server Error",
                                "message": "Something went wrong when retrieving brand data"
                            }));
                        }
                    }
                }
                Ok(None) => None,
                Err(err) => {
                    error!("(update_brand) Could not get brand/image bind: {:?}", err);
                    return HttpResponse::InternalServerError().json(json!({
                        "status": "Internal Server Error",
                        "message": "Something went wrong when retrieving brand data"
                    }));
                }
            };

            let dto = UpdateBrandResponse::from((brand, image));
            HttpResponse::Ok().json(dto)
        }
        Err(err) => {
            error!("(update_brand) Could not update brand: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Error when updating brand, please try again later"
            }))
        }
    }
}
