use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};

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
    pub products: Vec<ProductData>,
    pub total_price: Decimal,
}
