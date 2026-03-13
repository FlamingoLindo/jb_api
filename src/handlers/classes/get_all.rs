use actix_web::{HttpResponse, Responder, web};
use sea_orm::{DatabaseConnection, EntityTrait, PaginatorTrait, QueryOrder, QuerySelect};

use crate::{
    dto::{classes::get_all::GetClassesDTO, shared::pagination::PaginationParams},
    entities::classes,
};

pub async fn get_classes(
    db: web::Data<DatabaseConnection>,
    query: web::Query<PaginationParams>,
) -> impl Responder {
    let page = query.page.unwrap_or(0);
    let page_size = query.page_size.unwrap_or(10);

    let paginator = classes::Entity::find()
        .select_only()
        .columns([
            classes::Column::Id,
            classes::Column::Name,
            classes::Column::Blocked,
            classes::Column::CreatedAt,
            classes::Column::UpdatedAt,
        ])
        .order_by_asc(classes::Column::Name)
        .into_model::<GetClassesDTO>()
        .paginate(db.get_ref(), page_size);

    let total_pages = match paginator.num_pages().await {
        Ok(n) => n,
        Err(err) => {
            log::warn!("(get_classes) Could not count classes: {:?}", err);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "Internal Server Error",
                "message": "Something went wrong when retrieving classes data"
            }));
        }
    };

    match paginator.fetch_page(page).await {
        Ok(found_classes) => HttpResponse::Ok().json(serde_json::json!({
            "data": found_classes,
            "page": page,
            "page_size": page_size,
            "total_pages": total_pages,
        })),
        Err(err) => {
            log::warn!("(get_classes) Could not get classes data: {:?}", err);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "Internal Server Error",
                "message": "Something went wrong when retrieving classes data"
            }))
        }
    }
}
