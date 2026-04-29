# Database

This directory contains the database connection bootstrap for the application.

Its main job is to create the SeaORM connection pool, run migrations at startup, and provide the shared connection used by handlers, jobs, and routes.

## What It Does

The database layer currently:

- reads the `DATABASE_URL` environment variable
- configures the SeaORM connection pool
- runs migrations through the `migration` crate
- exposes the connected database handle to the rest of the app

## Files In This Directory

- `connect_to_db.rs` creates the database connection and runs migrations.
- `mod.rs` exposes the connection module.

## How It Works

`src/main.rs` calls `connect_to_db()` during startup. If the connection or migration step fails, the application does not continue booting.

After the connection is created, it is stored in Actix application state so handlers can access it through `web::Data<DatabaseConnection>`.

The database feature route at `/api/v1/database/dump` uses the same connection to look up authorized users and send dump emails.

## Example

```rust
// src/database/connect_to_db.rs
let db_conn = Database::connect(opt).await?;
Migrator::up(&db_conn, None).await?;

// src/main.rs
let db_conn = connect_to_db()
 .await
 .expect("Failed to connect to database");

App::new()
 .app_data(web::Data::new(db_conn.clone()));
```

That shows the database connection being created, migrated, and then shared with the application.

## Usage Pattern

The connection is reused everywhere instead of creating a new connection per request. That keeps the app efficient and ensures handlers, jobs, and maintenance tasks all work against the same database state.
