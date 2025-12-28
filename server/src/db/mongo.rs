use mongodb::{Client, Database};
use crate::config;

pub async fn connect() -> Database {
    let client = Client::with_uri_str(config::mongo_uri())
        .await
        .expect("Failed to initialize MongoDB client");

    client.database(&config::mongo_db())
}