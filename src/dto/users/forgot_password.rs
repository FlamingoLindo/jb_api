use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use utoipa::ToSchema;

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

fn validate_passwords_match(dto: &ForgotPasswordDTO) -> Result<(), ValidationError> {
    if dto.new_password != dto.confirm_password {
        let mut err = ValidationError::new("passwords_mismatch");
        err.message = Some("Passwords do not match".into());
        return Err(err);
    }
    Ok(())
}

#[derive(Serialize, Deserialize, Validate, ToSchema)]
#[validate(schema(function = "validate_passwords_match", skip_on_field_errors = false))]
pub struct ForgotPasswordDTO {
    #[validate(
        length(
            min = 8,
            max = 250,
            message = "Password must be between 8 and 250 characters"
        ),
        custom(function = "validate_password")
    )]
    pub new_password: String,
    #[validate(
        length(
            min = 8,
            max = 250,
            message = "Password must be between 8 and 250 characters"
        ),
        custom(function = "validate_password")
    )]
    pub confirm_password: String,
}
