use redis::{aio::MultiplexedConnection, Client, RedisResult};

pub async fn connect_redis() -> RedisResult<()> {
    let client = Client::open("redis://127.0.0.1/")?;
    let mut conn: MultiplexedConnection = client.get_multiplexed_async_connection().await?;

    let stream = "events:in";

    let entries: Vec<(String, Vec<(String,String)>)> = redis::cmd("XRANGE")
        .arg(stream)
        .arg("-")
        .arg("+")
        .query_async(&mut conn)
        .await?;

    for (id, fields) in entries {
        println!("id: {}", id);
        for (k, v) in fields {
            println!("  {}: {}", k, v);
        }
    } 

    Ok(())
}