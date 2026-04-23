use actix_web::{HttpResponse, Responder, web};
use log::{error, warn};
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait};
use serde_json::json;
use uuid::Uuid;

use crate::entities::types;

pub async fn delete_type(db: web::Data<DatabaseConnection>, id: web::Path<Uuid>) -> impl Responder {
    let id = id.into_inner();

    let existing_type = types::Entity::find_by_id(id).one(db.get_ref()).await;

    let delete_type = match existing_type {
        Ok(Some(found_type)) => found_type,
        Ok(None) => {
            warn!("(delete_type) Type not found");
            return HttpResponse::NotFound().json(json!({
                "status": "Not Found",
                "message": "Type not found"
            }));
        }
        Err(err) => {
            error!("(delete_type) Could not find Type: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when finding the provided type, please try again later"
            }));
        }
    };

    match delete_type.delete(db.get_ref()).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "Ok",
            "message": "Type deleted successfully"
        })),

        Err(err) => {
            error!("(delete_type) Could not delete type: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "An error occurred when deleting the type, please try again later"
            }));
        }
    }
}
