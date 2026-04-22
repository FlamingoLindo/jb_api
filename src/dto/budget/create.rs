use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct ProductData {
    pub product: String,
    pub amount: Decimal,
    pub size: Decimal,
    pub weight: Decimal,
    pub price: Decimal,
}

#[derive(Serialize, Deserialize)]
pub struct CreateBudgetDTO {
    pub client: Uuid,
    pub products: Vec<ProductData>,
    pub total_price: Decimal,
}
