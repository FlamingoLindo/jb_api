use actix_web::{HttpResponse, Responder, web};
use sea_orm::{
    DatabaseConnection, EntityTrait, JoinType, PaginatorTrait, QueryOrder, QuerySelect,
    RelationTrait,
};

use crate::{
    dto::{brands::get_all::GetBrandsDTO, shared::pagination::PaginationParams},
    entities::{brands, images},
};

pub async fn get_brands(
    db: web::Data<DatabaseConnection>,
    query: web::Query<PaginationParams>,
) -> impl Responder {
    let page = query.page.unwrap_or(0);
    let page_size = query.page_size.unwrap_or(10);

    let paginator = brands::Entity::find()
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
        .order_by_asc(brands::Column::Name)
        .into_model::<GetBrandsDTO>()
        .paginate(db.get_ref(), page_size);

    let total_pages = match paginator.num_pages().await {
        Ok(n) => n,
        Err(err) => {
            log::warn!("(get_brands) Could not count brands: {:?}", err);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "Internal Server Error",
                "message": "Something went wrong when retrieving brands data"
            }));
        }
    };

    match paginator.fetch_page(page).await {
        Ok(found_brands) => HttpResponse::Ok().json(serde_json::json!({
            "data": found_brands,
            "page": page,
            "page_size": page_size,
            "total_pages": total_pages,
        })),
        Err(err) => {
            log::warn!("(get_brands) Could not get brands data: {:?}", err);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "Internal Server Error",
                "message": "Something went wrong when retrieving brands data"
            }))
        }
    }
}
