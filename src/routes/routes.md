# Routes

This directory contains the HTTP routing layer for the API. Its job is to group endpoints by domain, attach them to Actix scopes, and connect each route to the matching handler function.

The routing code is intentionally thin. It does not implement business logic itself. Instead, it:

- defines which URL paths exist
- groups related endpoints under a shared scope
- applies middleware such as authentication and role checks where needed
- keeps the API surface organized by feature

## How It Is Wired

Application startup calls the route configuration in [src/routes/config.rs](config.rs), which mounts the full API tree.

The flow is:

1. `src/main.rs` creates the Actix application.
2. `src/routes/config.rs` adds the top-level API scope at `/api/v1`.
3. Each feature folder under `src/routes/` registers its own routes through its `routes.rs` file.
4. Each route points to a handler in `src/handlers/`.

In practice, this means the routes folder is the central map of the API, while the handlers contain the actual request logic.

## Folder Structure

Each feature usually follows the same pattern:

- `mod.rs` exports the feature module.
- `routes.rs` defines the Actix `config` function and the endpoint paths.
- The corresponding handler code lives in `src/handlers/<feature>/`.

Examples in this directory include:

- `users` for authentication and user management endpoints
- `products` for product CRUD and related actions
- `brands` for brand CRUD and brand-image binding
- `clients` for client CRUD, availability, and export
- `budgets` for budget creation and lookup
- `images` for upload and delete operations
- `types` and `classes` for catalog metadata
- `database` for database dump operations
- `openapi` for API documentation and Scalar UI routes

## Route Conventions

Most feature routes follow the same style:

- collection endpoints are placed on the feature scope, such as `/users` or `/products`
- item endpoints use path parameters like `/{id}`
- actions that change state often use `POST`, `PATCH`, or `DELETE`
- protected endpoints wrap the route with authentication middleware and role guards when required

For example, the users route set includes public authentication endpoints like `/login` and `/forgot-password/{token}`, plus Master-only management endpoints such as `/register`, `/status/{id}`, and `/delete/{id}`.

## Example

```rust
// src/routes/config.rs
web::scope("/api/v1")
 .configure(routes::users::routes::config)

// src/routes/users/routes.rs
web::scope("/users")
 .route("/login", web::post().to(handler::login::login))
 .route(
  "/send-password-email",
  web::post().to(handler::send_forgot_password::send_forgot_password),
 )
 .route(
  "/forgot-password/{token}",
  web::patch().to(handler::forgot_password::forgot_password),
 )
 .service(
  web::resource("/register")
   .wrap(RoleGuard(&["Master"]))
   .wrap(auth())
   .route(web::post().to(handler::create::create_user)),
 )
```

This pattern repeats across the other feature folders, so each area of the API is isolated but mounted under the same versioned base path.
