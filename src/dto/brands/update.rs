use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::entities::brands::Model;

#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateBrandDTO {
    #[validate(length(
        min = 3,
        max = 150,
        message = "Brand name must be between 3 and 150 characters"
    ))]
    pub name: String,
}

#[derive(Serialize)]
pub struct UpdateBrandResponse {
    pub name: String,
    pub updated_at: NaiveDateTime,
}

impl From<Model> for UpdateBrandResponse {
    fn from(brand: Model) -> Self {
        Self {
            name: brand.name,
            updated_at: brand.updated_at,
        }
    }
}
