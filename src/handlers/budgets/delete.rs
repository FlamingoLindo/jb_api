use actix_web::{HttpResponse, Responder, web};
use log::{error, warn};
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait};
use serde_json::json;
use uuid::Uuid;

use crate::entities::budgets;

#[utoipa::path(
    delete,
    path = "/api/v1/budgets/{id}",
    tag = "Budgets",
    params(("id" = Uuid, Path, description = "Budget ID")),
    responses(
        (status = 200, description = "Budget deleted successfully", body = serde_json::Value),
        (status = 404, description = "Budget not found", body = serde_json::Value),
        (status = 500, description = "Internal server error", body = serde_json::Value)
    )
)]
pub async fn delete_budget(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let exiting_budget = budgets::Entity::find_by_id(*id).one(db.get_ref()).await;

    let budget = match exiting_budget {
        Ok(Some(budget)) => budget,
        Ok(None) => {
            warn!("(delete_budget) Budget not found");
            return HttpResponse::NotFound().json(json!({
                "status": "Not Found",
                "message": "Budget not found"
            }));
        }
        Err(err) => {
            error!("(delete_budget) Could not find budget: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when finding budget, please try again"
            }));
        }
    };

    if let Err(err) = std::fs::remove_file(&budget.path) {
        error!("Could not delete file from disk: {:?}", err);
        return HttpResponse::InternalServerError().json(json!({
            "status": "Internal Server Error",
            "message": "Something went wrong when deleting budget"
        }));
    }

    match budget.delete(db.get_ref()).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "Ok",
            "message": "Budget deleted successfully"
        })),
        Err(err) => {
            error!(
                "(delete_budget) Could not delete budget from database: {:?}",
                err
            );
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when deleting budget, please try again"
            }))
        }
    }
}
