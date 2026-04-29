use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, FromQueryResult, ToSchema)]
pub struct SharedTypeResponse {
    #[sea_orm(alias = "type_name")]
    pub name: String,
}

#[derive(Serialize, Deserialize, FromQueryResult, ToSchema)]
pub struct SharedClassResponse {
    #[sea_orm(alias = "class_name")]
    pub name: String,
}

#[derive(Serialize, Deserialize, FromQueryResult, ToSchema)]
pub struct SharedBrandResponse {
    #[sea_orm(alias = "brand_name")]
    pub name: String,
    #[sea_orm(alias = "brand_image")]
    pub image: Option<String>,
}
