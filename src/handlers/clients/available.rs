use actix_web::{HttpResponse, Responder, web};
use log::warn;
use sea_orm::sea_query::Expr;
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect,
};
use serde_json::json;

use crate::{
    dto::clients::available::{AvailableDTO, AvailableQueryParams},
    entities::clients,
};

pub async fn available_clients(
    db: web::Data<DatabaseConnection>,
    query: web::Query<AvailableQueryParams>,
) -> impl Responder {
    let condition = match &query.search {
        Some(term) if !term.is_empty() => {
            let pattern = format!("%{}%", term);
            Condition::any().add(Expr::cust_with_values(
                "unaccent(clients.name) ILIKE unaccent($1)",
                [pattern],
            ))
        }
        _ => Condition::all(),
    };

    let available_clients = clients::Entity::find()
        .select_only()
        .columns([clients::Column::Id, clients::Column::Name])
        .filter(clients::Column::Blocked.ne(false))
        .filter(condition)
        .order_by_asc(clients::Column::Name)
        .into_model::<AvailableDTO>()
        .all(db.get_ref())
        .await;

    match available_clients {
        Ok(clients) => HttpResponse::Ok().json(clients),
        Err(err) => {
            warn!("(available_clients) Could not get clients data: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Something went wrong when retrieving clients data"
            }))
        }
    }
}
