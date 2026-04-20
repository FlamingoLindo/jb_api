use chrono::NaiveDateTime;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromQueryResult)]
pub struct GetBrandsDTO {
    pub id: Uuid,
    pub name: String,
    pub blocked: bool,
    pub image: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum BrandsSortOrder {
    #[default]
    NameAsc,
    NameDesc,
    CreatedAtAsc,
    CreatedAtDesc,
    BlockedAsc,
    BlockedDesc,
}

#[derive(Deserialize)]
pub struct BrandsQueryParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub search: Option<String>,
    #[serde(default)]
    pub sort: BrandsSortOrder,
}
