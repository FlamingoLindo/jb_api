use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(
            r#"
            INSERT INTO roles (id, title, description)
            VALUES 
                (gen_random_uuid(), 'Master', 'Master Role'),
                (gen_random_uuid(), 'User', 'User Role'),
                (gen_random_uuid(), 'DPO', 'DPO Role');
            "#,
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared(r#"DELETE FROM roles WHERE title IN ('Master', 'User');"#)
            .await?;

        Ok(())
    }
}
