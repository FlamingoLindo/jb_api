use actix_web::{HttpResponse, Responder, web};
use log::error;
use rust_xlsxwriter::{ExcelDateTime, Format, workbook::Workbook};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde_json::json;

use crate::entities::clients;

pub async fn export_clients(db: web::Data<DatabaseConnection>) -> impl Responder {
    let clients = match clients::Entity::find().all(db.get_ref()).await {
        Ok(clients) => clients,
        Err(err) => {
            error!("(export_clients) Could not fetch clients: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when exporting clients, please try again"
            }));
        }
    };

    let date_format = Format::new().set_num_format("yyyy-mm-dd hh:mm:ss");
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    worksheet.write(0, 0, "Name").unwrap();
    worksheet.write(0, 1, "Email").unwrap();
    worksheet.write(0, 2, "Phone").unwrap();
    worksheet.write(0, 3, "CPF/CNPJ").unwrap();
    worksheet.write(0, 4, "Zipcode").unwrap();
    worksheet.write(0, 5, "State").unwrap();
    worksheet.write(0, 6, "City").unwrap();
    worksheet.write(0, 7, "Street").unwrap();
    worksheet.write(0, 8, "Complement").unwrap();
    worksheet.write(0, 9, "Neighborhood").unwrap();
    worksheet.write(0, 10, "Observation").unwrap();
    worksheet.write(0, 11, "CreatedAt").unwrap();
    worksheet.write(0, 12, "UpdatedAt").unwrap();

    for (i, client) in clients.iter().enumerate() {
        let row = (i + 1) as u32;

        let created_at =
            ExcelDateTime::from_timestamp(client.created_at.and_utc().timestamp()).unwrap();
        let updated_at =
            ExcelDateTime::from_timestamp(client.updated_at.and_utc().timestamp()).unwrap();

        let document = client
            .cpf
            .as_deref()
            .unwrap_or_else(|| client.cnpj.as_deref().unwrap_or(""));

        worksheet.write(row, 0, &client.name).unwrap();
        worksheet.write(row, 1, &client.email).unwrap();
        worksheet.write(row, 2, &client.phone).unwrap();
        worksheet.write(row, 3, document).unwrap();
        worksheet.write(row, 4, &client.zipcode).unwrap();
        worksheet.write(row, 5, &client.state).unwrap();
        worksheet.write(row, 6, &client.city).unwrap();
        worksheet.write(row, 7, &client.street).unwrap();
        worksheet
            .write(row, 8, client.complement.as_deref().unwrap_or(""))
            .unwrap();
        worksheet
            .write(row, 9, client.neighborhood.as_deref().unwrap_or(""))
            .unwrap();
        worksheet
            .write(row, 10, client.observation.as_deref().unwrap_or(""))
            .unwrap();
        worksheet
            .write_with_format(row, 11, &created_at, &date_format)
            .unwrap();
        worksheet
            .write_with_format(row, 12, &updated_at, &date_format)
            .unwrap();
    }

    match workbook.save("exports/excel/clients/clients.xlsx") {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "Ok",
            "message": "Clients exported successfully"
        })),
        Err(err) => {
            error!(
                "(export_clients) Could not save clients export file: {:?}",
                err
            );
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when exporting clients, please try again"
            }))
        }
    }
}
