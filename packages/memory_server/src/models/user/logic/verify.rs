use std::str::FromStr;

use anyhow::bail;
use bson::de;
use chrono::Utc;
use lettre::{message::header::ContentType, Message, Transport};
use mongodb::{
    bson::{doc, oid::ObjectId},
    error::WriteFailure,
};
use tracing::{debug, error, instrument, trace};
use uuid::Uuid;
use validator::Validate;
use zxcvbn::zxcvbn;

use crate::{
    models::{
        user::model::{User, VerficationRequest},
        MongoDatabase,
    },
    startup::EmailClient,
};


pub async fn send_verification_email(
    db: &MongoDatabase,
    user_id: ObjectId,
    email_client: &EmailClient,
) -> Result<(), anyhow::Error> {
    debug!("Looking up user");
    let Some(user) = db
        .db()
        .collection::<User>("users")
        .find_one(doc! {"_id":user_id}, None)
        .await?
    else {
        bail!("User not found");
    };
    debug!("User found");
    let token = Uuid::new_v4().to_string();
    let verification = VerficationRequest {
        creation_date: bson::DateTime::now(),
        user_id: user.id,
        token: token.clone(),
    };
    debug!("Creating verification token");
    db.db()
        .collection("verifications")
        .insert_one(bson::to_bson(&verification).unwrap(), None)
        .await?;

    let email = Message::builder()
        .from("M3m0ry IO <admin@m3m0ry.io>".parse().unwrap())
        .reply_to("M3m0ry IO <admin@m3m0ry.io>".parse().unwrap())
        .to(format!("{} <{}>", user.username, user.email)
            .parse()
            .unwrap())
        .subject("Password Reset for M3m0ry.io")
        .header(ContentType::TEXT_HTML)
        .body(format!(
            "Click <a href='{}/auth/verify/{}'>here</a> to verify email",
            option_env!("DOMAIN").unwrap_or("http://localhost:5173"),
            token
        ))
        .unwrap();
    email_client.send(&email)?;
    debug!("Reset Token sent");

    Ok(())
}
