use crate::entities::users;
use actix_web::{HttpResponse, Responder, web};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use uuid::Uuid;

pub async fn delete_user(db: web::Data<DatabaseConnection>, id: web::Path<Uuid>) -> impl Responder {
    let user = users::Entity::find()
        .filter(users::Column::Id.eq(*id))
        .one(db.get_ref())
        .await;

    match user {
        Ok(Some(user)) => match user.delete(db.get_ref()).await {
            Ok(_) => HttpResponse::Ok().json(serde_json::json!({
                "status": "Ok",
                "message": "User deleted"
            })),
            Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "Internal Server Error",
                "message": err.to_string()
            })),
        },
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "status": "Not Found",
            "message": "User not found"
        })),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "Internal Server Error",
            "message": err.to_string()
        })),
    }
}
