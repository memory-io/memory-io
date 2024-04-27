use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct User {
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

#[derive(Deserialize, Serialize, Debug)]
pub struct PasswordReset {
    pub creation_date: bson::DateTime,
    pub user_id: ObjectId,
    pub token: String,
}
