use tokio_postgres::{Error, NoTls};

pub async fn init_db() -> Result<tokio_postgres::Client, Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=bananatreedbadmin password=bananatreeohyeah dbname=bananatree",
        NoTls,
    )
    .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Create tables if they don't exist
    client
        .execute(
            "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            username VARCHAR(255) UNIQUE NOT NULL
        )",
            &[],
        )
        .await
        .expect("Failed to create users table");

    client
        .execute(
            "CREATE TABLE IF NOT EXISTS bananas (
            id SERIAL PRIMARY KEY,
            user_id INTEGER NOT NULL REFERENCES users(id),
            content TEXT NOT NULL
        )",
            &[],
        )
        .await
        .expect("Failed to create bananas table");

    Ok(client)
}
