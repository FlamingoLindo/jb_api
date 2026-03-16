use chrono::NaiveDateTime;
use sea_orm::prelude::Decimal;
use serde::Serialize;

use crate::{
    dto::shared::responses::{SharedBrandResponse, SharedClassResponse, SharedTypeResponse},
    entities::{brands, classes, images, products, types},
};

#[derive(Serialize)]
pub struct ProductResponse {
    pub code: String,
    pub description: String,
    pub blocked: bool,
    pub type_data: Option<SharedTypeResponse>,
    pub class_data: Option<SharedClassResponse>,
    pub brand_data: Option<SharedBrandResponse>,

    pub price_kg: Option<Decimal>,
    pub price_kg_no_cut: Option<Decimal>,
    pub price_kg_cut: Option<Decimal>,
    pub price_3mt: Option<Decimal>,
    pub price_br: Option<Decimal>,
    pub price_rod: Option<Decimal>,
    pub weight_3mts: Option<f64>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl
    From<(
        products::Model,
        Option<types::Model>,
        Option<classes::Model>,
        Option<brands::Model>,
        Option<images::Model>,
    )> for ProductResponse
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

            price_kg: product.price_kg,
            price_kg_no_cut: product.price_kg_no_cut,
            price_kg_cut: product.price_kg_cut,
            price_3mt: product.price_3mt,
            price_br: product.price_br,
            price_rod: product.price_rod,
            weight_3mts: product.weight_3mts,

            created_at: product.created_at,
            updated_at: product.updated_at,
        }
    }
}
