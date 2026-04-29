# Handlers

This directory contains the request handlers for the API. Handlers sit behind the route definitions and implement the actual work for each endpoint.

The handlers are where requests are validated, database records are queried or updated, files are generated, and other services such as the mailer are called.

## What It Does

Handlers in this project typically:

- accept typed request bodies or path parameters
- validate incoming data
- query or modify the database through SeaORM
- return JSON responses with clear status codes
- call supporting services like the mailer or file system helpers when needed

## Folder Structure

The folder is organized by feature:

- `users` handles login, password recovery, user creation, updates, and exports
- `products` handles product CRUD, price adjustments, exports, and image bindings
- `brands` handles brand CRUD and brand-image bindings
- `clients` handles client CRUD, availability, and export
- `types` and `classes` handle catalog metadata
- `budgets` handles budget creation and lookup
- `images` handles upload and delete operations
- `database` handles database dump generation

Each feature directory usually contains one file per action, and its `mod.rs` re-exports those files for the route layer to call.

## How It Works

Routes in `src/routes/` point directly to these handlers. The handler function receives the request data, performs the business checks, and returns an HTTP response.

Many handlers also include `utoipa` annotations so the same code can be used to generate the OpenAPI specification.

## Example

```rust
#[utoipa::path(
 post,
 path = "/api/v1/users/send-password-email",
 tag = "Users",
 request_body = SendForgotPasswordDTO,
)]
pub async fn send_forgot_password(
 db: web::Data<DatabaseConnection>,
 data: web::Json<SendForgotPasswordDTO>,
) -> impl Responder {
 if let Err(errors) = data.validate() {
  return HttpResponse::BadRequest().json(errors);
 }

 let existing_user = users::Entity::find()
  .filter(users::Column::Email.eq(data.email.to_lowercase()))
  .one(db.get_ref())
  .await;

 // business logic, token creation, and email sending happen here
 HttpResponse::Ok().json(json!({"status": "Ok", "message": "E-mail sent!"}))
}
```

This is the general handler pattern in the project: validate input, apply the business rule, and return a response that the route can expose to the client.

## Usage Pattern

Handlers should stay focused on one endpoint or one action. Shared concerns such as authentication, role checks, and cron work belong in the middleware or jobs layers instead.
