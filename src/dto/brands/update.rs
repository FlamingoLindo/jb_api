use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::entities::{brands, images};

#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateBrandDTO {
    #[validate(length(
        min = 3,
        max = 150,
        message = "Brand name must be between 3 and 150 characters"
    ))]
    pub name: String,
    pub image_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct UpdateBrandResponse {
    pub name: String,
    pub image: Option<String>,
    pub updated_at: NaiveDateTime,
}

impl From<(brands::Model, Option<images::Model>)> for UpdateBrandResponse {
    fn from((brand, image): (brands::Model, Option<images::Model>)) -> Self {
        Self {
            name: brand.name,
            image: image.map(|img| img.path),
            updated_at: brand.updated_at,
        }
    }
}
