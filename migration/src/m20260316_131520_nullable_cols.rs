use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let columns = vec![
            decimal_len_null("price_kg", 10, 2),
            decimal_len_null("price_kg_no_cut", 10, 2),
            decimal_len_null("price_kg_cut", 10, 2),
            decimal_len_null("price_3mt", 10, 2),
            decimal_len_null("price_br", 10, 2),
            decimal_len_null("price_rod", 10, 2),
            double_null("weight_3mts"),
        ];

        for col in columns {
            manager
                .alter_table(
                    Table::alter()
                        .table(Alias::new("products"))
                        .modify_column(col)
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let columns = vec![
            decimal_len("price_kg", 10, 2),
            decimal_len("price_kg_no_cut", 10, 2),
            decimal_len("price_kg_cut", 10, 2),
            decimal_len("price_3mt", 10, 2),
            decimal_len("price_br", 10, 2),
            decimal_len("price_rod", 10, 2),
            double("weight_3mts"),
        ];

        for col in columns {
            manager
                .alter_table(
                    Table::alter()
                        .table(Alias::new("products"))
                        .modify_column(col)
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }
}
