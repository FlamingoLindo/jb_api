use actix_web::{HttpResponse, Responder, web};
use log::{error, warn};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

use crate::{
    dto::classes::update::{UpdateClassDTO, UpdateClassResponse},
    entities::classes,
};

pub async fn update_class(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
    class: web::Json<UpdateClassDTO>,
) -> impl Responder {
    let id = id.into_inner();

    if let Err(errors) = class.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    let (current_class, existing_class) = tokio::join!(
        classes::Entity::find_by_id(id).one(db.get_ref()),
        classes::Entity::find()
            .filter(classes::Column::Name.eq(&class.name))
            .filter(classes::Column::Id.ne(id))
            .one(db.get_ref())
    );

    match current_class {
        Ok(None) => {
            warn!("(update_class) Class not found: {}", id);
            return HttpResponse::NotFound().json(json!({
                "status": "Not Found",
                "message": "Class not found"
            }));
        }
        Err(err) => {
            error!("(update_class) Database error: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Error when finding class, please try again later"
            }));
        }
        Ok(Some(_)) => {}
    }

    match existing_class {
        Ok(Some(_)) => {
            warn!("(update_class) Class with same name already exists");
            return HttpResponse::Conflict().json(json!({
                "status": "Conflict",
                "message": "Class name already in use"
            }));
        }
        Err(err) => {
            error!("(update_class) Could not check class name: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Error when finding class, please try again later"
            }));
        }
        Ok(None) => {}
    }

    let updated = classes::ActiveModel {
        id: Set(id),
        name: Set(class.name.clone()),
        updated_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    }
    .update(db.get_ref())
    .await;

    match updated {
        Ok(class) => HttpResponse::Ok().json(UpdateClassResponse::from(class)),
        Err(err) => {
            error!("(update_class) Could not update class: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Error when updating class, please try again later"
            }))
        }
    }
}
