use chrono::NaiveDateTime;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::{IntoParams, ToSchema};

use crate::dto::shared::responses::{SharedBrandResponse, SharedClassResponse, SharedTypeResponse};

#[derive(Serialize, Deserialize, FromQueryResult, ToSchema)]
pub struct GetProductsDTO {
    pub id: Uuid,
    pub code: String,
    pub description: String,
    pub blocked: bool,
    #[sea_orm(nested)]
    pub type_data: Option<SharedTypeResponse>,
    #[sea_orm(nested)]
    pub class_data: Option<SharedClassResponse>,
    #[sea_orm(nested)]
    pub brand_data: Option<SharedBrandResponse>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,

    pub product_image: Option<String>,
}

#[derive(Deserialize, Default, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProductsSortOrder {
    #[default]
    DescriptionAsc,
    DescriptionDesc,
    CodeAsc,
    CodeDesc,
    BlockedAsc,
    BlockedDesc,
    TypeAsc,
    TypeDesc,
    ClassAsc,
    ClassDesc,
    BrandAsc,
    BrandDesc,
}

#[derive(Deserialize, IntoParams, ToSchema)]
pub struct ProductsQueryParams {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub search: Option<String>,
    #[serde(default)]
    pub sort: ProductsSortOrder,
}
