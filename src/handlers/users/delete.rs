use crate::entities::users;
use actix_web::{HttpResponse, Responder, web};
use log::error;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use serde_json::json;
use uuid::Uuid;

pub async fn delete_user(db: web::Data<DatabaseConnection>, id: web::Path<Uuid>) -> impl Responder {
    let user = users::Entity::find()
        .filter(users::Column::Id.eq(*id))
        .one(db.get_ref())
        .await;

    match user {
        Ok(Some(user)) => match user.delete(db.get_ref()).await {
            Ok(_) => HttpResponse::Ok().json(json!({
                "status": "Ok",
                "message": "User deleted"
            })),
            Err(err) => {
                error!("(delete_user) Could not delete user: {:?}", err);
                HttpResponse::InternalServerError().json(json!({
                    "status": "Internal Server Error",
                    "message": "There has been an error when deleting user, please try again"
                }))
            }
        },
        Ok(None) => HttpResponse::NotFound().json(json!({
            "status": "Not Found",
            "message": "User not found"
        })),
        Err(err) => {
            error!("(delete_user) Could not find user: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when finding user, please try again"
            }))
        }
    }
}
