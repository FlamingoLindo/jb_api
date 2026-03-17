use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("products"))
                    .modify_column(ColumnDef::new(Alias::new("weight_3mts")).decimal_len(10, 2))
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("products"))
                    .modify_column(ColumnDef::new(Alias::new("weight_3mts")).float().not_null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
