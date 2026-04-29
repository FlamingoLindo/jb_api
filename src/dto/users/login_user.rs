use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Validate, ToSchema)]
pub struct LoginDTO {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1, message = "Password cannot be empty"))]
    pub password: String,
}
