use std::net::TcpListener;

use crate::{
    models::{user::User, MongoDatabase},
    routes::factory,
};
use actix_cors::Cors;
use actix_identity::{Identity, IdentityMiddleware};
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::{
    cookie::Key,
    dev::{Server, Service},
    middleware::Logger,
    web, App, HttpServer,
};
use mongodb::{
    bson::doc,
    options::{ClientOptions, IndexOptions},
    Client, IndexModel,
};
use tracing::debug;

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
    Ok(HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(server.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials(),
            )
            .wrap(IdentityMiddleware::default())
            .wrap(Logger::default())
            // .wrap_fn(|req,srv| {
            //     debug!("Request Cookies: {:?}", req.request().cookies());
            //     let res = srv.call(req);
            //     async move {
            //         let mut res = res.await;
            //         res
            //     }
            // })
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

    Ok(())
}
