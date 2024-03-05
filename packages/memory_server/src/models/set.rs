
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
    pub cards: Vec<ObjectId>,
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
    pub cards: Vec<ObjectId>,
}

#[derive(Deserialize, Debug, Serialize)]
pub enum PatchSet {
    AddCard { front: String, back: String },
    UpdateCard {  card_id: String, front: String, back: String },
    RemoveCard { card_id: String },
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
    let mut a =  db
        .db()
        .collection::<Set>("sets")
        .aggregate(
            vec![doc! {"$match":{"_id":id}},
            doc! {
                "$lookup":{
                    "from":"cards",
                    "localField":"cards",
                    "foreignField":"_id",
                    "as":"cards"
                }
            }],
            None
        )
        .await?;
    let set = a.next().await;
    if let Some(set) = set {
        let set: Document = set?;
        let set = bson::from_bson(bson::Bson::Document(set))?;
        Ok(Some(set))
    } else {
        Ok(None)
    }
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


pub async fn delete_set(
    db: &MongoDatabase,
    id: &ObjectId,
    user_id: &ObjectId,
) -> Result<bool, mongodb::error::Error> {
    let set = get_set(db, id).await?;
    if let Some(set) = set {
        if set.user_id == *user_id {
            db
                .db()
                .collection::<Card>("cards")
                .delete_many(doc! {"_id":{"$in":set.cards}}, None)
                .await?;
            db.db()
                .collection::<Set>("sets")
                .delete_one(doc! {"_id":id}, None)
                .await?;
        }
    }
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

