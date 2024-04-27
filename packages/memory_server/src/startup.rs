use std::{net::TcpListener, sync::Arc, time::Duration};

use crate::{
    models::{
        user::{model::PasswordReset, model::User},
        MongoDatabase,
    },
    routes::factory,
};
use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_identity::{Identity, IdentityMiddleware};
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::{
    cookie::Key,
    dev::{Server, Service},
    middleware::Logger,
    web, App, HttpServer,
};
use mail_send::{SmtpClient, SmtpClientBuilder};
use mongodb::{
    bson::doc,
    options::{ClientOptions, IndexOptions},
    Client, IndexModel,
};
use tokio::{net::TcpStream, sync::Mutex};
use tokio_rustls::client::TlsStream;
use tracing::{debug, info};

pub type EmailClient = Arc<Mutex<SmtpClient<TlsStream<TcpStream>>>>;

#[derive(Clone, Copy)]
pub struct ServerConfig {
    pub test_mode: bool,
}
impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig { test_mode: false }
    }
}

pub async fn run(
    db: MongoDatabase,
    listener: TcpListener,
    server: ServerConfig,
) -> Result<Server, std::io::Error> {
    //change for prod
    let secret_key = Key::generate();

    info!("Setting up email...");
    let email = SmtpClientBuilder::new("smtp.gmail.com", 587)
        .implicit_tls(false)
        .credentials(("connerlreplogle@gmail.com", "ijij czfs cpbs saay"))
        .connect()
        .await
        .unwrap();
    let client: EmailClient = Arc::new(Mutex::new(email));

    info!("Starting server...");

    Ok(HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server.clone()))
            .app_data(web::Data::new(client.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials(),
            )
            // .wrap(Governor::new(&governor_conf))
            .wrap(IdentityMiddleware::default())
            .wrap(Logger::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .app_data(web::Data::new(db.clone()))
            .configure(factory)
    })
    .listen(listener)?
    .run())
}

pub async fn initialize_db(client: &MongoDatabase) -> Result<(), mongodb::error::Error> {
    client
        .db()
        .collection::<User>("users")
        .create_index(
            IndexModel::builder()
                .keys(doc! {"email":1})
                .options(IndexOptions::builder().unique(true).build())
                .build(),
            None,
        )
        .await?;

    client
        .db()
        .collection::<User>("users")
        .create_index(
            IndexModel::builder()
                .keys(doc! {"username":1})
                .options(IndexOptions::builder().unique(true).build())
                .build(),
            None,
        )
        .await?;
    client
        .db()
        .collection::<PasswordReset>("password_resets")
        .create_index(
            IndexModel::builder()
                .keys(doc! {"creation_date":1})
                .options(
                    IndexOptions::builder()
                        .expire_after(Duration::from_secs(60 * 10))
                        .build(),
                )
                .build(),
            None,
        )
        .await?;

    Ok(())
}
