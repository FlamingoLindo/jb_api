use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    dotenv::from_filename(".env.dev").ok();
    cli::run_cli(migration::Migrator).await;
}
