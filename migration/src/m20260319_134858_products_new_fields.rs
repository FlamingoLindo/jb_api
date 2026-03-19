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
                    .add_column(decimal_len("price_p_mt", 10, 2).null())
                    .add_column(decimal_len("cut_percentage", 5, 2).null())
                    .add_column(decimal_len("weight_p_mm", 12, 6).null())
                    .add_column(decimal_len("weight", 10, 3).null())
                    .add_column(decimal_len("weight_esp", 10, 3).null())
                    .add_column(decimal_len("weight_p_br", 10, 3).null())
                    .add_column(decimal_len("br_price", 10, 2).null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("products"))
                    .drop_column(Alias::new("price_p_mt"))
                    .drop_column(Alias::new("cut_percentage"))
                    .drop_column(Alias::new("weight_p_mm"))
                    .drop_column(Alias::new("weight"))
                    .drop_column(Alias::new("weight_esp"))
                    .drop_column(Alias::new("weight_p_br"))
                    .drop_column(Alias::new("br_price"))
                    .to_owned(),
            )
            .await
    }
}
