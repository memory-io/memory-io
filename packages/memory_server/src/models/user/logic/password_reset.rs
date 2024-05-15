

use anyhow::bail;

use chrono::Utc;
use lettre::{message::header::ContentType, Message, Transport};
use mongodb::{
    bson::{doc},
};
use tracing::{debug, trace};
use uuid::Uuid;

use zxcvbn::zxcvbn;

use crate::{
    models::{
        user::model::{PasswordReset, User},
        MongoDatabase,
    },
    startup::EmailClient,
};

pub(crate) async fn validate_reset(db: &MongoDatabase, token: &str) -> Result<(), anyhow::Error> {
    debug!("Validating password reset");
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
    debug!("Changing password {token:?}");
    if zxcvbn(new_password, &[])?.score() < 3 {
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
    debug!("Token Found");

    db.db()
        .collection::<User>("users")
        .update_one(
            doc! {"_id":reset.user_id,},
            doc! {"$set":{"password":bcrypt::hash(new_password,12).unwrap()}},
            None,
        )
        .await?;
    debug!("Set Password");

    db.db()
        .collection::<PasswordReset>("password_resets")
        .delete_one(doc! {"token":token}, None)
        .await?;
    debug!("Deleted password reset");

    Ok(())
}
pub(crate) async fn password_reset(
    db: &actix_web::web::Data<MongoDatabase>,
    client: &EmailClient,
    email: &str,
) -> Result<(), anyhow::Error> {
    debug!("Resetting password");
    trace!("Checking for email");
    let user = db
        .db()
        .collection::<User>("users")
        .find_one(doc! {"email":email}, None)
        .await?;

    if let Some(user) = user {
        debug!("Email found");

        let token = Uuid::new_v4().to_string();
        let reset = PasswordReset {
            creation_date: bson::DateTime::now(),
            user_id: user.id,
            token: token.clone(),
        };
        debug!("Create Reset Token");

        db.db()
            .collection("password_resets")
            .insert_one(bson::to_bson(&reset).unwrap(), None)
            .await?;
        debug!("Sending Reset Token");
        let email = Message::builder()
            .from("M3m0ry IO <admin@m3m0ry.io>".parse().unwrap())
            .reply_to("M3m0ry IO <admin@m3m0ry.io>".parse().unwrap())
            .to(format!("{} <{}>", user.username, user.email)
                .parse()
                .unwrap())
            .subject("Password Reset for M3m0ry.io")
            .header(ContentType::TEXT_HTML)
            .body(format!(
                "Click <a href='{}/auth/password_reset/{}'>here</a> to reset your password",
                option_env!("DOMAIN").unwrap_or("http://localhost:5173"),
                token
            ))
            .unwrap();
        client.send(&email)?;
        debug!("Reset Token sent");

        return Ok(());
    }
    bail!("Email not found");
}
