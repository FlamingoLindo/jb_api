use chrono::NaiveDateTime;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::{IntoParams, ToSchema};

#[derive(Serialize, Deserialize, FromQueryResult, ToSchema)]
pub struct GetClientsDTO {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub cpf: Option<String>,
    pub cnpj: Option<String>,
    pub blocked: bool,
    pub phone: String,
    pub budget_count: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Default, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ClientsSortOrder {
    #[default]
    NameAsc,
    NameDesc,
    EmailAsc,
    EmailDesc,
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

#[derive(Deserialize, IntoParams, ToSchema)]
pub struct ClientsQueryParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub search: Option<String>,
    #[serde(default)]
    pub sort: ClientsSortOrder,
}
