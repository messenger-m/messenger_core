use tokio_postgres::{Client, NoTls};

pub async fn connect() -> Client {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost port=5432 user=postgres password=postgres dbname=messenger",
        NoTls,
    )
    .await
    .expect("Failed to connect to DB");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Postgres connection error: {}", e);
        }
    });

    client
}
