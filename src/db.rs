use tokio_postgres::{Client, NoTls};

pub async fn init_db() -> Result<Client, tokio_postgres::Error> {
    let connection_str = "host=localhost user=myuser password=mypassword dbname=sample_db";
    let (client, connection) = tokio_postgres::connect(connection_str, NoTls)
        .await
        .expect("Error connecting to the database");

    // Bağlantıyı arka planda çalıştır
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}