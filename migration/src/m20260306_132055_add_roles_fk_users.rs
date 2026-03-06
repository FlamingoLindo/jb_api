use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("users"))
                    .add_column(ColumnDef::new(Alias::new("role_id")).uuid().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("users"))
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_users_role_id")
                            .from_tbl(Alias::new("users"))
                            .from_col(Alias::new("role_id"))
                            .to_tbl(Alias::new("roles"))
                            .to_col(Alias::new("id"))
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("users"))
                    .drop_foreign_key(Alias::new("fk_users_role_id"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("users"))
                    .drop_column(Alias::new("role_id"))
                    .to_owned(),
            )
            .await
    }
}
