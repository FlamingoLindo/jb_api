use crate::entities::clients;
use chrono::NaiveDateTime;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ClientResponse {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub cpf: Option<String>,
    pub cnpj: Option<String>,
    pub observation: Option<String>,
    pub blocked: bool,
    pub zipcode: String,
    pub state: String,
    pub city: String,
    pub street: String,
    pub complement: Option<String>,
    pub neighborhood: Option<String>,
    pub number: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<clients::Model> for ClientResponse {
    fn from(client: clients::Model) -> Self {
        Self {
            name: client.name,
            email: client.email,
            phone: client.phone,
            cpf: client.cpf,
            cnpj: client.cnpj,
            observation: client.observation,
            blocked: client.blocked,
            zipcode: client.zipcode,
            state: client.state,
            city: client.city,
            street: client.street,
            complement: client.complement,
            neighborhood: client.neighborhood,
            number: client.number,
            created_at: client.created_at,
            updated_at: client.updated_at,
        }
    }
}
