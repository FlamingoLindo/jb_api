use chrono::NaiveDateTime;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::{IntoParams, ToSchema};

#[derive(Serialize, Deserialize, FromQueryResult, ToSchema)]
pub struct GetTypesDTO {
    pub id: Uuid,
    pub name: Option<String>,
    pub blocked: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Default, ToSchema)]
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

#[derive(Deserialize, IntoParams, ToSchema)]
pub struct TypesQueryParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub search: Option<String>,
    #[serde(default)]
    pub sort: TypesSortOrder,
}
