use actix_web::{HttpResponse, Responder, web};
use log::error;
use rust_xlsxwriter::{Format, FormatAlign, FormatBorder, Image, Workbook};
use sea_orm::{
    DatabaseConnection, EntityTrait, Iterable, JoinType, QuerySelect, RelationTrait,
    prelude::Decimal,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;

use crate::entities::{brands, brands_images, classes, images, products, types};

#[derive(Debug, Serialize, Deserialize, sea_orm::FromQueryResult)]
pub struct ProductExportRow {
    pub id: Uuid,
    pub code: String,
    pub description: String,
    pub blocked: bool,
    pub price_kg: Option<Decimal>,
    pub price_kg_no_cut: Option<Decimal>,
    pub price_kg_cut: Option<Decimal>,
    pub price_3mt: Option<Decimal>,
    pub price_br: Option<Decimal>,
    pub price_rod: Option<Decimal>,
    pub price_p_mt: Option<Decimal>,
    pub br_price: Option<Decimal>,
    pub weight: Option<Decimal>,
    pub weight_3mts: Option<Decimal>,
    pub weight_esp: Option<Decimal>,
    pub weight_p_br: Option<Decimal>,
    pub weight_p_mm: Option<Decimal>,
    pub cut_percentage: Option<Decimal>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub brand_name: Option<String>,
    pub class_name: Option<String>,
    pub type_name: Option<String>,
    pub brand_image_id: Option<Uuid>,
}

#[utoipa::path(
    get,
    path = "/api/v1/products/export",
    tag = "Products",
    responses(
        (status = 200, description = "Products exported successfully", body = serde_json::Value),
        (status = 500, description = "Internal server error", body = serde_json::Value)
    )
)]
pub async fn export_products(db: web::Data<DatabaseConnection>) -> impl Responder {
    let rows = match products::Entity::find()
        .columns(products::Column::iter())
        .column_as(brands::Column::Name, "brand_name")
        .column_as(classes::Column::Name, "class_name")
        .column_as(types::Column::Name, "type_name")
        .column_as(brands_images::Column::ImageId, "brand_image_id")
        .join(JoinType::LeftJoin, products::Relation::Brands.def())
        .join_rev(JoinType::LeftJoin, brands_images::Relation::Brands.def())
        .join(JoinType::LeftJoin, products::Relation::Classes.def())
        .join(JoinType::LeftJoin, products::Relation::Types.def())
        .into_model::<ProductExportRow>()
        .all(db.get_ref())
        .await
    {
        Ok(rows) => rows,
        Err(err) => {
            error!("(export_products) Could not fetch products: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when finding products, please try again"
            }));
        }
    };

    // Group: brand -> (image_id, type -> Vec<row_index>)
    let mut brands_map: HashMap<String, (Option<Uuid>, HashMap<String, Vec<usize>>)> =
        HashMap::new();
    for (idx, row) in rows.iter().enumerate() {
        let brand = row.brand_name.clone().unwrap_or_else(|| "Unknown".into());
        let type_key = row.type_name.clone().unwrap_or_else(|| "Unknown".into());
        let entry = brands_map
            .entry(brand)
            .or_insert_with(|| (row.brand_image_id, HashMap::new()));
        entry.1.entry(type_key).or_default().push(idx);
    }

    // Formats
    let type_header_format = Format::new()
        .set_bold()
        .set_font_color(rust_xlsxwriter::Color::Red)
        .set_font_size(11.0)
        .set_background_color(0xFFFF00)
        .set_border(FormatBorder::Thin);

    let col_header_format = Format::new()
        .set_bold()
        .set_align(FormatAlign::Center)
        .set_background_color(0xBDD7EE)
        .set_border(FormatBorder::Thin);

    let data_format = Format::new().set_border(FormatBorder::Thin);

    let mut workbook = Workbook::new();

    // Column definitions: (header label, width)
    let columns: &[(&str, f64)] = &[
        ("DESCRIÇÃO", 30.0),
        ("PESO/MT", 10.0),
        ("VALOR KG S/CORTE", 16.0),
        ("VALOR P/MT", 12.0),
        ("% DO CORTE", 10.0),
        ("VALOR KG C/CORTE", 16.0),
        ("PÇS", 8.0),
        ("PESO/MM", 10.0),
        ("COMPRIMENTO MM", 14.0),
        ("VALOR C/CORTE", 13.0),
        ("PESO KG", 10.0),
        ("VALOR S/CORTE", 13.0),
    ];

    for (brand_name, (image_id, types_map)) in &brands_map {
        let worksheet = workbook.add_worksheet();
        let sheet_name = brand_name.chars().take(31).collect::<String>();
        worksheet.set_name(&sheet_name).unwrap();

        let mut current_row: u32 = 0;

        // Set column widths
        for (col_idx, (_, width)) in columns.iter().enumerate() {
            worksheet.set_column_width(col_idx as u16, *width).unwrap();
        }

        // Brand image
        if let Some(image_id) = image_id {
            match images::Entity::find_by_id(*image_id)
                .one(db.get_ref())
                .await
            {
                Ok(Some(img)) => {
                    let (orig_w, orig_h) = image::image_dimensions(&img.path).unwrap_or((400, 120));

                    let target_height_px: f64 = 120.0;
                    let target_width_px: f64 = 200.0;

                    let scale =
                        (target_height_px / orig_h as f64).min(target_width_px / orig_w as f64);

                    match Image::new(&img.path) {
                        Ok(xlsx_image) => {
                            let xlsx_image =
                                xlsx_image.set_scale_width(scale).set_scale_height(scale);
                            worksheet.insert_image(current_row, 0, &xlsx_image).unwrap();
                            let rows_needed = (target_height_px / 20.0).ceil() as u32;
                            current_row += rows_needed + 1;
                        }
                        Err(err) => {
                            error!(
                                "(export_products) Could not load image {}: {:?}",
                                img.path, err
                            );
                            current_row += 1;
                        }
                    }
                }
                Ok(None) => {
                    error!("(export_products) Image not found for id: {:?}", image_id);
                }
                Err(err) => {
                    error!(
                        "(export_products) Could not fetch image {:?}: {:?}",
                        image_id, err
                    );
                }
            }
        }

        // Write each type block
        for (type_name, product_indices) in types_map {
            // Type header row (yellow, like "REDONDO" / "SEXTAVADO")
            // Type header row — only col 0 gets the type name; rest are empty but styled
            for (col_idx, _) in columns.iter().enumerate() {
                let value = if col_idx == 0 { type_name.as_str() } else { "" };
                worksheet
                    .write_with_format(current_row, col_idx as u16, value, &type_header_format)
                    .unwrap();
            }
            current_row += 1;

            // Column sub-header row (blue)
            for (col_idx, (label, _)) in columns.iter().enumerate() {
                worksheet
                    .write_with_format(current_row, col_idx as u16, *label, &col_header_format)
                    .unwrap();
            }
            current_row += 1;

            // Product rows
            for &idx in product_indices {
                let p = &rows[idx];

                let dec = |v: Option<Decimal>| -> String {
                    v.map(|d| d.to_string()).unwrap_or_else(|| "-".into())
                };

                let values: &[String] = &[
                    p.description.clone(),
                    dec(p.weight),
                    dec(p.price_kg),
                    dec(p.price_p_mt),
                    p.cut_percentage
                        .map(|d| format!("{}%", d))
                        .unwrap_or_else(|| "-".into()),
                    dec(p.price_kg_cut),
                    "-".into(), // PÇS — no field for this yet
                    dec(p.weight_p_mm),
                    "-".into(),             // COMPRIMENTO MM — no field for this yet
                    dec(p.price_kg_cut),    // VALOR C/CORTE
                    dec(p.weight),          // PESO KG
                    dec(p.price_kg_no_cut), // VALOR S/CORTE
                ];

                for (col_idx, val) in values.iter().enumerate() {
                    worksheet
                        .write_with_format(current_row, col_idx as u16, val.as_str(), &data_format)
                        .unwrap();
                }

                current_row += 1;
            }

            // Blank row between type blocks
            current_row += 1;
        }
    }

    std::fs::create_dir_all("exports/excel/products").ok();

    match workbook.save("exports/excel/products/products.xlsx") {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "Ok",
            "message": "Products exported successfully"
        })),
        Err(err) => {
            error!(
                "(export_products) Could not save products export file: {:?}",
                err
            );
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when exporting products, please try again"
            }))
        }
    }
}
