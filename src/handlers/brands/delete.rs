use crate::entities::{brands, brands_images, images};
use actix_web::{HttpResponse, Responder, web};
use log::{error, warn};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter};
use serde_json::json;
use uuid::Uuid;

pub async fn delete_brand(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let id = id.into_inner();
    let existing_brand = brands::Entity::find_by_id(id).one(db.get_ref()).await;
    let brand = match existing_brand {
        Ok(Some(brand)) => brand,
        Ok(None) => {
            warn!("(delete_brand) Brand not found");
            return HttpResponse::NotFound().json(json!({
                "status": "Not Found",
                "message": "Brand not found"
            }));
        }
        Err(err) => {
            error!("(delete_brand) Could not find brand: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when finding the provided brand, please try again later"
            }));
        }
    };

    let bind_imgs = brands_images::Entity::find()
        .filter(brands_images::Column::BrandId.eq(id))
        .all(db.get_ref())
        .await;

    match bind_imgs {
        Ok(binds) => {
            for bind in binds {
                let image = images::Entity::find()
                    .filter(images::Column::Id.eq(bind.image_id))
                    .one(db.get_ref())
                    .await;

                match image {
                    Ok(Some(img)) => {
                        if let Err(err) = std::fs::remove_file(&img.path) {
                            error!("(delete_brand) Could not delete file from disk: {:?}", err);
                            return HttpResponse::InternalServerError().json(json!({
                                "status": "Internal Server Error",
                                "message": "Something went wrong when deleting image"
                            }));
                        }
                        let _ = img.delete(db.get_ref()).await;
                    }
                    Ok(None) => {
                        warn!("(delete_brand) Could not find image");
                        return HttpResponse::NotFound().json(json!({
                            "status": "Not Found",
                            "message": "Image not found"
                        }));
                    }
                    Err(err) => {
                        error!("(delete_brand) Could not find image: {:?}", err);
                        return HttpResponse::InternalServerError().json(json!({
                            "status": "Internal Server Error",
                            "message": "There has been an error when deleting brand image, please try again later"
                        }));
                    }
                }

                let _ = bind.delete(db.get_ref()).await;
            }
        }
        Err(err) => {
            error!("(delete_brand) Could not find brand's images: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "An error occurred when deleting the brand's images, please try again later"
            }));
        }
    }

    match brand.delete(db.get_ref()).await {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "Ok",
            "message": "Brand deleted successfully"
        })),
        Err(err) => {
            error!("(delete_brand) Could not delete brand: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "An error occurred when deleting the brand, please try again later"
            }))
        }
    }
}
