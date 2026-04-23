use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("brands_images")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alias::new("id"))
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(uuid("brand_id").not_null())
                    .col(uuid("image_id").not_null())
                    .col(
                        timestamp("created_at")
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_brands_images_brand")
                            .from("brands_images", Alias::new("brand_id"))
                            .to("brands", Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_brands_images_image")
                            .from("brands_images", Alias::new("image_id"))
                            .to("images", Alias::new("id"))
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("brands"))
                    .drop_column(Alias::new("image_id"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("brands_images").to_owned())
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("brands"))
                    .add_column(uuid("image_id").null())
                    .to_owned(),
            )
            .await
    }
}
