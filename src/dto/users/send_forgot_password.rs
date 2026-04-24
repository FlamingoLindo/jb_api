use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct SendForgotPasswordDTO {
    #[validate(email)]
    pub email: String,
}
