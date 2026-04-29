use actix_web::{HttpResponse, Responder, web};
use log::{error, warn};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

use crate::{
    dto::clients::create::{CreateClientDTO, CreateClientResponse},
    entities::{clients, sea_orm_active_enums::ClientType},
};

#[utoipa::path(
    post,
    path = "/api/v1/clients/register",
    tag = "Clients",
    request_body = CreateClientDTO,
    responses(
        (status = 201, description = "Client created successfully", body = CreateClientResponse),
        (status = 400, description = "Validation error", body = serde_json::Value),
        (status = 409, description = "Email, name, phone, or CPF/CNPJ already in use", body = serde_json::Value),
        (status = 500, description = "Internal server error", body = serde_json::Value)
    )
)]
pub async fn create_client(
    db: web::Data<DatabaseConnection>,
    client: web::Json<CreateClientDTO>,
) -> impl Responder {
    if let Err(errors) = client.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    // Check email
    match clients::Entity::find()
        .filter(clients::Column::Email.eq(&client.email.to_lowercase()))
        .one(db.get_ref())
        .await
    {
        Ok(Some(_)) => {
            warn!("(create_client) Email already in use");
            return HttpResponse::Conflict().json(json!({
                "status": "Conflict",
                "message": "Invalid data"
            }));
        }
        Err(err) => {
            error!("(create_client) Could not check client email: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Something went wrong"
            }));
        }
        Ok(None) => {}
    }

    // Check name
    match clients::Entity::find()
        .filter(clients::Column::Name.eq(&client.name))
        .one(db.get_ref())
        .await
    {
        Ok(Some(_)) => {
            warn!("(create_client) Name already in use");
            return HttpResponse::Conflict().json(json!({
                "status": "Conflict",
                "message": "Invalid data"
            }));
        }
        Err(err) => {
            error!("(create_client) Could not check client name: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Something went wrong"
            }));
        }
        Ok(None) => {}
    }

    // Check phone
    match clients::Entity::find()
        .filter(clients::Column::Phone.eq(&client.phone))
        .one(db.get_ref())
        .await
    {
        Ok(Some(_)) => {
            warn!("(create_client) Phone already in use");
            return HttpResponse::Conflict().json(json!({
                "status": "Conflict",
                "message": "Invalid data"
            }));
        }
        Err(err) => {
            error!("(create_client) Could not check client phone: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Something went wrong"
            }));
        }
        Ok(None) => {}
    }

    // Check cpf/cnpj depending on type
    match client.client_type {
        ClientType::Pf => {
            if let Some(cpf) = &client.cpf {
                match clients::Entity::find()
                    .filter(clients::Column::Cpf.eq(cpf))
                    .one(db.get_ref())
                    .await
                {
                    Ok(Some(_)) => {
                        warn!("(create_client) CPF already in use");
                        return HttpResponse::Conflict().json(json!({
                            "status": "Conflict",
                            "message": "Invalid data"
                        }));
                    }
                    Err(err) => {
                        error!("(create_client) Could not check client CPF: {:?}", err);
                        return HttpResponse::InternalServerError().json(json!({
                            "status": "Internal Server Error",
                            "message": "Something went wrong"
                        }));
                    }
                    Ok(None) => {}
                }
            } else {
                warn!("(create_client) Missing CPF");
                return HttpResponse::BadRequest().json(json!({
                    "status": "Bad Request",
                    "message": "CPF is required for PF clients"
                }));
            }
        }
        ClientType::Pj => {
            if let Some(cnpj) = &client.cnpj {
                match clients::Entity::find()
                    .filter(clients::Column::Cnpj.eq(cnpj))
                    .one(db.get_ref())
                    .await
                {
                    Ok(Some(_)) => {
                        warn!("(create_client) CPNJ already in use");
                        return HttpResponse::Conflict().json(json!({
                            "status": "Conflict",
                            "message": "Invalid data"
                        }));
                    }
                    Err(err) => {
                        error!("(create_client) Could not check client CNPJ: {:?}", err);
                        return HttpResponse::InternalServerError().json(json!({
                            "status": "Internal Server Error",
                            "message": "Something went wrong"
                        }));
                    }
                    Ok(None) => {}
                }
            } else {
                warn!("(create_client) Missing CPNJ");
                return HttpResponse::BadRequest().json(json!({
                    "status": "Bad Request",
                    "message": "CNPJ is required for PJ clients"
                }));
            }
        }
    }

    let client = client.into_inner();

    let new_client = clients::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(client.name),
        email: Set(client.email.to_lowercase()),
        phone: Set(client.phone),
        client_type: Set(client.client_type),
        cpf: Set(client.cpf),
        cnpj: Set(client.cnpj),
        observation: Set(client.observation),
        zipcode: Set(client.zipcode),
        state: Set(client.state),
        city: Set(client.city),
        street: Set(client.street),
        complement: Set(client.complement),
        neighborhood: Set(client.neighborhood),
        number: Set(client.number),
        ..Default::default()
    }
    .insert(db.get_ref())
    .await;

    match new_client {
        Ok(client) => HttpResponse::Created().json(CreateClientResponse::from(client)),
        Err(err) => {
            error!("Could not insert client: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Something went wrong when creating client"
            }))
        }
    }
}
