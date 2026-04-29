use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Validate, ToSchema)]
pub struct ReadjustPriceDTO {
    #[validate(length(min = 1, message = "At least one ID is required"))]
    pub ids: Vec<Uuid>,
    #[schema(value_type = f64)]
    pub percentage: Decimal,
}
