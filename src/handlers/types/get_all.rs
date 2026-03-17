use actix_web::{HttpResponse, Responder, web};
use log::warn;
use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait, QueryOrder, QuerySelect};
use serde_json::json;

use crate::{
    dto::{shared::pagination::PaginationParams, types::get_all::GetTypesDTO},
    entities::types,
};

pub async fn get_types(
    db: web::Data<DatabaseConnection>,
    query: web::Query<PaginationParams>,
) -> impl Responder {
    let page = query.page.unwrap_or(0);
    let page_size = query.page.unwrap_or(10);

    let paginator = types::Entity::find()
        .select_only()
        .columns([
            types::Column::Id,
            types::Column::Name,
            types::Column::Blocked,
            types::Column::CreatedAt,
            types::Column::UpdatedAt,
        ])
        .order_by_asc(types::Column::Name)
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
