use actix_web::{HttpResponse, Responder, web};
use log::warn;
use sea_orm::sea_query::Expr;
use sea_orm::{
    Condition, DatabaseConnection, EntityTrait, JoinType, Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect
};
use serde_json::json;

use crate::entities::budgets;
use crate::{
    dto::clients::get_all::{ClientsQueryParams, ClientsSortOrder, GetClientsDTO},
    entities::clients,
};

pub async fn get_clients(
    db: web::Data<DatabaseConnection>,
    query: web::Query<ClientsQueryParams>,
) -> impl Responder {
    let page = query.page.unwrap_or(0);
    let page_size = query.page_size.unwrap_or(10);

    let condition = match &query.search {
        Some(term) if !term.is_empty() => {
            let pattern = format!("%{}%", term);
            Condition::any()
                .add(Expr::cust_with_values(
                    "unaccent(clients.name) ILIKE unaccent($1)",
                    [&pattern],
                ))
                .add(Expr::cust_with_values(
                    "unaccent(clients.cpf) ILIKE unaccent($1)",
                    [&pattern],
                ))
                .add(Expr::cust_with_values(
                    "unaccent(clients.cnpj) ILIKE unaccent($1)",
                    [&pattern],
                ))
                .add(Expr::cust_with_values(
                    "unaccent(clients.phone) ILIKE unaccent($1)",
                    [&pattern],
                ))
        }
        _ => Condition::all(),
    };

    let mut select = clients::Entity::find()
        .select_only()
        .columns([
            clients::Column::Id,
            clients::Column::Name,
            clients::Column::Cpf,
            clients::Column::Cnpj,
            clients::Column::Blocked,
            clients::Column::Phone,
            clients::Column::CreatedAt,
            clients::Column::UpdatedAt,
        ])
        .column_as(Expr::cust("COUNT(budgets.id)"), "budget_count")
        .join_rev(
            JoinType::LeftJoin,
            budgets::Entity::belongs_to(clients::Entity)
                .from(budgets::Column::ClientId)
                .to(clients::Column::Id)
                .into(),
        )
        .group_by(clients::Column::Id)
        .group_by(clients::Column::Name)
        .group_by(clients::Column::Cpf)
        .group_by(clients::Column::Cnpj)
        .group_by(clients::Column::Blocked)
        .group_by(clients::Column::Phone)
        .group_by(clients::Column::CreatedAt)
        .group_by(clients::Column::UpdatedAt)
        .filter(condition);

    select = match query.sort {
        ClientsSortOrder::NameAsc => select.order_by(clients::Column::Name, Order::Asc),
        ClientsSortOrder::NameDesc => select.order_by(clients::Column::Name, Order::Desc),
        ClientsSortOrder::PhoneAsc => select.order_by(clients::Column::Phone, Order::Asc),
        ClientsSortOrder::PhoneDesc => select.order_by(clients::Column::Phone, Order::Desc),
        ClientsSortOrder::CpfAsc => select.order_by(clients::Column::Cpf, Order::Asc),
        ClientsSortOrder::CpfDesc => select.order_by(clients::Column::Cpf, Order::Desc),
        ClientsSortOrder::CnpjAsc => select.order_by(clients::Column::Cnpj, Order::Asc),
        ClientsSortOrder::CnpjDesc => select.order_by(clients::Column::Cnpj, Order::Desc),
        ClientsSortOrder::BlockedAsc => select.order_by(clients::Column::Blocked, Order::Asc),
        ClientsSortOrder::BlockedDesc => select.order_by(clients::Column::Blocked, Order::Desc),
        ClientsSortOrder::CreatedAtAsc => select.order_by(clients::Column::CreatedAt, Order::Asc),
        ClientsSortOrder::CreatedAtDesc => select.order_by(clients::Column::CreatedAt, Order::Desc),
    };

    let paginator = select
        .into_model::<GetClientsDTO>()
        .paginate(db.get_ref(), page_size);

    let total_pages = match paginator.num_pages().await {
        Ok(n) => n,
        Err(err) => {
            warn!("(get_clients) Could not count clients: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Something went wrong when retrieving clients data"
            }));
        }
    };

    match paginator.fetch_page(page).await {
        Ok(clients) => HttpResponse::Ok().json(json!({
            "data": clients,
            "page": page,
            "page_size": page_size,
            "total_pages": total_pages,
        })),
        Err(err) => {
            warn!("(get_clients) Could not get clients data: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Something went wrong when retrieving clients data"
            }))
        }
    }
}
