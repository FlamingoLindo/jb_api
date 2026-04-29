use crate::entities::{brands, images};
use chrono::NaiveDateTime;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct BrandResponse {
    pub name: String,
    pub image: Option<String>,
    pub blocked: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<(brands::Model, Option<images::Model>)> for BrandResponse {
    fn from((brand, image): (brands::Model, Option<images::Model>)) -> Self {
        Self {
            name: brand.name,
            image: image.map(|img| img.path),
            blocked: brand.blocked,
            created_at: brand.created_at,
            updated_at: brand.updated_at,
        }
    }
}
