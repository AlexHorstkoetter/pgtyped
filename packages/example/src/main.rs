mod books;
mod query;
mod query_processor;

use tokio_postgres::{Error, NoTls};

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Hello, world!");

    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=postgres", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let rows = client.query(r#"SELECT * FROM "books""#, &[]).await?;
    let value: &str = rows[0].get(2);

    println!("value: {}", value);
    Ok(())
}
