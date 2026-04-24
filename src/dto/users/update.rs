use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::entities::users::Model;

#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateUserDTO {
    #[validate(email)]
    pub email: String,

    #[validate(length(
        min = 3,
        max = 20,
        message = "Username must be between 3 and 20 characters"
    ))]
    pub username: String,
}

#[derive(Serialize)]
pub struct UpdateUserResponse {
    pub name: String,
    pub updated_at: NaiveDateTime,
}

impl From<Model> for UpdateUserResponse {
    fn from(user: Model) -> Self {
        Self {
            name: user.username,
            updated_at: user.updated_at,
        }
    }
}
