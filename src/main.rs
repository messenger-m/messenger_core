use std::sync::Arc;
mod db;
mod entity;
mod grpc;
mod services;

use grpc::server::start_grpc_server;
use services::core::CoreService;
use db::postgres;
use dotenvy::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let db_conn = postgres::connect().await;

    let core_service = Arc::new(CoreService::new(db_conn));

    // запускаем gRPC сервер
    start_grpc_server(core_service).await?;

    Ok(())
}