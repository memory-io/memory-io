use std::{net::TcpListener, time::Duration};

use crate::{
    models::{
        memorize::model::Memorize,
        user::model::{PasswordReset, User},
        MongoDatabase,
    },
    routes::factory,
};
use actix_cors::Cors;

use actix_identity::IdentityMiddleware;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::{cookie::Key, dev::Server, middleware::Logger, web, App, HttpServer};
use lettre::SmtpTransport;

use mongodb::{bson::doc, options::IndexOptions, IndexModel};

use tracing::info;
use tracing_actix_web::TracingLogger;

pub type EmailClient = SmtpTransport;

#[derive(Clone, Copy, Default)]
pub struct ServerConfig {
    pub test_mode: bool,
}

pub async fn run(
    db: MongoDatabase,
    listener: TcpListener,
    server: ServerConfig,
    secret_key: Key,
    email_client: EmailClient,
) -> Result<Server, anyhow::Error> {
    //change for prod

    info!("Starting server...");

    Ok(HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server))
            .app_data(web::Data::new(email_client.clone()))
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_origin("https://m3m0ry.io")
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

    client
        .db()
        .collection::<Memorize>("memorize")
        .create_index(
            IndexModel::builder()
                .keys(doc! {"last_answered":1})
                .options(
                    IndexOptions::builder()
                        .expire_after(Duration::from_secs(60 * 24 * 30 * 3))
                        .build(),
                )
                .build(),
            None,
        )
        .await?;

    Ok(())
}
