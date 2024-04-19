use crate::models::user::User;
use bson::Document;
use futures_util::{future, StreamExt, TryStreamExt};
use mongodb::options::AggregateOptions;
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::FindOptions,
    results::DeleteResult,
};
use serde::{Deserialize, Serialize};
use tracing::debug;

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
pub struct OptionSet {
    #[serde(alias = "_id")]
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub visibility: SetVisibility,
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub user_id: ObjectId,
    pub user: Option<User>,
    pub title: String,
    pub cards: Option<Vec<Card>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SetWithCards {
    #[serde(alias = "_id")]
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub visibility: SetVisibility,
    pub folder_id: Option<ObjectId>,
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
    UpdateCard(Card),
    RemoveCard { id: bson::Uuid },
}

pub async fn create_set(
    db: &MongoDatabase,
    set: CreateSet,
) -> Result<mongodb::results::InsertOneResult, mongodb::error::Error> {
    return db.db().collection("sets").insert_one(set, None).await;
}

pub async fn get_set(
    db: &MongoDatabase,
    id: &ObjectId,
    include_users: bool,
    include_cards: bool,
) -> Result<Option<OptionSet>, mongodb::error::Error> {
    // let set = db
    //     .db()
    //     .collection::<Set>("sets")
    //     .find_one(doc! {"_id":id}, None)
    //     .await?;
    let mut query = if include_users {
        debug!("Including users");
        vec![
            doc! {
                "$match": {
                    "_id":id,
                }
            },
            doc! {
                "$lookup": {
                    "from": "users",
                    "localField": "user_id",
                    "foreignField": "_id",
                    "as": "user"
                }
            },
        ]
    } else {
        debug!("Non including users");

        vec![doc! {
            "$match": {
                "_id":id,
            }
        }]
    };
    if !include_cards {
        debug!("Including Cards");

        query.push(doc! {
            "$project": {
                "cards": 0
            }
        });
    }

    let Some(result) = db
        .db()
        .collection::<OptionSet>("sets")
        .aggregate(query, None)
        .await?
        .next()
        .await
    else {
        return Ok(None);
    };
    let a = bson::from_document::<OptionSet>(result?)?;
    Ok(Some(a))
}

pub async fn get_most_recent_public_sets(
    db: &MongoDatabase,
    count: usize,
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
    include_users: bool,
    include_cards: bool,
) -> Result<Vec<OptionSet>, mongodb::error::Error> {
    let mut query = if include_users {
        vec![
            doc! {
                "$match": {
                    "user_id":id,
                }
            },
            doc! {
                "$lookup": {
                  "from": "users",
                  "localField": "user_id",
                  "foreignField": "_id",
                  "as": "user"
                }
            },
        ]
    } else {
        vec![doc! {
            "$match": {
                "user_id":id,
            }
        }]
    };

    if include_cards {
        query.push(doc! {
            "$project": {
                "cards": 0
            }
        });
    }

    let result: Vec<OptionSet> = db
        .db()
        .collection::<OptionSet>("sets")
        .aggregate(query, None)
        .await?
        .filter_map(|a| future::ready(a.ok().map(|a| bson::from_document(a).ok()).flatten()))
        .take(count as usize)
        .collect()
        .await;

    Ok(result)
}
