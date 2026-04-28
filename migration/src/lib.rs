pub use sea_orm_migration::prelude::*;

mod m20260304_124432_create_users_table;
mod m20260304_142033_add_token_to_user;
mod m20260305_132645_add_created_updated_at_to_users;
mod m20260306_130733_add_roles;
mod m20260306_132055_add_roles_fk_users;
mod m20260307_135800_create_class;
mod m20260307_141844_create_types;
mod m20260307_142040_create_brands;
mod m20260307_142334_create_images;
mod m20260307_143229_create_products;
mod m20260307_144941_create_products_images_junction;
mod m20260316_131520_nullable_cols;
mod m20260316_132307_add_brand_to_product;
mod m20260316_184710_products_weight_to_decimal;
mod m20260319_134858_products_new_fields;
mod m20260418_151752_auto_add_roles;
mod m20260418_152857_auto_add_master_user;
mod m20260420_164641_clients;
mod m20260422_125320_budget_table;
mod m20260423_003553_brands_images;
mod m20260423_174904_add_clients_email;
mod m20260423_180701_delete_budget_cascade;
mod m20260424_125643_add_users_email;
mod m20260424_194432_password_reset_tokens;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260304_124432_create_users_table::Migration),
            Box::new(m20260304_142033_add_token_to_user::Migration),
            Box::new(m20260305_132645_add_created_updated_at_to_users::Migration),
            Box::new(m20260306_130733_add_roles::Migration),
            Box::new(m20260306_132055_add_roles_fk_users::Migration),
            Box::new(m20260307_135800_create_class::Migration),
            Box::new(m20260307_141844_create_types::Migration),
            Box::new(m20260307_142334_create_images::Migration),
            Box::new(m20260307_142040_create_brands::Migration),
            Box::new(m20260307_143229_create_products::Migration),
            Box::new(m20260307_144941_create_products_images_junction::Migration),
            Box::new(m20260316_131520_nullable_cols::Migration),
            Box::new(m20260316_132307_add_brand_to_product::Migration),
            Box::new(m20260316_184710_products_weight_to_decimal::Migration),
            Box::new(m20260319_134858_products_new_fields::Migration),
            Box::new(m20260418_151752_auto_add_roles::Migration),
            Box::new(m20260420_164641_clients::Migration),
            Box::new(m20260422_125320_budget_table::Migration),
            Box::new(m20260423_003553_brands_images::Migration),
            Box::new(m20260423_174904_add_clients_email::Migration),
            Box::new(m20260423_180701_delete_budget_cascade::Migration),
            Box::new(m20260424_125643_add_users_email::Migration),
            Box::new(m20260424_194432_password_reset_tokens::Migration),
            Box::new(m20260418_152857_auto_add_master_user::Migration),
        ]
    }
}
