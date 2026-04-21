use actix_web::{HttpResponse, Responder, web};
use log::warn;
use sea_orm::sea_query::Expr;
use sea_orm::{
    Condition, DatabaseConnection, EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect,
};
use serde_json::json;

use crate::{
    dto::classes::get_all::{ClassesQueryParams, ClassesSortOrder, GetClassesDTO},
    entities::classes,
};

pub async fn get_classes(
    db: web::Data<DatabaseConnection>,
    query: web::Query<ClassesQueryParams>,
) -> impl Responder {
    let page = query.page.unwrap_or(0);
    let page_size = query.page_size.unwrap_or(10);

    let condition = match &query.search {
        Some(term) if !term.is_empty() => {
            let pattern = format!("%{}%", term);
            Condition::all().add(Expr::cust_with_values(
                "unaccent(classes.name) ILIKE unaccent($1)",
                [pattern],
            ))
        }
        _ => Condition::all(),
    };

    let mut select = classes::Entity::find()
        .select_only()
        .columns([
            classes::Column::Id,
            classes::Column::Name,
            classes::Column::Blocked,
            classes::Column::CreatedAt,
            classes::Column::UpdatedAt,
        ])
        .filter(condition);

    select = match query.sort {
        ClassesSortOrder::NameAsc => select.order_by(classes::Column::Name, Order::Asc),
        ClassesSortOrder::NameDesc => select.order_by(classes::Column::Name, Order::Desc),
        ClassesSortOrder::CreatedAtAsc => select.order_by(classes::Column::CreatedAt, Order::Asc),
        ClassesSortOrder::CreatedAtDesc => select.order_by(classes::Column::CreatedAt, Order::Desc),
        ClassesSortOrder::BlockedAsc => select.order_by(classes::Column::Blocked, Order::Asc),
        ClassesSortOrder::BlockedDesc => select.order_by(classes::Column::Blocked, Order::Desc),
    };

    let paginator = select
        .into_model::<GetClassesDTO>()
        .paginate(db.get_ref(), page_size);

    let total_pages = match paginator.num_pages().await {
        Ok(n) => n,
        Err(err) => {
            warn!("(get_classes) Could not count classes: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Something went wrong when retrieving classes data"
            }));
        }
    };

    match paginator.fetch_page(page).await {
        Ok(found_classes) => HttpResponse::Ok().json(json!({
            "data": found_classes,
            "page": page,
            "page_size": page_size,
            "total_pages": total_pages,
        })),
        Err(err) => {
            warn!("(get_classes) Could not get classes data: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Something went wrong when retrieving classes data"
            }))
        }
    }
}
