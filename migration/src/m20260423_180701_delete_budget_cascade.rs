use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("budgets"))
                    .drop_foreign_key(Alias::new("fk_budgets_client"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("budgets"))
                    .add_foreign_key(
                        ForeignKey::create()
                            .name("fk_budgets_client")
                            .from("budgets", Alias::new("client_id"))
                            .to("clients", Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                            .to_owned()
                            .get_foreign_key(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("budgets"))
                    .drop_foreign_key(Alias::new("fk_budgets_client"))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("budgets"))
                    .add_foreign_key(
                        ForeignKey::create()
                            .name("fk_budgets_client")
                            .from("budgets", Alias::new("client_id"))
                            .to("clients", Alias::new("id"))
                            .on_delete(ForeignKeyAction::Restrict)
                            .on_update(ForeignKeyAction::Cascade)
                            .to_owned()
                            .get_foreign_key(),
                    )
                    .to_owned(),
            )
            .await
    }
}
