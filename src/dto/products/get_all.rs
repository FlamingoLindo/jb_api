use chrono::NaiveDateTime;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::dto::shared::responses::{SharedBrandResponse, SharedClassResponse, SharedTypeResponse};

#[derive(Serialize, Deserialize, FromQueryResult)]
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
