use actix_web::{HttpResponse, Responder, web};
use log::error;
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait};
use serde_json::json;
use uuid::Uuid;

use crate::entities::classes;

pub async fn delete_class(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let id = id.into_inner();

    let existing_class = classes::Entity::find_by_id(id).one(db.get_ref()).await;

    let class = match existing_class {
        Ok(Some(class)) => class,
        Ok(None) => {
            return HttpResponse::NotFound().json(json!({
                "status": "Not Found",
                "message": "Class not found"
            }));
        }
        Err(err) => {
            error!("(delete_class) Could not find class: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when finding the provided class, please try again later"
            }));
        }
    };

    match class.delete(db.get_ref()).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "status": "Ok",
            "message": "Class deleted successfully"
        })),

        Err(err) => {
            error!("(delete_class) Could not delete class: {:?}", err);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "Internal Server Error",
                "message": "An error occurred when deleting the class, please try again later"
            }));
        }
    }
}
