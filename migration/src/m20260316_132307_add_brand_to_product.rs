use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("products"))
                    .add_column(uuid_null("brand_id"))
                    .add_foreign_key(
                        ForeignKey::create()
                            .name("fk_products_brand")
                            .from("products", Alias::new("brand_id"))
                            .to("brands", Alias::new("id"))
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade)
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
                    .table(Alias::new("products"))
                    .drop_foreign_key(Alias::new("fk_products_brand"))
                    .drop_column(Alias::new("brand_id"))
                    .to_owned(),
            )
            .await
    }
}
