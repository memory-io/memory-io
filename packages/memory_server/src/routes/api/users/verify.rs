use std::{str::FromStr, time::Duration};

use actix_identity::Identity;

use actix_web::{
    get, post,
    web::{self, Data, Json, Path},
    HttpMessage, HttpRequest, HttpResponse, Responder,
};
use bson::oid::ObjectId;

use crate::{
    models::{
        user::{self, model::UserSignup},
        MongoDatabase,
    },
    startup::EmailClient,
};
use serde::Deserialize;
use tokio::time::sleep;
use tracing::{debug, info, instrument, trace, warn};
use user::logic::verify;

#[instrument(skip(id,db,client),fields(user_id = id.id().unwrap()))]
#[post("/send_verification")]
pub async fn user_verfication(
    id: Identity,
    db: Data<MongoDatabase>,
    client: Data<EmailClient>,
) -> impl Responder {
    debug!("Sending user verification email");
    let id = ObjectId::from_str(id.id().unwrap().as_str()).unwrap();
    match verify::send_verification_email(&db, id, &client).await {
        Ok(_) => HttpResponse::Ok().await.unwrap(),
        Err(err) => {
            warn!("Failed to send verification email {err}");
            sleep(Duration::from_millis(500)).await;
            HttpResponse::Ok().await.unwrap()
        }
    }
}

#[instrument(skip(id,db,client),fields(user_id = id.id().unwrap()))]
#[post("/verify")]
pub async fn verify_code(
    id: Identity,
    db: Data<MongoDatabase>,
    client: Data<EmailClient>,
) -> impl Responder {
    debug!("Sending user verification email");
    let id = ObjectId::from_str(id.id().unwrap().as_str()).unwrap();
    match verify::send_verification_email(&db, id, &client).await {
        Ok(_) => HttpResponse::Ok().await.unwrap(),
        Err(err) => {
            warn!("Failed to send verification email {err}");
            sleep(Duration::from_millis(500)).await;
            HttpResponse::Ok().await.unwrap()
        }
    }
}
