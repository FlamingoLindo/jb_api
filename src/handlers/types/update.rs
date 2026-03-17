use actix_web::{HttpResponse, Responder, web};
use log::{error, warn};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

use crate::{
    dto::types::update::{UpdateTypeDTO, UpdateTypeResponse},
    entities::types,
};

pub async fn update_type(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
    type_data: web::Json<UpdateTypeDTO>,
) -> impl Responder {
    let id = id.into_inner();

    if let Err(errors) = type_data.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    let (current_type, existing_type) = tokio::join!(
        types::Entity::find_by_id(id).one(db.get_ref()),
        types::Entity::find()
            .filter(types::Column::Name.eq(&type_data.name))
            .filter(types::Column::Id.ne(id))
            .one(db.get_ref())
    );

    match current_type {
        Ok(None) => {
            warn!("(update_type) Type not found: {}", id);
            return HttpResponse::NotFound().json(json!({
                "status": "Not Found",
                "message": "Type not found"
            }));
        }
        Err(err) => {
            error!("(update_type) Database error: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Error when finding type, please try again later"
            }));
        }
        Ok(Some(_)) => {}
    }

    match existing_type {
        Ok(Some(_)) => {
            warn!("(update_type) Type with same name already exists");
            return HttpResponse::Conflict().json(json!({
                "status": "Conflict",
                "message": "Type name already in use"
            }));
        }
        Err(err) => {
            error!("(update_type) Could not check type name: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Error when finding type, please try again later"
            }));
        }
        Ok(None) => {}
    }

    let updated = types::ActiveModel {
        id: Set(id),
        name: Set(type_data.name.clone()),
        updated_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    }
    .update(db.get_ref())
    .await;

    match updated {
        Ok(updated_type) => HttpResponse::Ok().json(UpdateTypeResponse::from(updated_type)),
        Err(err) => {
            error!("(update_type) Could not update type: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Error when updating type, please try again later"
            }))
        }
    }
}
