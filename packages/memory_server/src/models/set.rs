
use bson::Document;
use futures_util::{StreamExt, TryStreamExt};
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::FindOptions,
    results::DeleteResult,
};
use serde::{Deserialize, Serialize};

use super::{card::Card, MongoDatabase};
#[derive(Deserialize, Serialize, Debug)]
pub struct Set {
    #[serde(alias = "_id")]
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub visibility: SetVisibility,
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub user_id: ObjectId,
    pub title: String,
    pub cards: Vec<Card>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SetWithCards {
    #[serde(alias = "_id")]
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub visibility: SetVisibility,
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub user_id: ObjectId,
    pub title: String,
    pub cards: Vec<Card>,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum SetVisibility {
    Public,
    Private,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct CreateSet {
    pub user_id: ObjectId,
    pub title: String,
    pub visibility: SetVisibility,
    pub cards: Vec<Card>,
}

#[derive(Deserialize, Debug, Serialize)]
pub enum PatchSet {
    AddCard { front: String, back: String },
    UpdateCard (Card),
    RemoveCard { id: bson::Uuid, },
}


pub async fn create_set(
    db: &MongoDatabase,
    set: CreateSet,
) -> Result<mongodb::results::InsertOneResult, mongodb::error::Error> {
    return db.db().collection("sets").insert_one(set, None).await;
}

pub async fn get_set_with_cards(
    db: &MongoDatabase,
    id: &ObjectId,
) -> Result<Option<SetWithCards>, mongodb::error::Error> {
    let set = db
    .db()
    .collection::<SetWithCards>("sets")
    .find_one(doc! {"_id":id}, None)
    .await?;
    Ok(set)
}

pub async fn get_set(
    db: &MongoDatabase,
    id: &ObjectId,
) -> Result<Option<Set>, mongodb::error::Error> {
    let set = db
        .db()
        .collection::<Set>("sets")
        .find_one(doc! {"_id":id}, None)
        .await?;
    Ok(set)
}

pub async fn get_most_recent_public_sets(
    db: &MongoDatabase,
    count: usize
) -> Result<Vec<Set>, mongodb::error::Error> {
    let result: Vec<Set> = db
        .db()
        .collection::<Set>("sets")
        .find(
            doc! {"visibility":"Public"},
            Some(FindOptions::builder().limit(Some(count as i64)).build()),
        )
        .await?
        .try_collect()
        .await?;
    Ok(result)
}


pub async fn delete_set(
    db: &MongoDatabase,
    id: &ObjectId,
    user_id: &ObjectId,
) -> Result<bool, mongodb::error::Error> {
    db.db()
        .collection::<Set>("sets")
        .delete_one(doc! {"_id":id,"user_id":user_id}, None)
        .await?;
    Ok(true)
}
pub async fn get_sets_from_user(
    db: &MongoDatabase,
    id: &ObjectId,
    count: i64,
) -> Result<Vec<Set>, mongodb::error::Error> {
    let result: Vec<Set> = db
        .db()
        .collection::<Set>("sets")
        .find(
            doc! {"user_id":id},
            Some(FindOptions::builder().limit(Some(count)).build()),
        )
        .await?
        .try_collect()
        .await?;

    Ok(result)
}

