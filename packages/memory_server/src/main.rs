use std::{io::Read, net::TcpListener};

use actix_web::cookie::Key;
use lettre::{transport::smtp::authentication::Credentials, SmtpTransport};
use memory_server::{
    models::MongoDatabase,
    startup::{initialize_db, run, ServerConfig},
};
use mongodb::{options::ClientOptions, Client};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use std::fs::File;
use std::io::Write;

use tracing::{info};



#[tokio::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    // LogTracer::init().expect("Unable to setup log tracer!");
  
    // let app_name = concat!(env!("CARGO_PKG_NAME"), "-", env!("CARGO_PKG_VERSION")).to_string();
    // let (non_blocking_writer, _guard) = tracing_appender::non_blocking(std::io::stdout());
    // let bunyan_formatting_layer = BunyanFormattingLayer::new(app_name, non_blocking_writer);
    // let subscriber = Registry::default()
    //     .with(EnvFilter::new("DEBUG"))
    //     .with(JsonStorageLayer)
    //     .with(bunyan_formatting_layer);
    // tracing::subscriber::set_global_default(subscriber).unwrap();
    let client_options = ClientOptions::parse(
        &std::env::var("MONGO_URI").unwrap_or("mongodb://localhost:27017".to_string()),
    )
    .await
    .unwrap();

    let domain = option_env!("DOMAIN").unwrap_or("localhost");  
    info!("Domain: {domain}");

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

        if let Ok(mut file) = File::open(secret_key_path) {
            let mut secret_key = Vec::new();
            file.read_to_end(&mut secret_key).unwrap();
            info!("Loaded secret key from file");
            Key::derive_from(&secret_key)
        } else {
            info!("Generating secret key and storing to file");
            let secret_key: Key = Key::generate();
            std::fs::create_dir_all("./data").unwrap();
            let mut file = File::create(secret_key_path).unwrap();
            file.write_all(secret_key.master()).unwrap();
            secret_key
        }
    }

    let secret_key = open_or_generate_secret_key();

    info!("Setting up email...");
    let creds = Credentials::new(
        "apikey".to_owned(),
        std::env::var("SEND_GRID_API_KEY").unwrap()
    );

    // Open a remote connection to gmail
    let email_client = SmtpTransport::relay("smtp.sendgrid.net")
        .unwrap()
        .credentials(creds)
        .build();
    run(
        mongo_database,
        address,
        ServerConfig::default(),
        secret_key,
        email_client,
    )
    .await
    .unwrap()
    .await
    .unwrap();
}
