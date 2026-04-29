use actix_multipart::form::{MultipartForm, tempfile::TempFile};
use actix_web::{HttpResponse, Responder, web};
use log::{error, info, warn};
use mime::IMAGE;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection};
use serde_json::json;
use uuid::Uuid;

use crate::entities::images;

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

#[utoipa::path(
    post,
    path = "/api/v1/images/upload/{entity}",
    tag = "Images",
    params(("entity" = String, Path, description = "Entity type (brands, products, users)")),
    responses(
        (status = 200, description = "Images uploaded successfully", body = serde_json::Value),
        (status = 400, description = "Invalid file format", body = serde_json::Value),
        (status = 500, description = "Internal server error", body = serde_json::Value)
    )
)]
pub async fn save_file(
    db: web::Data<DatabaseConnection>,
    path: web::Path<String>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> impl Responder {
    let entity = path.into_inner();
    let allowed = ["brands", "users", "products"];

    if !allowed.contains(&entity.as_str()) {
        warn!("(save_file) Wrong selected entity");
        return HttpResponse::BadRequest().json(json!({
            "status": "Bad Request",
            "message": format!("'{}' is not a valid entity", entity)
        }));
    }

    if form.files.is_empty() {
        warn!("(save_file) No file provided");
        return HttpResponse::BadRequest().json(json!({
            "status": "Bad Request",
            "message": "No file provided"
        }));
    }

    for f in &form.files {
        let is_image = f
            .content_type
            .as_ref()
            .map(|m| m.type_() == IMAGE)
            .unwrap_or(false);

        if !is_image {
            warn!("(save_file) File different from image sent");
            return HttpResponse::BadRequest().json(json!({
                "status": "Bad Request",
                "message": "Only images are acceptable"
            }));
        }
    }

    let dir = format!("./uploads/{entity}");
    std::fs::create_dir_all(&dir).unwrap();

    let mut saved: Vec<serde_json::Value> = Vec::new();

    for f in form.files {
        let file_name = f.file_name.unwrap();
        let file_size = f.size;
        let file_mime = f
            .content_type
            .map(|m| m.to_string())
            .unwrap_or("application/octet-stream".to_string());
        let dest = format!("{dir}/{file_name}");
        info!("Saving to {dest}");
        std::fs::copy(f.file.path(), &dest).unwrap();

        let new_image = images::ActiveModel {
            id: Set(Uuid::new_v4()),
            file_name: Set(file_name),
            path: Set(dest.clone()),
            mime_type: Set(Some(file_mime)),
            size: Set(file_size.try_into().unwrap()),
            created_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        }
        .insert(db.get_ref())
        .await;

        match new_image {
            Ok(inserted) => {
                saved.push(json!({
                    "id": inserted.id,
                    "path": dest
                }));
            }
            Err(err) => {
                error!("Failed to insert image: {:?}", err);
                return HttpResponse::InternalServerError().json(json!({
                    "status": "Error",
                    "message": "Failed to save image to database"
                }));
            }
        }
    }

    HttpResponse::Ok().json(json!({
        "status": "Ok",
        "files": saved,
    }))
}
