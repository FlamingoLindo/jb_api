use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Validate, ToSchema)]
pub struct SendForgotPasswordDTO {
    #[validate(email)]
    pub email: String,
}
