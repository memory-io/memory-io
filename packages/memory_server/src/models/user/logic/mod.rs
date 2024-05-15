pub mod auth;
pub mod password_reset;
pub mod verify;
use std::str::FromStr;





use mongodb::{
    bson::{doc, oid::ObjectId},
};
use tracing::{debug};




use crate::{models::MongoDatabase};

use super::model::UserSendable;

pub async fn get_user(
    db: &MongoDatabase,
    user_id: String,
) -> Result<Option<UserSendable>, mongodb::error::Error> {
    debug!("Getting user");
    let user_id = ObjectId::from_str(&user_id).unwrap();
    let user = db
        .db()
        .collection("users")
        .find_one(doc! {"_id":user_id}, None)
        .await;
    debug!("Recieved user");
    user
}
