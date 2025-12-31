mod db;
use std::error::Error;
use db::redis_session::connect_redis;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    connect_redis().await?;

    Ok(())
}
