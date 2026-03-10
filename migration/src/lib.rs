pub use sea_orm_migration::prelude::*;

mod m20260310_115101_create_users;

pub struct Migrator;

use dotenvy::dotenv;
use std::env;

pub fn get_database_url() -> String {
    dotenv().ok(); // загружаем .env
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260310_115101_create_users::Migration),
        ]
    }
}
