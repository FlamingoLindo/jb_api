use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::entities::users::Model;

fn validate_password(password: &str) -> Result<(), ValidationError> {
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password.chars().any(|c| !c.is_alphanumeric());

    if !has_digit {
        let mut err = ValidationError::new("invalid_password");
        err.message = Some("Password must contain at least one number".into());
        return Err(err);
    }

    if !has_special {
        let mut err = ValidationError::new("invalid_password");
        err.message = Some("Password must contain at least one special character".into());
        return Err(err);
    }
    Ok(())
}

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateUserDTO {
    #[validate(length(
        min = 3,
        max = 20,
        message = "Username must be between 3 and 20 characters"
    ))]
    pub username: String,

    #[validate(
        length(
            min = 8,
            max = 250,
            message = "Password must be between 8 and 250 characters"
        ),
        custom(function = "validate_password")
    )]
    pub password: String,

    // #[validate(required(message = "Blocked status must be provided"))]
    pub blocked: bool,
}

#[derive(Serialize)]
pub struct CreateUserResponseDTO {
    pub username: String,
    pub blocked: bool,
}

impl From<Model> for CreateUserResponseDTO {
    fn from(user: Model) -> Self {
        Self {
            username: user.username,
            blocked: user.blocked,
        }
    }
}
