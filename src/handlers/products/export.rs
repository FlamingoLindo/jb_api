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

    let mut brands_map: HashMap<String, (Option<Uuid>, HashMap<String, Vec<usize>>)> =
        HashMap::new();
    for (idx, row) in rows.iter().enumerate() {
        let brand = row.brand_name.clone().unwrap_or_else(|| "Unknown".into());
        let class = row.class_name.clone().unwrap_or_else(|| "Unknown".into());
        let entry = brands_map
            .entry(brand)
            .or_insert_with(|| (row.brand_image_id, HashMap::new()));
        entry.1.entry(class).or_default().push(idx);
    }

    let _header_format = Format::new().set_bold();
    let class_format = Format::new()
        .set_bold()
        .set_font_size(11.0)
        .set_background_color(0xD9E1F2)
        .set_border(FormatBorder::Thin);
    let col_header_format = Format::new()
        .set_bold()
        .set_align(FormatAlign::Center)
        .set_background_color(0xBDD7EE)
        .set_border(FormatBorder::Thin);

    let mut workbook = Workbook::new();

    for (brand_name, (image_id, classes_map)) in &brands_map {
        let worksheet = workbook.add_worksheet();
        let sheet_name = brand_name.chars().take(31).collect::<String>();
        worksheet.set_name(&sheet_name).unwrap();

        let mut current_row: u32 = 0;

        if let Some(image_id) = image_id {
            match images::Entity::find_by_id(*image_id)
                .one(db.get_ref())
                .await
            {
                Ok(Some(img)) => {
                    let (orig_w, orig_h) = image::image_dimensions(&img.path).unwrap_or((400, 120));

                    let target_height_px: f64 = 120.0;
                    let target_width_px: f64 = 200.0;

                    let scale_h = target_height_px / orig_h as f64;
                    let scale_w = target_width_px / orig_w as f64;
                    let scale = scale_h.min(scale_w);

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

        for (class_name, product_indices) in classes_map {
            worksheet
                .write_with_format(current_row, 0, "Class", &class_format)
                .unwrap();
            worksheet
                .write_with_format(current_row, 1, class_name.as_str(), &class_format)
                .unwrap();
            current_row += 1;

            worksheet
                .write_with_format(current_row, 0, "Type", &col_header_format)
                .unwrap();
            worksheet
                .write_with_format(current_row, 1, "Price/kg", &col_header_format)
                .unwrap();
            worksheet
                .write_with_format(current_row, 2, "Price/kg no cut", &col_header_format)
                .unwrap();
            worksheet
                .write_with_format(current_row, 3, "Price/kg cut", &col_header_format)
                .unwrap();
            worksheet
                .write_with_format(current_row, 4, "Price 3mt", &col_header_format)
                .unwrap();
            worksheet
                .write_with_format(current_row, 5, "Price BR", &col_header_format)
                .unwrap();
            worksheet
                .write_with_format(current_row, 6, "Price Rod", &col_header_format)
                .unwrap();
            current_row += 1;

            for &idx in product_indices {
                let product = &rows[idx];
                let type_name = product.type_name.as_deref().unwrap_or("-");
                worksheet.write(current_row, 0, type_name).unwrap();
                worksheet
                    .write(
                        current_row,
                        1,
                        product
                            .price_kg
                            .map(|d| d.to_string())
                            .as_deref()
                            .unwrap_or("-"),
                    )
                    .unwrap();
                worksheet
                    .write(
                        current_row,
                        2,
                        product
                            .price_kg_no_cut
                            .map(|d| d.to_string())
                            .as_deref()
                            .unwrap_or("-"),
                    )
                    .unwrap();
                worksheet
                    .write(
                        current_row,
                        3,
                        product
                            .price_kg_cut
                            .map(|d| d.to_string())
                            .as_deref()
                            .unwrap_or("-"),
                    )
                    .unwrap();
                worksheet
                    .write(
                        current_row,
                        4,
                        product
                            .price_3mt
                            .map(|d| d.to_string())
                            .as_deref()
                            .unwrap_or("-"),
                    )
                    .unwrap();
                worksheet
                    .write(
                        current_row,
                        5,
                        product
                            .price_br
                            .map(|d| d.to_string())
                            .as_deref()
                            .unwrap_or("-"),
                    )
                    .unwrap();
                worksheet
                    .write(
                        current_row,
                        6,
                        product
                            .price_rod
                            .map(|d| d.to_string())
                            .as_deref()
                            .unwrap_or("-"),
                    )
                    .unwrap();
                current_row += 1;
            }

            current_row += 1;
        }

        worksheet.set_column_width(0, 20).unwrap();
        worksheet.set_column_width(1, 15).unwrap();
        worksheet.set_column_width(2, 18).unwrap();
        worksheet.set_column_width(3, 15).unwrap();
        worksheet.set_column_width(4, 12).unwrap();
        worksheet.set_column_width(5, 12).unwrap();
        worksheet.set_column_width(6, 12).unwrap();
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
