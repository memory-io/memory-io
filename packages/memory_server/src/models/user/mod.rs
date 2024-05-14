pub mod model;
use std::str::FromStr;

use anyhow::bail;
use chrono::Utc;
use mail_send::{mail_auth::hickory_resolver::proto::rr::rdata::A, mail_builder::MessageBuilder};
use mongodb::{
    bson::{doc, oid::ObjectId},
    error::WriteFailure,
};
use tracing::error;
use uuid::Uuid;
use validator::Validate;
use zxcvbn::zxcvbn;

use crate::startup::EmailClient;

use self::model::{PasswordReset, User, UserSendable, UserSignup};

use super::MongoDatabase;

pub async fn check_username(db: &MongoDatabase, username: &str) -> Result<(), anyhow::Error> {
    if username.len() < 3 || username.len() > 20 || username.contains(' ') {
        bail!("Username must be between 3 and 20 characters");
    }
    let user = db
        .db()
        .collection::<User>("users")
        .find_one(doc! {"username":username}, None)
        .await?;
    if user.is_some() {
        bail!("Be More Original");
    }
    Ok(())
}

pub async fn get_user(
    db: &MongoDatabase,
    user_id: String,
) -> Result<Option<UserSendable>, mongodb::error::Error> {
    let user_id = ObjectId::from_str(&user_id).unwrap();
    db.db()
        .collection("users")
        .find_one(doc! {"_id":user_id}, None)
        .await
}

pub async fn create_user(
    db: &MongoDatabase,
    mut user: UserSignup,
) -> Result<mongodb::results::InsertOneResult, anyhow::Error> {
    user.validate()?;
    if zxcvbn(&user.password, &[])?.score() < 3 {
        bail!("Password is too weak");
    }
    user.password = match bcrypt::hash(user.password, 12) {
        Ok(a) => a,
        Err(err) => {
            error!("Failed to hash password Err: {err}");
            bail!("Failed to hash password");
        }
    };
    check_username(db, &user.username).await?;
    match db.db().collection("users").insert_one(user, None).await {
        Ok(a) => Ok(a),
        Err(err) => {
            match &*err.kind {
                mongodb::error::ErrorKind::Write(err) => {
                    if let WriteFailure::WriteError(write_err) = err {
                        if write_err.code == 11000 {
                            bail!("User already exists");
                        }
                    }
                }
                _ => {}
            }
            error!("Failed to create user Err: {err}");
            bail!("Failed to create user");
        }
    }
}

pub async fn authenticate_user(
    db: &MongoDatabase,
    email: &str,
    password: &str,
) -> Result<Option<ObjectId>, anyhow::Error> {
    let a = db
        .db()
        .collection::<User>("users")
        .find_one(doc! {"email":email}, None)
        .await?;
    if let Some(user) = a {
        if bcrypt::verify(password, &user.password).unwrap() {
            return Ok(Some(user.id));
        }
    }
    Ok(None)
}

pub(crate) async fn validate_reset(db: &MongoDatabase, token: &str) -> Result<(), anyhow::Error> {
    let Some(reset) = db
        .db()
        .collection::<PasswordReset>("password_resets")
        .find_one(doc! {"token":token}, None)
        .await?
    else {
        bail!("Invalid Token");
    };
    if reset.creation_date.to_chrono() < Utc::now() - chrono::Duration::seconds(10 * 60) {
        bail!("Expired Token");
    }
    Ok(())
}

pub(crate) async fn change_password(
    db: &MongoDatabase,
    token: &str,
    new_password: &str,
) -> Result<(), anyhow::Error> {
    if zxcvbn(&new_password, &[])?.score() < 3 {
        bail!("Password is too weak");
    }
    let Some(reset) = db
        .db()
        .collection::<PasswordReset>("password_resets")
        .find_one(doc! {"token":token}, None)
        .await?
    else {
        anyhow::bail!("Invalid Token");
    };
    if reset.creation_date.to_chrono() < Utc::now() - chrono::Duration::seconds(10 * 60) {
        anyhow::bail!("Token Expired");
    }

    db.db()
        .collection::<User>("users")
        .update_one(
            doc! {"_id":reset.user_id,},
            doc! {"$set":{"password":bcrypt::hash(new_password,12).unwrap()}},
            None,
        )
        .await?;

    db.db()
        .collection::<PasswordReset>("password_resets")
        .delete_one(doc! {"token":token}, None)
        .await?;

    Ok(())
}

pub(crate) async fn password_reset(
    db: &actix_web::web::Data<MongoDatabase>,
    client: &EmailClient,
    email: &str,
) -> Result<(), anyhow::Error> {
    let user = db
        .db()
        .collection::<User>("users")
        .find_one(doc! {"email":email}, None)
        .await?;
    if let Some(user) = user {
        let token = Uuid::new_v4().to_string();
        let reset = PasswordReset {
            creation_date: bson::DateTime::now(),
            user_id: user.id,
            token: token.clone(),
        };
        db.db()
            .collection("password_resets")
            .insert_one(bson::to_bson(&reset).unwrap(), None)
            .await?;
        let message = MessageBuilder::new()
            .from(("Memory IO", "admin@m3m0ry.io"))
            .to(vec![("Password Reset", user.email.as_str())])
            .subject("Password Reset")
            .html_body(format!(
                "Click <a href='{}/auth/password_reset/{}'>here</a> to reset your password",
                option_env!("DOMAIN").unwrap_or("http://localhost:5173"),
                token
            ))
            .text_body("Hello world!");

        client.lock().await.send(message).await?;

        return Ok(());
    }
    bail!("User not found");
}
