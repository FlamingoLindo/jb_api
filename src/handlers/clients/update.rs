use crate::{
    dto::clients::update::{UpdateClientDTO, UpdateClientResponse},
    entities::clients,
};
use actix_web::{HttpResponse, Responder, web};
use log::{error, warn};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

pub async fn update_client(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
    client_data: web::Json<UpdateClientDTO>,
) -> impl Responder {
    let id = id.into_inner();

    if let Err(errors) = client_data.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    let (current_client, existing_client) = tokio::join!(
        clients::Entity::find_by_id(id).one(db.get_ref()),
        clients::Entity::find()
            .filter(clients::Column::Email.eq(&client_data.email))
            .filter(clients::Column::Id.ne(id))
            .one(db.get_ref())
    );

    match current_client {
        Ok(None) => {
            warn!("(update_client) Client not found: {}", id);
            return HttpResponse::NotFound().json(json!({
                "status": "Not Found",
                "message": "Client not found"
            }));
        }
        Err(err) => {
            error!("(update_client) Database error: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Error when finding client, please try again later"
            }));
        }
        Ok(Some(_)) => {}
    }

    match existing_client {
        Ok(Some(_)) => {
            warn!("(update_client) Client with same email already exists");
            return HttpResponse::Conflict().json(json!({
                "status": "Conflict",
                "message": "Client email already in use"
            }));
        }
        Err(err) => {
            error!("(update_client) Could not check client name: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Error when finding client, please try again later"
            }));
        }
        Ok(None) => {}
    }

    let client_data = client_data.into_inner();

    let updated = clients::ActiveModel {
        id: Set(id),
        name: Set(client_data.name),
        email: Set(client_data.email),
        phone: Set(client_data.phone),
        client_type: Set(client_data.client_type),
        cpf: Set(client_data.cpf),
        cnpj: Set(client_data.cnpj),
        observation: Set(client_data.observation),
        zipcode: Set(client_data.zipcode),
        state: Set(client_data.state),
        city: Set(client_data.city),
        street: Set(client_data.street),
        complement: Set(client_data.complement),
        neighborhood: Set(client_data.neighborhood),
        number: Set(client_data.number),
        updated_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    }
    .update(db.get_ref())
    .await;

    match updated {
        Ok(updated_client) => HttpResponse::Ok().json(UpdateClientResponse::from(updated_client)),
        Err(err) => {
            error!("(update_client) Could not update client: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Error when updating client, please try again later"
            }))
        }
    }
}
