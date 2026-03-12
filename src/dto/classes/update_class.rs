use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::entities::classes::Model;

#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateClassDTO {
    #[validate(length(
        min = 3,
        max = 150,
        message = "Class name must be between 3 and 150 characters"
    ))]
    pub name: String,
}

#[derive(Serialize)]
pub struct UpdateClassResponse {
    pub name: String,
    pub blocked: bool,
    pub updated_at: NaiveDateTime,
}

impl From<Model> for UpdateClassResponse {
    fn from(class: Model) -> Self {
        Self {
            name: class.name,
            blocked: class.blocked,
            updated_at: class.updated_at,
        }
    }
}
