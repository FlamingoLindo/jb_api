use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, FromQueryResult)]
pub struct SharedTypeResponse {
    #[sea_orm(alias = "type_name")]
    pub name: String,
}

#[derive(Serialize, Deserialize, FromQueryResult)]
pub struct SharedClassResponse {
    #[sea_orm(alias = "class_name")]
    pub name: String,
}

#[derive(Serialize, Deserialize, FromQueryResult)]
pub struct SharedBrandResponse {
    #[sea_orm(alias = "brand_name")]
    pub name: String,
    #[sea_orm(alias = "brand_image")]
    pub image: Option<String>,
}
