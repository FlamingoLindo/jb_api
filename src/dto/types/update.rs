use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::entities::types::Model;

#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateTypeDTO {
    #[validate(length(
        min = 3,
        max = 150,
        message = "Type name must be between 3 and 150 characters"
    ))]
    pub name: String,
}

#[derive(Serialize)]
pub struct UpdateTypeResponse {
    pub name: String,
    pub blocked: bool,
    pub updated_at: NaiveDateTime,
}

impl From<Model> for UpdateTypeResponse {
    fn from(type_data: Model) -> Self {
        Self {
            name: type_data.name,
            blocked: type_data.blocked,
            updated_at: type_data.updated_at,
        }
    }
}
