use actix_web::{HttpResponse, Responder, error::ErrorInternalServerError, web};
use log::error;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

use crate::{
    dto::types::create::{CreateTypeDTO, CreateTypeResponse},
    entities::types,
};

#[utoipa::path(
    post,
    path = "/api/v1/types/create",
    tag = "Types",
    request_body = CreateTypeDTO,
    responses(
        (status = 201, description = "Type created successfully", body = CreateTypeResponse),
        (status = 400, description = "Validation error", body = serde_json::Value),
        (status = 409, description = "Type name already exists", body = serde_json::Value),
        (status = 500, description = "Internal server error", body = serde_json::Value)
    )
)]
pub async fn create_type(
    db: web::Data<DatabaseConnection>,
    type_data: web::Json<CreateTypeDTO>,
) -> impl Responder {
    if let Err(errors) = type_data.validate() {
        return Ok(HttpResponse::BadRequest().json(errors));
    }

    let existing_type = types::Entity::find()
        .filter(types::Column::Name.eq(&type_data.name))
        .one(db.get_ref())
        .await;

    match existing_type {
        Ok(Some(_)) => {
            error!("(create_type) Type name already in use");
            return Ok(HttpResponse::Conflict().json(json!({
                "status": "Conflict",
                "message": "Type name already in use"
            })));
        }
        Err(err) => {
            error!("(create_type) Could not find type: {:?}", err);
            return Ok(HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when getting types data, please try again later"
            })));
        }
        Ok(None) => {}
    }

    let type_data = type_data.into_inner();
    let new_type = types::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(type_data.name),
        blocked: Set(false),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    }
    .insert(db.get_ref())
    .await;

    match new_type {
        Ok(new_type) => Ok(HttpResponse::Created().json(CreateTypeResponse::from(new_type))),
        Err(err) => Err(ErrorInternalServerError(err)),
    }
}
