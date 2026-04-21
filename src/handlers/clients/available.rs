use actix_web::{HttpResponse, Responder, web};
use log::warn;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use serde_json::json;

use crate::{dto::clients::available::AvailableDTO, entities::clients};

pub async fn available_clients(db: web::Data<DatabaseConnection>) -> impl Responder {
    let available_clients = clients::Entity::find()
        .select_only()
        .columns([clients::Column::Id, clients::Column::Name])
        .filter(clients::Column::Blocked.ne(false))
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
