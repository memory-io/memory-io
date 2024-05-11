use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;
use zxcvbn::zxcvbn;
#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    #[serde(alias = "_id")]
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub username: String,
    pub email: String,
    pub password: String,
    pub paid_user: bool,
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
    #[serde(default,skip_deserializing)]
    pub paid_user: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserSendable {
    #[serde(alias = "_id")]
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub username: String,
    pub email: String,
    pub paid_user: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PasswordReset {
    pub creation_date: bson::DateTime,
    pub user_id: ObjectId,
    pub token: String,
}
