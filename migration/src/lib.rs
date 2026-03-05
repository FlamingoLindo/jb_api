pub use sea_orm_migration::prelude::*;

mod m20260304_124432_create_users_table;
mod m20260304_142033_add_token_to_user;
mod m20260305_132645_add_created_updated_at_to_users;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260304_124432_create_users_table::Migration),
            Box::new(m20260304_142033_add_token_to_user::Migration),
            Box::new(m20260305_132645_add_created_updated_at_to_users::Migration),
        ]
    }
}
