use chrono::NaiveDateTime;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromQueryResult)]
pub struct GetTypesDTO {
    pub id: Uuid,
    pub name: Option<String>,
    pub blocked: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TypesSortOrder {
    #[default]
    NameAsc,
    NameDesc,
    BlockedAsc,
    BlockedDesc,
    CreateAtAsc,
    CreateAtDesc,
}

#[derive(Deserialize)]
pub struct TypesQueryParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub search: Option<String>,
    #[serde(default)]
    pub sort: TypesSortOrder,
}
