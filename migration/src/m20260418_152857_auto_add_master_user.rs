use argon2::password_hash::{SaltString, rand_core::OsRng};
use argon2::{Argon2, PasswordHasher};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let username =
            std::env::var("MASTER_USERNAME").expect("MASTER_USERNAME must be set in env");
        let email = std::env::var("MASTER_EMAIL").expect("MASTER_EMAIL must be set in env");
        let password =
            std::env::var("MASTER_PASSWORD").expect("MASTER_PASSWORD must be set in env");

        let salt = SaltString::generate(&mut OsRng);
        let hashed = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .expect("Failed to hash password")
            .to_string();

        let db = manager.get_connection();

        db.execute_unprepared(&format!(
            r#"
            INSERT INTO users (id, username, email, blocked, password, role_id)
            VALUES (
                gen_random_uuid(),
                '{username}',
                '{email}',
                'false',
                '{hashed}',
                (SELECT id FROM roles WHERE title = 'Master' LIMIT 1)
            );
            "#
        ))
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let username =
            std::env::var("MASTER_USERNAME").expect("MASTER_USERNAME must be set in env");
        let db = manager.get_connection();
        db.execute_unprepared(&format!(
            r#"DELETE FROM users WHERE username = '{username}';"#
        ))
        .await?;
        Ok(())
    }
}
