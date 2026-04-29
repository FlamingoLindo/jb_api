use chrono::NaiveDateTime;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::{IntoParams, ToSchema};

#[derive(Serialize, Deserialize, FromQueryResult, ToSchema)]
pub struct GetBrandsDTO {
    pub id: Uuid,
    pub name: String,
    pub blocked: bool,
    pub image: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Default, ToSchema)]
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

#[derive(Deserialize, IntoParams, ToSchema)]
pub struct BrandsQueryParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub search: Option<String>,
    #[serde(default)]
    pub sort: BrandsSortOrder,
}
