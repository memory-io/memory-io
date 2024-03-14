use std::net::TcpListener;

use memory_server::{
    models::MongoDatabase,
    startup::{initialize_db, run, ServerConfig},
};
use mongodb::{options::ClientOptions, Client};
use tracing::debug;

#[tokio::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let client_options = ClientOptions::parse(
        &std::env::var("MONGO_URI").unwrap_or("mongodb://localhost:27017".to_string()),
    )
    .await
    .unwrap();

    debug!("Connecting to database {client_options:?}");
    let address = TcpListener::bind("127.0.0.1:8000").unwrap();

    let db = Client::with_options(client_options).unwrap();
    let mongo_database = MongoDatabase::new(
        db,
        &std::env::var("DATABASE_NAME").unwrap_or("dev".to_string()),
    );
    initialize_db(&mongo_database).await.unwrap();

    run(mongo_database, address, ServerConfig::default())
        .await
        .unwrap()
        .await
        .unwrap();
}
