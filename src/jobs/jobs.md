# Jobs

This directory contains the background jobs that run outside the normal request/response flow. These jobs keep temporary data clean and handle recurring maintenance tasks.

The jobs here are started once when the application boots, then run on a cron schedule in the background.

## What It Does

The job system currently handles two cleanup tasks:

- removing expired password reset tokens from the database
- deleting generated dump files from `exports/dumps`

## Files In This Directory

- `scheduler.rs` creates the cron scheduler and registers the jobs.
- `clear_reset_password_tokens.rs` removes password reset tokens older than five minutes.
- `clear_dumps.rs` deletes generated dump files from disk.
- `mod.rs` exposes the module tree.

## How It Runs

`src/main.rs` starts the scheduler with `tokio::spawn(jobs::scheduler::start(db_arc))`.

Inside the scheduler, each job is registered with a cron expression. Both current jobs are configured to run every day at midnight.

The scheduler then keeps running in the background while the API server handles requests.

## Example

```rust
// src/main.rs
tokio::spawn(jobs::scheduler::start(db_arc));

// src/jobs/scheduler.rs
Job::new_async("0 0 * * * *", move |_uuid, _lock| {
 let db = db_clone.clone();
 Box::pin(async move {
  super::clear_reset_password_tokens::clear_reset_password_tokens(&db).await;
 })
});
```

That example shows the scheduler being started and one of the cleanup jobs being registered.

## Usage Pattern

These jobs are not called by handlers directly. Instead, they are maintenance tasks that the application runs automatically after startup.
