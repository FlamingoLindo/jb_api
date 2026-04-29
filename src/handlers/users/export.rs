use crate::entities::users;
use actix_web::{HttpResponse, Responder, web};
use log::error;
use rust_xlsxwriter::{ExcelDateTime, Format, Workbook};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde_json::json;

#[utoipa::path(
    get,
    path = "/api/v1/users/export",
    tag = "Users",
    responses(
        (status = 200, description = "Users exported successfully", body = serde_json::Value),
        (status = 500, description = "Internal server error", body = serde_json::Value)
    )
)]
pub async fn export_users(db: web::Data<DatabaseConnection>) -> impl Responder {
    let users = match users::Entity::find().all(db.get_ref()).await {
        Ok(users) => users,
        Err(err) => {
            error!("(export_users) Could not fetch users: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when exporting users, please try again"
            }));
        }
    };
    let date_format = Format::new().set_num_format("yyyy-mm-dd hh:mm:ss");
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    worksheet.write(0, 0, "Username").unwrap();
    worksheet.write(0, 1, "Email").unwrap();
    worksheet.write(0, 2, "Blocked").unwrap();
    worksheet.write(0, 3, "Created At").unwrap();
    worksheet.write(0, 4, "Updated At").unwrap();

    for (i, user) in users.iter().enumerate() {
        let row = (i + 1) as u32;
        let created_at =
            ExcelDateTime::from_timestamp(user.created_at.and_utc().timestamp()).unwrap();
        let updated_at =
            ExcelDateTime::from_timestamp(user.updated_at.and_utc().timestamp()).unwrap();

        worksheet.write(row, 0, &user.username).unwrap();
        worksheet
            .write(row, 1, user.email.as_deref().unwrap_or(""))
            .unwrap();
        worksheet.write(row, 2, user.blocked).unwrap();
        worksheet
            .write_with_format(row, 3, &created_at, &date_format)
            .unwrap();
        worksheet
            .write_with_format(row, 4, &updated_at, &date_format)
            .unwrap();
    }

    match workbook.save("exports/excel/users/users.xlsx") {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "Ok",
            "message": "Users exported successfully"
        })),
        Err(err) => {
            error!("(export_users) Could not save users export file: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when exporting users, please try again"
            }))
        }
    }
}
