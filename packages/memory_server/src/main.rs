use std::{io::Read, net::TcpListener, sync::Arc};

use actix_web::cookie::Key;
use mail_send::SmtpClientBuilder;
use memory_server::{
    models::MongoDatabase,
    startup::{initialize_db, run, EmailClient, ServerConfig},
};
use mongodb::{options::ClientOptions, Client};
use std::fs::File;
use std::io::Write;
use tokio::sync::Mutex;
use tracing::info;



#[tokio::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let client_options = ClientOptions::parse(
        &std::env::var("MONGO_URI").unwrap_or("mongodb://localhost:27017".to_string()),
    )
    .await
    .unwrap();

    info!("Connecting to database {client_options:?}");
    let address = TcpListener::bind("0.0.0.0:8000").unwrap();

    info!("Server starting on {:?}", address.local_addr().unwrap());

    let db = Client::with_options(client_options).unwrap();
    let mongo_database = MongoDatabase::new(
        db,
        &std::env::var("DATABASE_NAME").unwrap_or("dev".to_string()),
    );
    initialize_db(&mongo_database).await.unwrap();

    info!("Connected to database");

    fn open_or_generate_secret_key() -> Key {
        let secret_key_path = "./data/private.pem";

        if let Ok(mut file) = File::open(&secret_key_path) {
            let mut secret_key = Vec::new();
            file.read_to_end(&mut secret_key).unwrap();
            info!("Loaded secret key from file");
            Key::derive_from(&secret_key)
        } else {
            info!("Generating secret key and storing to file");
            let secret_key: Key = Key::generate();
            std::fs::create_dir_all("./data").unwrap();
            let mut file = File::create(&secret_key_path).unwrap();
            file.write_all(secret_key.master()).unwrap();
            secret_key
        }
    }

    let secret_key = open_or_generate_secret_key();

    info!("Setting up email...");
    let email = SmtpClientBuilder::new("smtp.sendgrid.net", 587)
        .implicit_tls(false)
        .credentials((
            "apikey",
            std::env::var("SEND_GRID_API_KEY")
                .unwrap_or("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".to_string())
                .as_str(),
        ))
        .connect()
        .await
        .unwrap();
    let email_client: EmailClient = Arc::new(Mutex::new(email));

    run(mongo_database, address, ServerConfig::default(), secret_key, email_client)
        .await
        .unwrap()
        .await
        .unwrap();
}
