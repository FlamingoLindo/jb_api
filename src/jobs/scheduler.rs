use log::info;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};

pub async fn start(db: Arc<DatabaseConnection>) {
    let scheduler = JobScheduler::new()
        .await
        .expect("Failed to create scheduler");

    let db_clone = db.clone();
    scheduler
        .add(
            Job::new_async("0 0 * * * *", move |_uuid, _lock| {
                // 0 0 * * * * = every day at midnight
                // 1/10 * * * * * = every 10 seconds
                // cron format: sec min hour day month weekday
                let db = db_clone.clone();
                Box::pin(async move {
                    info!("(cron) Running delete_expired_tokens");
                    super::clear_reset_password_tokens::clear_reset_password_tokens(&db).await;
                })
            })
            .expect("Failed to create job"),
        )
        .await
        .expect("Failed to add job");

    scheduler.start().await.expect("Failed to start scheduler");
    info!("(cron) Scheduler started");
}
