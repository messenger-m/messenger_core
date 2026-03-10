use sea_orm::Database;
use std::env;

pub async fn connect() -> sea_orm::DatabaseConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    Database::connect(database_url).await.expect("Failed to connect to DB")
}