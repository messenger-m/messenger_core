use tokio_postgres::{NoTls, Error};


pub async fn connect_postgres() -> Result<(), Error> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost port=5432 user=postgres password=postgres dbname=mydb", NoTls).await?;
}