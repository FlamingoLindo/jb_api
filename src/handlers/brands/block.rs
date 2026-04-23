use crate::entities::brands;
use actix_web::{HttpResponse, Responder, web};
use log::{error, warn};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Serialize;
use serde_json::json;
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
            warn!("(block_brand) Brand not found");
            return HttpResponse::NotFound().json(json!({
                "status": "Not Found",
                "message": "Brand not found"
            }));
        }
        Err(err) => {
            error!("(block_brand) Could not find brand: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when finding brand, please try again"
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
        Err(err) => {
            error!(
                "(block_brand) Could not update brand block status: {:?}",
                err
            );
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when updating brand, please try again"
            }))
        }
    }
}
