use actix_web::{HttpResponse, Responder, web};
use log::{error, warn};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde_json::json;
use uuid::Uuid;

use crate::{dto::clients::get::ClientResponse, entities::clients};

pub async fn get_client(db: web::Data<DatabaseConnection>, id: web::Path<Uuid>) -> impl Responder {
    let client = clients::Entity::find_by_id(id.into_inner())
        .one(db.get_ref())
        .await;

    match client {
        Ok(Some(client)) => {
            let dto = ClientResponse::from(client);
            HttpResponse::Ok().json(dto)
        }
        Ok(None) => {
            warn!("(get_client) Client not found");
            HttpResponse::NotFound().json(json!({
                "status": "Not found",
                "message": "Client not found"
            }))
        }
        Err(err) => {
            error!("Could not get client data: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Something went wrong when retrieving client data"
            }))
        }
    }
}
