use actix_web::{HttpResponse, Responder, web};
use log::{error, warn};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Serialize;
use serde_json::json;
use uuid::Uuid;
use utoipa::ToSchema;

use crate::entities::clients;

#[derive(Serialize, ToSchema)]
pub struct BlockClientResponse {
    pub name: String,
    pub blocked: bool,
}

#[utoipa::path(
    patch,
    path = "/api/v1/clients/block/{id}",
    tag = "Clients",
    params(("id" = Uuid, Path, description = "Client ID")),
    responses(
        (status = 200, description = "Client block status toggled", body = BlockClientResponse),
        (status = 404, description = "Client not found", body = serde_json::Value),
        (status = 500, description = "Internal server error", body = serde_json::Value)
    )
)]
pub async fn block_client(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let found_client = clients::Entity::find()
        .filter(clients::Column::Id.eq(*id))
        .one(db.get_ref())
        .await;

    let client = match found_client {
        Ok(Some(client)) => client,
        Ok(None) => {
            warn!("(block_client) Client not found");
            return HttpResponse::NotFound().json(json!({
                "status": "Not Found",
                "message": "Client not found"
            }));
        }
        Err(err) => {
            error!("(block_client) Could not find client: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when finding client, please try again"
            }));
        }
    };

    let mut active_client: clients::ActiveModel = client.into();
    active_client.blocked = Set(!active_client.blocked.unwrap());

    match active_client.update(db.get_ref()).await {
        Ok(client) => HttpResponse::Ok().json(BlockClientResponse {
            name: client.name,
            blocked: client.blocked,
        }),
        Err(err) => {
            error!(
                "(block_client) Could not update client block status: {:?}",
                err
            );
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when updating client, please try again"
            }))
        }
    }
}
