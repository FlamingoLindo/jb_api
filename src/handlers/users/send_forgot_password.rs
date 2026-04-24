use crate::{
    dto::users::send_forgot_password::SendForgotPasswordDTO,
    entities::{reset_password_tokens, users},
    mailer::mailer::Mailer,
};
use actix_web::{HttpResponse, Responder, web};
use log::{error, warn};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

pub async fn send_forgot_password(
    db: web::Data<DatabaseConnection>,
    data: web::Json<SendForgotPasswordDTO>,
) -> impl Responder {
    if let Err(errors) = data.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    let existing_user = match users::Entity::find()
        .filter(users::Column::Email.eq(data.email.to_lowercase()))
        .one(db.get_ref())
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            warn!("(send_forgot_password) User not found");
            return HttpResponse::Ok().json(json!({
                "status": "Ok",
                "message": "Ok"
            }));
        }
        Err(err) => {
            error!("(send_forgot_password) Could not find user: {:?}", err);
            return HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when finding user, please try again"
            }));
        }
    };

    let email = match existing_user.email.as_deref() {
        Some(e) => e.to_owned(),
        None => {
            warn!("(send_forgot_password) User has no email");
            return HttpResponse::BadRequest().json(json!({
                "status": "Bad Request",
                "message": "User has no email address"
            }));
        }
    };

    let gen_token = Uuid::new_v4();

    let reset_token = reset_password_tokens::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(existing_user.id),
        token: Set(gen_token),
        ..Default::default()
    }
    .insert(db.get_ref())
    .await;

    if let Err(err) =
        Mailer::send_forgot_password(&email, &existing_user.username, &gen_token.to_string())
    {
        error!(
            "(send_forgot_password) Could not send password email: {:?}",
            err
        );
        return HttpResponse::InternalServerError().json(json!({
            "status": "Internal Server Error",
            "message": "Could not send password email"
        }));
    }

    match reset_token {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "Ok",
            "message": "E-mail sent!"
        })),
        Err(err) => {
            error!(
                "(send_forgot_password) Could not send email to user: {:?}",
                err
            );
            HttpResponse::InternalServerError().json(json!({
                "status": "Internal Server Error",
                "message": "There has been an error when sending email to user, please try again"
            }))
        }
    }
}
