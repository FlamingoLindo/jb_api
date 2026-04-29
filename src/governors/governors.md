# Governors

This directory contains the request governor configuration used to rate-limit specific endpoints.

The current setup is small and focused: it only defines the governor used for database dump creation, which prevents that endpoint from being called too often.

## What It Does

The governor layer currently:

- builds the rate-limit policy for the database dump route
- stores the configured governors in a shared registry struct
- passes the relevant governor into the route layer during startup

## Files In This Directory

- `dump.rs` defines the dump-specific governor settings.
- `registry.rs` stores the governors that the application uses at runtime.
- `mod.rs` exposes the module tree.

## How It Works

`Governors::init()` builds the available governors once when the application starts.

That registry is then passed into `src/routes/config.rs`, which forwards the dump governor into the database route configuration.

The database dump endpoint wraps the route with `actix_governor::Governor`, so requests are limited before the handler runs.

## Example

```rust
// src/governors/dump.rs
GovernorConfigBuilder::default()
 .seconds_per_request(86400)
 .burst_size(1)
 .key_extractor(GlobalKeyExtractor)
 .finish()
 .unwrap();

// src/routes/database/routes.rs
web::resource("/dump")
 .wrap(RoleGuard(&["Master", "DPO"]))
 .wrap(auth())
 .wrap(Governor::new(dump_governor))
 .route(web::post().to(handler::dump::create_db_dump));
```

That example shows the governor being defined once and then applied to the database dump route alongside authentication and role checks.

## Usage Pattern

Governors are not used broadly across the API. They are applied only to routes that need extra request throttling, which keeps the policy explicit and easy to audit.
