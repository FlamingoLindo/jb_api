use crate::entities::brands;
use actix_web::{HttpResponse, Responder, web};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct BlockBrandResponse {
    pub name: String,
    pub blocked: bool,
}

pub async fn block_brand(db: web::Data<DatabaseConnection>, id: web::Path<Uuid>) -> impl Responder {
    let existing_brand = brands::Entity::find()
        .filter(brands::Column::Id.eq(*id))
        .one(db.get_ref())
        .await;

    let brand = match existing_brand {
        Ok(Some(brand)) => brand,
        Ok(None) => {
            return HttpResponse::NotFound().json(serde_json::json!({
                "status": "Not Found",
                "message": "Brand not found"
            }));
        }
        Err(err) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "Internal Server Error",
                "message": err.to_string()
            }));
        }
    };

    let mut active_brand: brands::ActiveModel = brand.into();
    active_brand.blocked = Set(!active_brand.blocked.unwrap());

    match active_brand.update(db.get_ref()).await {
        Ok(updated_brand) => HttpResponse::Ok().json(BlockBrandResponse {
            name: updated_brand.name,
            blocked: updated_brand.blocked,
        }),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "Internal Server Error",
            "message": err.to_string()
        })),
    }
}
