use std::{str::FromStr, time::Duration};

use actix_identity::Identity;

use actix_web::{
    post,
    web::{Data}, HttpResponse, Responder,
};
use bson::oid::ObjectId;

use crate::{
    models::{
        user::{self},
        MongoDatabase,
    },
    startup::EmailClient,
};

use tokio::time::sleep;
use tracing::{debug, instrument, warn};
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
