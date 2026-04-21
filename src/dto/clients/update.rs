use crate::entities::{clients::Model, sea_orm_active_enums::ClientType};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct UpdateClientDTO {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(min = 14, max = 15))]
    pub phone: String,

    pub client_type: ClientType,

    #[validate(length(min = 14, max = 14))]
    pub cpf: Option<String>,
    #[validate(length(min = 18, max = 18))]
    pub cnpj: Option<String>,
    pub observation: Option<String>,
    #[validate(length(min = 9, max = 9))]
    pub zipcode: String,
    #[validate(length(min = 2, max = 2))]
    pub state: String,
    pub city: String,
    pub street: String,
    pub complement: Option<String>,
    pub neighborhood: Option<String>,
    pub number: String,
}

#[derive(Serialize)]
pub struct UpdateClientResponse {
    pub name: String,
    pub updated_at: NaiveDateTime,
}

impl From<Model> for UpdateClientResponse {
    fn from(client: Model) -> Self {
        Self {
            name: client.name,
            updated_at: client.updated_at,
        }
    }
}
