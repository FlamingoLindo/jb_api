use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::entities::classes::Model;

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateClassDTO {
    #[validate(length(
        min = 3,
        max = 150,
        message = "Class name must be between 3 and 150 characters"
    ))]
    pub name: String,
    // pub blocked: bool,
}

#[derive(Serialize)]
pub struct CreateClassResponse {
    pub name: String,
    pub blocked: bool,
}

impl From<Model> for CreateClassResponse {
    fn from(class: Model) -> Self {
        Self {
            name: class.name,
            blocked: class.blocked,
        }
    }
}
