use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table("products")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alias::new("id"))
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(string("code"))
                    .col(string("description"))
                    .col(uuid_null("type_id"))
                    .col(uuid_null("class_id"))
                    .col(boolean("blocked"))
                    .col(decimal_len("price_kg", 10, 2))
                    .col(decimal_len("price_kg_no_cut", 10, 2))
                    .col(decimal_len("price_kg_cut", 10, 2))
                    .col(decimal_len("price_3mt", 10, 2))
                    .col(decimal_len("price_br", 10, 2))
                    .col(decimal_len("price_rod", 10, 2))
                    .col(double("weight_3mts"))
                    .col(
                        timestamp("created_at")
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp("updated_at")
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_products_type")
                            .from("products", Alias::new("type_id"))
                            .to("types", Alias::new("id"))
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_products_class")
                            .from("products", Alias::new("class_id"))
                            .to("classes", Alias::new("id"))
                            .on_delete(ForeignKeyAction::SetNull)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table("products").to_owned())
            .await
    }
}
