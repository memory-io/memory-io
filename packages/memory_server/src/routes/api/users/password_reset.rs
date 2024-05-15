use std::time::Duration;



use actix_web::{
    get, post,
    web::{Data, Json, Path}, HttpResponse, Responder,
};

use serde::Deserialize;
use tokio::time::sleep;
use tracing::{instrument, warn};

use crate::{
    models::{
        user::{self},
        MongoDatabase,
    },
    startup::EmailClient,
};
use user::logic::password_reset as pr;

#[derive(Deserialize, Debug)]
struct ResetRequest {
    email: String,
}
#[post("password_reset")]
#[instrument(skip(db, client),fields(email = %email.email))]
pub async fn password_reset(
    email: Json<ResetRequest>,
    db: Data<MongoDatabase>,
    client: Data<EmailClient>,
) -> impl Responder {
    match pr::password_reset(&db, &client, &email.email).await {
        Ok(_) => HttpResponse::Ok().await.unwrap(),
        Err(err) => {
            warn!("Failed to send reset {err}");
            sleep(Duration::from_millis(500)).await;
            HttpResponse::Ok().await.unwrap()
        }
    }
}

#[get("password_reset/{token}")]
#[instrument(skip(db))]
pub async fn validate_password_reset(
    token: Path<String>,
    db: Data<MongoDatabase>,
) -> impl Responder {
    match pr::validate_reset(&db, &token).await {
        Ok(_) => HttpResponse::Ok().await.unwrap(),
        Err(err) => {
            warn!("Failed to validate reset {err}");
            HttpResponse::InternalServerError().await.unwrap()
        }
    }
}
#[derive(Deserialize, Debug)]
struct PasswordReset {
    token: String,
    password: String,
}

#[post("change_password")]
#[instrument(skip(db),fields(token = %reset.token))]
pub async fn change_password(
    reset: Json<PasswordReset>,
    db: Data<MongoDatabase>,
) -> impl Responder {
    match pr::change_password(&db, &reset.token, &reset.password).await {
        Ok(()) => HttpResponse::Ok().await.unwrap(),
        Err(err) => {
            warn!("Failed to change password {err}");
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}
