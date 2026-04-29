use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ProductData {
    pub product: String,
    #[schema(value_type = f64)]
    pub amount: Decimal,
    #[schema(value_type = f64)]
    pub size: Decimal,
    #[schema(value_type = f64)]
    pub weight: Decimal,
    #[schema(value_type = f64)]
    pub price: Decimal,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateBudgetDTO {
    pub client: Uuid,
    pub products: Vec<ProductData>,
    #[schema(value_type = f64)]
    pub total_price: Decimal,
}
