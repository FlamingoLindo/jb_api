use crate::entities::{brands, images};
use actix_web::{HttpResponse, Responder, web};
use log::error;
use sea_orm::{DatabaseConnection, EntityTrait, ModelTrait, TransactionTrait};
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

    let image_id = brand.image_id;

    let result = db
        .get_ref()
        .transaction::<_, _, sea_orm::DbErr>(|txn| {
            Box::pin(async move {
                brand.delete(txn).await?;

                if let Some(img_id) = image_id {
                    images::Entity::delete_by_id(img_id).exec(txn).await?;
                }

                Ok(())
            })
        })
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "Ok",
            "message": "Brand deleted successfully"
        })),
        Err(err) => {
            error!("(delete_brand) Transaction failed: {:?}", err);
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "An error occurred when deleting the brand, please try again later"
            }))
        }
    }
}
