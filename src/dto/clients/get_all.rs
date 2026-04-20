use chrono::NaiveDateTime;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromQueryResult)]
pub struct GetClientsDTO {
    pub id: Uuid,
    pub name: String,
    pub cpf: Option<String>,
    pub cnpj: Option<String>,
    pub blocked: bool,
    pub phone: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ClientsSortOrder {
    #[default]
    NameAsc,
    NameDesc,
    CreatedAtAsc,
    CreatedAtDesc,
    BlockedAsc,
    BlockedDesc,
    PhoneAsc,
    PhoneDesc,
    CpfAsc,
    CpfDesc,
    CnpjAsc,
    CnpjDesc,
}

#[derive(Deserialize)]
pub struct ClientsQueryParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub search: Option<String>,
    #[serde(default)]
    pub sort: ClientsSortOrder,
}
