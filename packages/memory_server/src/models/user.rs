use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

use super::MongoDatabase;
#[derive(Deserialize,Serialize, Debug)]
pub struct User{
    #[serde(alias = "_id")]
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub id:  ObjectId,
   // pub username: String,
    pub email: String,
    pub password: String,
}

impl User{
    pub fn validate(&self) -> Result<(),&str>{
        // if self.username.len() < 3 || self.username.len() > 20 || self.username.contains(" ") {
        //     return Err("Username must be between 3 and 20 characters");
        // }
        if self.password.len() < 3 || self.password.len() > 20{
            return Err("Password must be between 3 and 20 characters");
        }
        
        return Ok(());
    }
}


#[derive(Deserialize, Debug, Serialize)]
pub struct UserSignup {
    //pub username: String,
    pub email: String,
    pub password: String,
}




pub async fn create_user(db: &MongoDatabase, mut user: UserSignup) -> Result<mongodb::results::InsertOneResult, mongodb::error::Error>{
    user.password = bcrypt::hash(user.password, 12).unwrap();
    return db.db().collection("users").insert_one(user, None).await
}

pub async fn authenticate_user(db: &MongoDatabase, email:&str,password:&str) -> Result<Option<ObjectId>, mongodb::error::Error>{

    let a = db.db().collection::<User>("users").find_one(doc!{"email":email}, None).await?;
    if let Some(user) = a{
        if  bcrypt::verify(password, &user.password).unwrap(){
            return Ok(Some(user.id));
        }
    }
    return Ok(None);

}