use actix_web::dev::ServiceRequest;
use actix_web::{Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    role: Option<String>,
    iat: usize,
    exp: usize,
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    dotenv::from_filename(".env").ok();
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set in env");

    let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.validate_exp = true;

    match decode::<Claims>(
        credentials.token(),
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    ) {
        Ok(token_data) => {
            req.extensions_mut().insert(token_data.claims);
            Ok(req)
        }
        Err(_) => Err((actix_web::error::ErrorUnauthorized("Invalid token"), req)),
    }
}
