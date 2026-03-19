use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{
    dto::shared::responses::{SharedBrandResponse, SharedClassResponse, SharedTypeResponse},
    entities::{brands, classes, images, products, types},
};

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateProductDTO {
    #[validate(length(
        min = 3,
        max = 350,
        message = "Code must be between 3 and 350 characters"
    ))]
    pub code: String,
    #[validate(length(
        min = 3,
        max = 550,
        message = "Description must be between 3 and 550 characters"
    ))]
    pub description: String,
    pub type_id: Option<Uuid>,
    pub class_id: Option<Uuid>,
    pub brand_id: Option<Uuid>,
    pub price_kg: Option<Decimal>,
    pub price_kg_no_cut: Option<Decimal>,
    pub price_kg_cut: Option<Decimal>,
    pub price_3mt: Option<Decimal>,
    pub price_br: Option<Decimal>,
    pub price_rod: Option<Decimal>,
    pub weight_3mts: Option<Decimal>,

    pub price_p_mt: Option<Decimal>,
    pub cut_percentage: Option<Decimal>,
    pub weight_p_mm: Option<Decimal>,
    pub weight: Option<Decimal>,
    pub weight_esp: Option<Decimal>,
    pub weight_p_br: Option<Decimal>,
    pub br_price: Option<Decimal>,
}

#[derive(Serialize)]
pub struct CreateProductResponse {
    pub code: String,
    pub description: String,
    pub blocked: bool,
    pub type_data: Option<SharedTypeResponse>,
    pub class_data: Option<SharedClassResponse>,
    pub brand_data: Option<SharedBrandResponse>,
}

impl
    From<(
        products::Model,
        Option<types::Model>,
        Option<classes::Model>,
        Option<brands::Model>,
        Option<images::Model>,
    )> for CreateProductResponse
{
    fn from(
        (product, type_data, class_data, brand_data, brand_image): (
            products::Model,
            Option<types::Model>,
            Option<classes::Model>,
            Option<brands::Model>,
            Option<images::Model>,
        ),
    ) -> Self {
        Self {
            code: product.code,
            description: product.description,
            blocked: product.blocked,
            type_data: type_data.map(|t| SharedTypeResponse { name: t.name }),
            class_data: class_data.map(|t| SharedClassResponse { name: t.name }),
            brand_data: brand_data.map(|b| SharedBrandResponse {
                name: b.name,
                image: brand_image.map(|i| i.path),
            }),
        }
    }
}
