use std::{net::TcpListener};

use actix_web::cookie::Key;
use lettre::{transport::smtp::authentication::Credentials, SmtpTransport};

use memory_server::{
    models::{
        MongoDatabase,
    },
    startup::{initialize_db, run, ServerConfig},
};
use mongodb::{options::ClientOptions, Client};


use tracing::info;

pub struct TestApp {
    pub address: String,
    pub db: MongoDatabase,
}
impl Drop for TestApp {
    fn drop(&mut self) {
        // let db = self.db.clone();
        // tokio::task::spawn(async move {
        //     db.db().drop(None).await.unwrap();
        // });
    }
}
pub async fn spawn_app() -> TestApp {
    env_logger::try_init_from_env(env_logger::Env::new().default_filter_or("debug")).ok();

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let database_name = format!("test-{port}");

    let options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .unwrap();
    let db = Client::with_options(options).unwrap();
    let mongo_database = MongoDatabase::new(db, &database_name);
    initialize_db(&mongo_database).await.unwrap();
    info!("Setting up email...");
    let creds = Credentials::new(
        "apikey".to_owned(),
        std::env::var("SEND_GRID_API_KEY").unwrap_or(
            "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".to_string(),
        ),
    );

    // Open a remote connection to gmail
    let email_client = SmtpTransport::relay("smtp.sendgrid.net")
        .unwrap()
        .credentials(creds)
        .build();

    let server = run(
        mongo_database.clone(),
        listener,
        ServerConfig { test_mode: true },
        Key::generate(),
        email_client,
    )
    .await
    .expect("Failed to bind address");
    let _ = tokio::spawn(server);
    // We return the application address to the caller!
    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db: mongo_database,
    }
}

