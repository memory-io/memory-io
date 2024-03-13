use std::str::FromStr;

use actix_identity::Identity;
use mongodb::{
    bson::{doc, oid::ObjectId},
    error::Error,
};
use serde::{Deserialize, Serialize};

use super::MongoDatabase;
#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    #[serde(alias = "_id")]
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn validate(&self) -> Result<(), &str> {
        // if self.username.len() < 3 || self.username.len() > 20 || self.username.contains(" ") {
        //     return Err("Username must be between 3 and 20 characters");
        // }
        if self.password.len() < 3 || self.password.len() > 20 {
            return Err("Password must be between 3 and 20 characters");
        }

        return Ok(());
    }
}

#[derive(Deserialize, Debug, Serialize)]
pub struct UserSignup {
    pub username: String,
    pub email: String,
    pub password: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct UserSendable {
    #[serde(alias = "_id")]
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub username: String,
    pub email: String,
}

pub async fn check_username(
    db: &MongoDatabase,
    username: &str,
) -> Result<(), mongodb::error::Error> {
    if username.len() < 3 || username.len() > 20 || username.contains(" ") {
        return Err(Error::custom(
            "Username must be between 3 and 20 characters".to_string(),
        ));
    }
    let user = db
        .db()
        .collection::<User>("users")
        .find_one(doc! {"username":username}, None)
        .await?;
    if user.is_some() {
        return Err(Error::custom("Be More Original".to_string()));
    }
    return Ok(());
}

pub async fn get_user(
    db: &MongoDatabase,
    user_id: String,
) -> Result<Option<UserSendable>, mongodb::error::Error> {
    let user_id = ObjectId::from_str(&user_id).unwrap();
    return db
        .db()
        .collection("users")
        .find_one(doc! {"_id":user_id}, None)
        .await;
}

pub async fn create_user(
    db: &MongoDatabase,
    mut user: UserSignup,
) -> Result<mongodb::results::InsertOneResult, mongodb::error::Error> {
    user.password = bcrypt::hash(user.password, 12).unwrap();
    return db.db().collection("users").insert_one(user, None).await;
}

pub async fn authenticate_user(
    db: &MongoDatabase,
    email: &str,
    password: &str,
) -> Result<Option<ObjectId>, mongodb::error::Error> {
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
    return Ok(None);
}
