# Middlewares

This directory contains the shared middleware used by the API to protect routes and attach authenticated user information to requests.

The middleware code here is small but important. It handles two main concerns:

- validating JWT bearer tokens
- enforcing role-based access on protected endpoints

## Files In This Directory

- `auth.rs` validates the bearer token, decodes the JWT claims, and stores the claims in the request context.
- `role.rs` checks whether the authenticated user has one of the roles required by the route.

## How It Works

The auth middleware is used first. It reads the token from the `Authorization` header, validates it with `JWT_SECRET`, and inserts the decoded claims into the request extensions.

The role guard runs after authentication. It reads the claims from the request and rejects the call if the user role is not in the allowed role list.

In practice, routes wrap both middlewares together when an endpoint must be authenticated and restricted to specific roles.

## Example

```rust
use crate::middlewares::role::RoleGuard;
use crate::middlewares::auth::validator;
use actix_web::web;
use actix_web_httpauth::middleware::HttpAuthentication;

let auth = || HttpAuthentication::bearer(validator);

web::resource("/register")
 .wrap(RoleGuard(&["Master"]))
 .wrap(auth())
 .route(web::post().to(handler::create::create_user));
```

In this example, the request must include a valid JWT, and the decoded role must be `Master` before the handler can run.

## Usage Pattern

This directory is not meant to be used directly from handlers. Instead, the route modules in `src/routes/` import these middlewares and apply them to the resources that need protection.
