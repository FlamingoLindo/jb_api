use actix_web::{HttpResponse, Responder, error::ErrorInternalServerError, web};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;
use validator::Validate;

use crate::{
    dto::brands::create_brand::{CreateBrandDTO, CreateBrandResponse},
    entities::brands,
};

pub async fn create_brand(
    db: web::Data<DatabaseConnection>,
    brand: web::Json<CreateBrandDTO>,
) -> impl Responder {
    let existing_brand = brands::Entity::find()
        .filter(brands::Column::Name.eq(&brand.name))
        .one(db.get_ref())
        .await;

    match existing_brand {
        Ok(Some(_)) => {
            return Ok(HttpResponse::Conflict().json(serde_json::json!({
                "status": "Conflict",
                "message": "Username already taken"
            })));
        }
        Err(err) => {
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "Internal Server Error",
                "message": err.to_string()
            })));
        }
        Ok(None) => {}
    }

    if let Err(errors) = brand.validate() {
        return Ok(HttpResponse::BadRequest().json(errors));
    }

    // Create brand
    let new_brand = brands::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(brand.name.clone()),
        image_id: Set(brand.image_id),
        created_at: Set(chrono::Utc::now().naive_utc()),
        blocked: Set(false),
        ..Default::default()
    }
    .insert(db.get_ref())
    .await;

    match new_brand {
        Ok(brand) => Ok(HttpResponse::Created().json(CreateBrandResponse::from(brand))),
        Err(err) => Err(ErrorInternalServerError(err)),
    }
}
