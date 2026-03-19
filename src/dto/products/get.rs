use chrono::NaiveDateTime;
use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};

use crate::{
    dto::shared::responses::{SharedBrandResponse, SharedClassResponse, SharedTypeResponse},
    entities::{brands, classes, images, products, types},
};

#[derive(Serialize, Deserialize)]
pub struct ImageData {
    pub path: Vec<String>,
}

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
    pub weight_3mts: Option<Decimal>,

    pub price_p_mt: Option<Decimal>,
    pub cut_percentage: Option<Decimal>,
    pub weight_p_mm: Option<Decimal>,
    pub weight: Option<Decimal>,
    pub weight_esp: Option<Decimal>,
    pub weight_p_br: Option<Decimal>,
    pub br_price: Option<Decimal>,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,

    pub image_data: Option<ImageData>,
}

impl
    From<(
        products::Model,
        Option<types::Model>,
        Option<classes::Model>,
        Option<brands::Model>,
        Option<images::Model>, // brand
        Vec<images::Model>,    // product
    )> for ProductResponse
{
    fn from(
        (product, type_data, class_data, brand_data, brand_image, product_images): (
            products::Model,
            Option<types::Model>,
            Option<classes::Model>,
            Option<brands::Model>,
            Option<images::Model>,
            Vec<images::Model>,
        ),
    ) -> Self {
        let paths: Vec<String> = product_images.into_iter().map(|i| i.path).collect();
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

            price_p_mt: product.price_p_mt,
            cut_percentage: product.cut_percentage,
            weight_p_mm: product.weight_p_mm,
            weight: product.weight,
            weight_esp: product.weight_esp,
            weight_p_br: product.weight_p_br,
            br_price: product.br_price,

            created_at: product.created_at,
            updated_at: product.updated_at,

            image_data: if paths.is_empty() {
                None
            } else {
                Some(ImageData { path: paths })
            },
        }
    }
}
