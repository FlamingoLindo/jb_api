use sea_orm_migration::{
    prelude::{extension::postgres::Type, *},
    schema::*,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum("client_type")
                    .values(["pf", "pj"])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table("clients")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Alias::new("id"))
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(string("name"))
                    .col(string("phone"))
                    .col(
                        ColumnDef::new(Alias::new("client_type"))
                            .custom(Alias::new("client_type"))
                            .not_null(),
                    )
                    .col(string_null("cpf"))
                    .col(string_null("cnpj"))
                    .col(string_null("observation"))
                    .col(
                        ColumnDef::new(Alias::new("blocked"))
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(string("zipcode"))
                    .col(string("state"))
                    .col(string("city"))
                    .col(string("street"))
                    .col(string_null("complement"))
                    .col(string_null("neighborhood"))
                    .col(string("number"))
                    .col(
                        ColumnDef::new(Alias::new("created_at"))
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Alias::new("updated_at"))
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Alias::new("clients")).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(Alias::new("client_type")).to_owned())
            .await
    }
}
