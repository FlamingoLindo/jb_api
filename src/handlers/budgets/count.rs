use actix_web::{HttpResponse, Responder, web};
use log::{error, warn};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect,
};
use serde_json::json;
use uuid::Uuid;

use crate::entities::{budgets, clients};

pub async fn count_client_budgets(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let existing_client = clients::Entity::find_by_id(*id).one(db.get_ref()).await;

    let client = match existing_client {
        Ok(Some(client)) => client,
        Ok(None) => {
            return HttpResponse::NotFound().json(json!({
                "status": "Not Found",
                "message": "Client not found"
            }));
        }
        Err(err) => {
            error!("(delete_image) Could not find image: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when finding image, please try again"
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
            warn!("(count_client_budgets) Could not count budgets: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Something went wrong when counting budgets"
            }))
        }
    }
}
