use chrono::NaiveDateTime;
use sea_orm::{FromQueryResult, prelude::Decimal};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::{IntoParams, ToSchema};

#[derive(Serialize, Deserialize, FromQueryResult, ToSchema)]
pub struct GetAllBudgetsPerClientDTO {
    pub id: Uuid,
    pub file_name: String,
    pub path: String,
    #[schema(value_type = f64)]
    pub amount: Decimal,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize, Default, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum GetAllBudgetsPerClientSortOrder {
    #[default]
    CreatedAtDesc,
    CreatedAtAsc,
    FileNameAsc,
    FileNameDesc,
    AmountAsc,
    AmountDesc,
}

#[derive(Deserialize, IntoParams, ToSchema)]
pub struct GetAllBudgetsPerClientQueryParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub search: Option<String>,
    #[serde(default)]
    pub sort: GetAllBudgetsPerClientSortOrder,
}
