use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct ReadjustPriceDTO {
    #[validate(length(min = 1, message = "At least one ID is required"))]
    pub ids: Vec<Uuid>,
    pub percentage: Decimal,
}
