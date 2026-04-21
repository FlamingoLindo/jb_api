use actix_web::{HttpResponse, Responder, web};
use log::warn;
use sea_orm::sea_query::Expr;
use sea_orm::{
    Condition, DatabaseConnection, EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect,
};
use serde_json::json;

use crate::{
    dto::types::get_all::{GetTypesDTO, TypesQueryParams, TypesSortOrder},
    entities::types,
};

pub async fn get_types(
    db: web::Data<DatabaseConnection>,
    query: web::Query<TypesQueryParams>,
) -> impl Responder {
    let page = query.page.unwrap_or(0);
    let page_size = query.page_size.unwrap_or(10);

    let condition = match &query.search {
        Some(term) if !term.is_empty() => {
            let pattern = format!("%{}%", term);
            Condition::all().add(Expr::cust_with_values(
                "unaccent(types.name) ILIKE unaccent($1)",
                [&pattern],
            ))
        }
        _ => Condition::all(),
    };

    let mut select = types::Entity::find()
        .select_only()
        .columns([
            types::Column::Id,
            types::Column::Name,
            types::Column::Blocked,
            types::Column::CreatedAt,
            types::Column::UpdatedAt,
        ])
        .filter(condition);

    select = match query.sort {
        TypesSortOrder::NameDesc => select.order_by(types::Column::Name, Order::Desc),
        TypesSortOrder::NameAsc => select.order_by(types::Column::CreatedAt, Order::Asc),
        TypesSortOrder::BlockedDesc => select.order_by(types::Column::Blocked, Order::Desc),
        TypesSortOrder::BlockedAsc => select.order_by(types::Column::Blocked, Order::Asc),
        TypesSortOrder::CreateAtDesc => select.order_by(types::Column::CreatedAt, Order::Desc),
        TypesSortOrder::CreateAtAsc => select.order_by(types::Column::CreatedAt, Order::Asc),
    };

    let paginator = select
        .into_model::<GetTypesDTO>()
        .paginate(db.get_ref(), page_size);

    let total_pages = match paginator.num_pages().await {
        Ok(n) => n,
        Err(err) => {
            warn!("(get_types) Could not count types: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Something went wrong when retrieving types data"
            }));
        }
    };

    match paginator.fetch_page(page).await {
        Ok(found_types) => HttpResponse::Ok().json(json!({
            "data": found_types,
            "page": page,
            "page_size": page_size,
            "total_pages": total_pages,
        })),
        Err(err) => {
            warn!("(get_types) Could not get types data: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Something went wrong when retrieving types data"
            }))
        }
    }
}
