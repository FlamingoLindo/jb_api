# DTOs

This directory contains the data transfer objects used by the API. DTOs define the typed request bodies, query parameters, and response shapes that move between the route layer, handlers, and clients.

The DTO layer keeps HTTP payloads separate from database entities and handler logic. That makes the API contracts explicit and easier to validate.

## What It Does

DTOs in this project are used for:

- request payloads such as login, create, and update forms
- response payloads returned by handlers
- shared response shapes reused across multiple features
- validation and OpenAPI schema generation

## Folder Structure

The DTOs are grouped by feature:

- `users` contains login, password reset, user creation, and update DTOs
- `products` contains product create, update, export, and image-binding DTOs
- `brands` contains brand create, update, and image-binding DTOs
- `clients` contains client create, update, availability, and export DTOs
- `types`, `classes`, and `budget` contain their feature-specific payloads
- `shared` contains reusable response types

Each feature folder usually has its own `mod.rs` and one file per DTO or action.

## How It Works

Handlers accept these DTOs as typed inputs, validate them, and then map them into database queries or service calls.

Most DTO structs derive `Serialize`, `Deserialize`, `Validate`, and `ToSchema`, which lets them work with Actix, the `validator` crate, and OpenAPI generation.

Shared response DTOs are sometimes built from SeaORM query results using `FromQueryResult`.

## Example

```rust
use serde::{Deserialize, Serialize};
use validator::Validate;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Validate, ToSchema)]
pub struct LoginDTO {
 #[validate(email)]
 pub email: String,
 #[validate(length(min = 1, message = "Password cannot be empty"))]
 pub password: String,
}

#[derive(Serialize, Deserialize, FromQueryResult, ToSchema)]
pub struct SharedBrandResponse {
 #[sea_orm(alias = "brand_name")]
 pub name: String,
 #[sea_orm(alias = "brand_image")]
 pub image: Option<String>,
}
```

The first struct is used for incoming request data, while the second is used for returning a query-backed response shape.

## Usage Pattern

DTOs should stay focused on transport concerns only. They should describe what the API accepts or returns, while handlers and entities handle the actual business rules and persistence details.
