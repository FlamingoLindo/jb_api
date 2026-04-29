use actix_web::{HttpResponse, Responder, web};
use log::{error, warn};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect,
};
use serde_json::json;
use uuid::Uuid;

use crate::entities::{budgets, clients};

#[utoipa::path(
    get,
    path = "/api/v1/budgets/count/{id}",
    tag = "Budgets",
    params(("id" = Uuid, Path, description = "Client ID")),
    responses(
        (status = 200, description = "Budget count retrieved", body = serde_json::Value),
        (status = 404, description = "Client not found", body = serde_json::Value),
        (status = 500, description = "Internal server error", body = serde_json::Value)
    )
)]
pub async fn count_client_budgets(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let existing_client = clients::Entity::find_by_id(*id).one(db.get_ref()).await;

    let client = match existing_client {
        Ok(Some(client)) => client,
        Ok(None) => {
            warn!("(count_client_budgets) Client not found");
            return HttpResponse::NotFound().json(json!({
                "status": "Not Found",
                "message": "Client not found"
            }));
        }
        Err(err) => {
            error!("(count_client_budgets) Could not find client: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when finding client, please try again"
            }));
        }
    };

    let count = budgets::Entity::find()
        .select_only()
        .filter(budgets::Column::ClientId.eq(client.id))
        .count(db.get_ref())
        .await;

    match count {
        Ok(c) => HttpResponse::Ok().json(c),
        Err(err) => {
            error!("(count_client_budgets) Could not count budgets: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Something went wrong when counting budgets"
            }))
        }
    }
}
