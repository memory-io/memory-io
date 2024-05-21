use bson::oid::ObjectId;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use validator::Validate;
#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    #[serde(alias = "_id")]
    pub id: ObjectId,
    pub username: String,
    pub email: String,
    pub password: String,
    #[serde(default)]
    pub paid_user: bool,
    #[serde(default)]
    pub verified: bool,
}



impl User {
    pub fn validate(&self) -> Result<(), &str> {
        // if self.username.len() < 3 || self.username.len() > 20 || self.username.contains(" ") {
        //     return Err("Username must be between 3 and 20 characters");
        // }
        if self.password.len() < 3 || self.password.len() > 20 {
            return Err("Password must be between 3 and 20 characters");
        }

        Ok(())
    }
}

#[derive(Deserialize, Validate, Debug, Serialize)]
pub struct UserSignup {
    #[validate(length(min = 3, max = 20))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 3, max = 32))]
    pub password: String,
    #[serde(default, skip_deserializing)]
    pub paid_user: bool,
    #[serde(default, skip_deserializing)]
    pub verified: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserSendable {
    #[serde(alias = "_id")]
    pub id: ObjectId,
    pub username: String,
    pub email: String,
    #[serde(default)]
    pub paid_user: bool,
    #[serde(default)]
    pub verified: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PasswordReset {
    pub creation_date: bson::DateTime,
    pub user_id: ObjectId,
    pub token: String,
}
#[derive(Deserialize, Serialize, Debug)]

pub struct VerficationRequest {
    pub creation_date: bson::DateTime,

    pub user_id: ObjectId,
    pub token: String,
}
