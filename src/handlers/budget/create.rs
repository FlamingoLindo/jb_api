use std::fs;
use std::path::Path;

use actix_web::{HttpResponse, Responder, web};
use log::error;
use sea_orm::prelude::Decimal;
use serde_json::json;
use typst::layout::PagedDocument;
use typst_as_lib::TypstEngine;
use typst_pdf::PdfOptions;
use uuid::Uuid;

use crate::dto::budget::create::CreateBudgetDTO;

pub async fn create_budget(data: web::Json<CreateBudgetDTO>) -> impl Responder {
    let data = data.into_inner();
    let typst_source = build_budget_typst(&data);
    let fonts = load_typst_fonts();

    if fonts.is_empty() {
        error!("(create_budget) Could not load any system fonts for Typst");
        return HttpResponse::InternalServerError().json(json!({
            "status": "Internal Server Error",
            "message": "No usable fonts were found for PDF rendering"
        }));
    }

    let engine = TypstEngine::builder()
        .main_file(typst_source)
        .fonts(fonts)
        .build();
    let compilation = engine.compile::<PagedDocument>();

    let document = match compilation.output {
        Ok(document) => document,
        Err(err) => {
            error!("(create_budget) Could not compile typst source: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Could not compile budget PDF"
            }));
        }
    };

    let pdf = match typst_pdf::pdf(&document, &PdfOptions::default()) {
        Ok(bytes) => bytes,
        Err(err) => {
            error!("(create_budget) Could not render PDF bytes: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "Could not render budget PDF"
            }));
        }
    };

    if let Err(err) = fs::create_dir_all("exports") {
        error!("(create_budget) Could not create exports dir: {:?}", err);
        return HttpResponse::InternalServerError().json(json!({
            "status": "Internal Server Error",
            "message": "Could not prepare export directory"
        }));
    }

    let file_name = format!("budget_{}.pdf", Uuid::new_v4());
    let file_path = format!("exports/{}", file_name);

    if let Err(err) = fs::write(&file_path, pdf) {
        error!("(create_budget) Could not write PDF file: {:?}", err);
        return HttpResponse::InternalServerError().json(json!({
            "status": "Internal Server Error",
            "message": "Could not save budget PDF"
        }));
    }

    HttpResponse::Ok().json(json!({
        "status": "Ok",
        "message": "Budget PDF generated successfully",
        "file": file_path
    }))
}

fn build_budget_typst(data: &CreateBudgetDTO) -> String {
    let products = data
        .products
        .iter()
        .map(|product| {
            format!(
                "    (product: \"{}\", amount: {}, size: {}, weight: {}, price: {}),",
                escape_typst_string(&product.product),
                format_decimal(&product.amount),
                format_decimal(&product.size),
                format_decimal(&product.weight),
                format_decimal(&product.price)
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    format!(
        r#"#set page(paper: "a4", margin: 2cm)
#set text(font: ("New Computer Modern", "Arial", "Liberation Sans", "DejaVu Sans"), size: 11pt)

#let budget = (
    products: (
{products}
    ),
    total_price: {total_price},
)

= Budget Report

#table(
    columns: (2fr, 1fr, 1fr, 1fr, 1fr),
    stroke: 0.5pt,
    inset: 8pt,
    fill: (col, row) =>
        if row == 0 {{ luma(210) }}
        else if calc.odd(row) {{ luma(245) }}
        else {{ white }},

    table.header(
        [*Product*], [*Amount*], [*Size*], [*Weight*], [*Price*],
    ),

    ..budget.products.map(p => (
        [#p.product],
        [#p.amount],
        [#p.size],
        [#p.weight],
        [R\$#p.price],
    )).flatten(),

    table.footer(
        table.cell(colspan: 4, align: right)[*Total*],
        [R\$#budget.total_price],
    ),
)
"#,
        products = products,
        total_price = format_decimal(&data.total_price)
    )
}

fn load_typst_fonts() -> Vec<Vec<u8>> {
    let fonts_dir = Path::new("assets/fonts");

    if !fonts_dir.exists() {
        return Vec::new();
    }

    let entries = match fs::read_dir(fonts_dir) {
        Ok(entries) => entries,
        Err(_) => return Vec::new(),
    };

    entries
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_file()
                && path
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| matches!(ext.to_ascii_lowercase().as_str(), "ttf" | "otf" | "ttc"))
                    .unwrap_or(false)
        })
        .filter_map(|path| fs::read(path).ok())
        .collect()
}

fn escape_typst_string(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

fn format_decimal(value: &Decimal) -> String {
    value.normalize().to_string()
}
