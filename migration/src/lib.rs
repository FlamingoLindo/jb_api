pub use sea_orm_migration::prelude::*;

mod m20260304_124432_create_users_table;
mod m20260304_142033_add_token_to_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260304_124432_create_users_table::Migration),
            Box::new(m20260304_142033_add_token_to_user::Migration),
        ]
    }
}
