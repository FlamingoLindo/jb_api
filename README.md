# JB-API

JB-API is a Rust backend built with Actix Web and SeaORM. It exposes a versioned HTTP API for managing users, products, brands, clients, classes, types, budgets, images, and database dump operations.

## Features

- REST API with typed request and response payloads
- JWT-based authentication and role-based authorization
- SeaORM database access with automatic migrations on startup
- OpenAPI documentation and Scalar UI
- Email delivery for password reset, budget, and database dump flows
- Background cleanup jobs for reset tokens and generated dump files

## Project Layout

- `src/routes` defines the HTTP route tree
- `src/handlers` contains endpoint logic
- `src/dto` holds request and response types
- `src/database` boots the database connection
- `src/middlewares` contains authentication and role guards
- `src/mailer` sends templated emails
- `src/jobs` runs scheduled maintenance tasks
- `src/governors` defines rate-limited routes
- `migration` contains SeaORM migrations

## Setup

The application loads environment variables from `.env.<APP_ENV>`, or `.env.dev` if `APP_ENV` is not set.

Required environment variables include:

- `APP_ENV`
- `PORT`
- `DATABASE_URL`
- `JWT_SECRET`
- `SMTP_HOST`
- `SMTP_USERNAME`
- `SMTP_PASSWORD`
- `POSTGRES_USER`
- `POSTGRES_HOST`
- `POSTGRES_DB`
- `POSTGRES_PASSWORD`
- `MASTER_USERNAME`
- `MASTER_EMAIL`
- `MASTER_PASSWORD`

## Run

Typical development flow:

```bash
cargo run
```

The server starts by loading the environment, connecting to the database, running migrations, and starting the background scheduler.

## API Docs

- OpenAPI JSON: `/api/v1/openapi.json`
- Scalar UI: `/scalar`

## Notes

- Database dump requests are rate-limited and restricted to `Master` or `DPO` roles.
- Password reset tokens are cleaned up automatically by a scheduled job.
- Generated dump files are written under `exports/dumps`.
