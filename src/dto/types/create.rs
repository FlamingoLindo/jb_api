use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::entities::types::Model;

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateTypeDTO {
    #[validate(length(
        min = 3,
        max = 150,
        message = "Type name must be between 3 and 150 characters"
    ))]
    pub name: String,
}

#[derive(Serialize)]
pub struct CreateTypeResponse {
    pub name: String,
    pub blocked: bool,
}

impl From<Model> for CreateTypeResponse {
    fn from(created_type: Model) -> Self {
        Self {
            name: created_type.name,
            blocked: created_type.blocked,
        }
    }
}
