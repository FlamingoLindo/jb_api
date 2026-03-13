use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::entities::brands::Model;

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateBrandDTO {
    #[validate(length(
        min = 3,
        max = 50,
        message = "Brand name must be between 3 and 50 characters"
    ))]
    pub name: String,

    pub image_id: Option<Uuid>, // pub blocked: bool,
                                // pub created_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct CreateBrandResponse {
    pub name: String,
    pub blocked: bool,
}

impl From<Model> for CreateBrandResponse {
    fn from(brand: Model) -> Self {
        Self {
            name: brand.name,
            blocked: brand.blocked,
        }
    }
}
