use actix_web::{HttpResponse, Responder, error::ErrorInternalServerError, web};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

use crate::{
    dto::classes::create_class::{CreateClassDTO, CreateClassResponse},
    entities::classes,
};

pub async fn create_class(
    db: web::Data<DatabaseConnection>,
    class: web::Json<CreateClassDTO>,
) -> impl Responder {
    let existing_class = classes::Entity::find()
        .filter(classes::Column::Name.eq(&class.name))
        .one(db.get_ref())
        .await;

    match existing_class {
        Ok(Some(_)) => {
            return Ok(HttpResponse::Conflict().json(json!({
                "status": "Conflict",
                "message": "Class name already in use"
            })));
        }
        Err(err) => {
            return Ok(HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": err.to_string()
            })));
        }
        Ok(None) => {}
    }

    if let Err(errors) = class.validate() {
        return Ok(HttpResponse::BadRequest().json(errors));
    }

    let class = classes::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(class.name.clone()),
        blocked: Set(false),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    }
    .insert(db.get_ref())
    .await;

    match class {
        Ok(class) => Ok(HttpResponse::Created().json(CreateClassResponse::from(class))),
        Err(err) => Err(ErrorInternalServerError(err)),
    }
}
