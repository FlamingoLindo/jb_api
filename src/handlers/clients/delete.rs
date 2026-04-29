use actix_web::{HttpResponse, Responder, web};
use log::{error, warn};
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait};
use serde_json::json;
use uuid::Uuid;

use crate::entities::clients;

#[utoipa::path(
    delete,
    path = "/api/v1/clients/{id}",
    tag = "Clients",
    params(("id" = Uuid, Path, description = "Client ID")),
    responses(
        (status = 200, description = "Client deleted successfully", body = serde_json::Value),
        (status = 404, description = "Client not found", body = serde_json::Value),
        (status = 500, description = "Internal server error", body = serde_json::Value)
    )
)]
pub async fn delete_client(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let id = id.into_inner();

    let existing_client = clients::Entity::find_by_id(id).one(db.get_ref()).await;

    let client = match existing_client {
        Ok(Some(found_client)) => found_client,
        Ok(None) => {
            warn!("(delete_client) Client not found");
            return HttpResponse::NotFound().json(json!({
                "status": "Not Found",
                "message": "Client not found"
            }));
        }
        Err(err) => {
            error!("(delete_client) Could not find client: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when finding the provided client, please try again later"
            }));
        }
    };

    match client.delete(db.get_ref()).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "Ok",
            "message": "Client deleted successfully"
        })),

        Err(err) => {
            error!("(delete_client) Could not delete client: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "An error occurred when deleting the client, please try again later"
            }));
        }
    }
}
