use crate::entities::sea_orm_active_enums::ClientType;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::entities::clients::Model;

#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateClientDTO {
    #[validate(length(min = 1, max = 255))]
    pub name: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 14, max = 15))]
    pub phone: String,

    #[schema(value_type = String, example = "pf")]
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

#[derive(Serialize, ToSchema)]
pub struct CreateClientResponse {
    pub name: String,
    pub created_at: NaiveDateTime,
}

impl From<Model> for CreateClientResponse {
    fn from(client: Model) -> Self {
        Self {
            name: client.name,
            created_at: client.created_at,
        }
    }
}
