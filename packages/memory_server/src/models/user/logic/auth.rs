use anyhow::bail;

use mongodb::{
    bson::{doc, oid::ObjectId},
    error::WriteFailure,
};
use tracing::{debug, error, trace};

use validator::Validate;
use zxcvbn::zxcvbn;

use crate::models::{
    user::model::{User, UserSignup},
    MongoDatabase,
};

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
pub async fn create_user(
    db: &MongoDatabase,
    mut user: UserSignup,
) -> Result<mongodb::results::InsertOneResult, anyhow::Error> {
    debug!("Creating user");
    user.validate()?;
    if zxcvbn(&user.password, &[])?.score() < 3 {
        bail!("Password is too weak");
    }
    trace!("Hashing password");
    user.password = match bcrypt::hash(user.password, 12) {
        Ok(a) => a,
        Err(err) => {
            error!("Failed to hash password Err: {err}");
            bail!("Failed to hash password");
        }
    };
    trace!("Checking username");
    check_username(db, &user.username).await?;
    debug!("Inserting user");
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

pub(crate) async fn delete_user(
    db: &actix_web::web::Data<MongoDatabase>,
    user_id: ObjectId,
) -> Result<(), mongodb::error::Error> {
    db.db()
        .collection::<User>("users")
        .delete_one(
            doc! {
                "_id": user_id
            },
            None,
        )
        .await?;
    return Ok(());
}
