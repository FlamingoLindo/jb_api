use log::{error, info};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entities::reset_password_tokens;

pub async fn clear_reset_password_tokens(db: &DatabaseConnection) {
    let tokens = reset_password_tokens::Entity::delete_many()
        .filter(
            reset_password_tokens::Column::CreatedAt
                .lt(chrono::Utc::now() - chrono::Duration::minutes(5)),
        )
        .exec(db)
        .await;

    match tokens {
        Ok(r) => info!(
            "(cron - clear_reset_password_tokens) Deleted {} expired reset password tokens",
            r.rows_affected
        ),
        Err(err) => error!(
            "(cron - clear_reset_password_tokens) Failed to delete expired tokens: {:?}",
            err
        ),
    }
}
