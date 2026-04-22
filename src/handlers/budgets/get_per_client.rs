use actix_web::{HttpResponse, Responder, web};
use log::{error, warn};
use sea_orm::sea_query::Expr;
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, Order, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect,
};
use serde_json::json;
use uuid::Uuid;

use crate::entities::clients;
use crate::{
    dto::budget::get_per_client::{
        GetAllBudgetsPerClientDTO, GetAllBudgetsPerClientQueryParams,
        GetAllBudgetsPerClientSortOrder,
    },
    entities::budgets,
};

pub async fn get_all_budgets_per_client(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
    query: web::Query<GetAllBudgetsPerClientQueryParams>,
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

    let page = query.page.unwrap_or(0);
    let page_size = query.page_size.unwrap_or(10);

    let condition = match &query.search {
        Some(term) if !term.is_empty() => {
            let pattern = format!("%{}%", term);
            Condition::all()
                .add(Expr::cust_with_values(
                    "unaccent(budgets.file_name) ILIKE unaccent($1)",
                    [&pattern],
                ))
                .add(Expr::cust_with_values(
                    "unaccent(budgets.created_at) ILIKE unaccent($1)",
                    [&pattern],
                ))
                .add(Expr::cust_with_values(
                    "unaccent(budgets.amount) ILIKE unaccent($1)",
                    [&pattern],
                ))
        }
        _ => Condition::all(),
    };

    let mut select = budgets::Entity::find()
        .select_only()
        .columns([
            budgets::Column::Id,
            budgets::Column::CreatedAt,
            budgets::Column::FileName,
            budgets::Column::Path,
            budgets::Column::Amount,
        ])
        .filter(budgets::Column::ClientId.eq(client.id))
        .filter(condition);

    select = match query.sort {
        GetAllBudgetsPerClientSortOrder::CreatedAtAsc => {
            select.order_by(budgets::Column::CreatedAt, Order::Asc)
        }
        GetAllBudgetsPerClientSortOrder::CreatedAtDesc => {
            select.order_by(budgets::Column::CreatedAt, Order::Desc)
        }
        GetAllBudgetsPerClientSortOrder::AmountAsc => {
            select.order_by(budgets::Column::Amount, Order::Asc)
        }
        GetAllBudgetsPerClientSortOrder::AmountDesc => {
            select.order_by(budgets::Column::Amount, Order::Desc)
        }
        GetAllBudgetsPerClientSortOrder::FileNameAsc => {
            select.order_by(budgets::Column::FileName, Order::Asc)
        }
        GetAllBudgetsPerClientSortOrder::FileNameDesc => {
            select.order_by(budgets::Column::Amount, Order::Desc)
        }
    };

    let paginator = select
        .into_model::<GetAllBudgetsPerClientDTO>()
        .paginate(db.get_ref(), page_size);

    let total_pages = match paginator.num_pages().await {
        Ok(n) => n,
        Err(err) => {
            warn!(
                "(get_all_budgets_per_client) Could not count budgets: {:?}",
                err
            );
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Something went wrong when retrieving budgets data"
            }));
        }
    };

    match paginator.fetch_page(page).await {
        Ok(found_budgets) => HttpResponse::Ok().json(json!({
            "data": found_budgets,
            "page": page,
            "page_size": page_size,
            "total_pages": total_pages,
        })),
        Err(err) => {
            warn!("(get_budgets) Could not get budgets data: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Something went wrong when retrieving budgets data"
            }))
        }
    }
}
