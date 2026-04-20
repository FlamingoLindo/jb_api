use actix_web::{HttpResponse, Responder, web};
use log::warn;
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, JoinType, Order, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, RelationTrait,
};
use serde_json::json;

use crate::{
    dto::brands::get_all::{BrandsQueryParams, BrandsSortOrder, GetBrandsDTO},
    entities::{brands, images},
};

pub async fn get_brands(
    db: web::Data<DatabaseConnection>,
    query: web::Query<BrandsQueryParams>,
) -> impl Responder {
    let page = query.page.unwrap_or(0);
    let page_size = query.page_size.unwrap_or(10);

    let condition = match &query.search {
        Some(term) if !term.is_empty() => {
            let pattern = format!("%{}%", term);
            Condition::all().add(brands::Column::Name.ilike(&pattern))
        }
        _ => Condition::all(),
    };

    let mut select = brands::Entity::find()
        .select_only()
        .columns([
            brands::Column::Id,
            brands::Column::Name,
            brands::Column::Blocked,
            brands::Column::CreatedAt,
            brands::Column::UpdatedAt,
        ])
        .column_as(images::Column::Path, "image")
        .join(JoinType::LeftJoin, brands::Relation::Images.def())
        .filter(condition);

    select = match query.sort {
        BrandsSortOrder::NameDesc => select.order_by(brands::Column::Name, Order::Desc),
        BrandsSortOrder::CreatedAtAsc => select.order_by(brands::Column::CreatedAt, Order::Asc),
        BrandsSortOrder::CreatedAtDesc => select.order_by(brands::Column::CreatedAt, Order::Desc),
        BrandsSortOrder::NameAsc => select.order_by(brands::Column::Name, Order::Asc),
        BrandsSortOrder::BlockedAsc => select.order_by(brands::Column::Blocked, Order::Asc),
        BrandsSortOrder::BlockedDesc => select.order_by(brands::Column::Blocked, Order::Desc),
    };

    let paginator = select
        .into_model::<GetBrandsDTO>()
        .paginate(db.get_ref(), page_size);

    let total_pages = match paginator.num_pages().await {
        Ok(n) => n,
        Err(err) => {
            warn!("(get_brands) Could not count brands: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Something went wrong when retrieving brands data"
            }));
        }
    };

    match paginator.fetch_page(page).await {
        Ok(found_brands) => HttpResponse::Ok().json(json!({
            "data": found_brands,
            "page": page,
            "page_size": page_size,
            "total_pages": total_pages,
        })),
        Err(err) => {
            warn!("(get_brands) Could not get brands data: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Something went wrong when retrieving brands data"
            }))
        }
    }
}
