use crate::entities::users;
use actix_web::{HttpResponse, Responder, web};
use rust_xlsxwriter::{ExcelDateTime, Format, Workbook};
use sea_orm::{DatabaseConnection, EntityTrait};

pub async fn export_users(db: web::Data<DatabaseConnection>) -> impl Responder {
    let users = match users::Entity::find().all(db.get_ref()).await {
        Ok(users) => users,
        Err(err) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "Internal Server Error",
                "message": err.to_string()
            }));
        }
    };
    let date_format = Format::new().set_num_format("yyyy-mm-dd hh:mm:ss");
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    worksheet.write(0, 0, "Username").unwrap();
    worksheet.write(0, 1, "Blocked").unwrap();
    worksheet.write(0, 2, "Created At").unwrap();
    worksheet.write(0, 3, "Updated At").unwrap();

    for (i, user) in users.iter().enumerate() {
        let row = (i + 1) as u32;
        let created_at =
            ExcelDateTime::from_timestamp(user.created_at.and_utc().timestamp()).unwrap();
        let updated_at =
            ExcelDateTime::from_timestamp(user.updated_at.and_utc().timestamp()).unwrap();

        worksheet.write(row, 0, &user.username).unwrap();
        worksheet.write(row, 1, user.blocked).unwrap();
        worksheet
            .write_with_format(row, 2, &created_at, &date_format)
            .unwrap();
        worksheet
            .write_with_format(row, 3, &updated_at, &date_format)
            .unwrap();
    }

    match workbook.save("exports/users.xlsx") {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "status": "Ok",
            "message": "Users exported successfully"
        })),
        Err(err) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "Internal Server Error",
            "message": err.to_string()
        })),
    }
}
