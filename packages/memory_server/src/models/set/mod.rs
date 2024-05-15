pub mod model;

use self::model::{CreateSet, OptionSet, Set, UpdateSet};
use super::MongoDatabase;
use bson::de;
use futures_util::{future, StreamExt, TryStreamExt};
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::FindOptions,
};
use tracing::{debug, trace};

pub async fn create_set(
    db: &MongoDatabase,
    set: CreateSet,
) -> Result<mongodb::results::InsertOneResult, mongodb::error::Error> {
    debug!("Creating Set");
    let a = db.db().collection("sets").insert_one(set, None).await?;
    debug!("Created Set");
    return Ok(a);
}
pub async fn update_set(
    db: &MongoDatabase,
    set_id: &ObjectId,
    user_id: &ObjectId,
    set: &UpdateSet,
) -> Result<bool, mongodb::error::Error> {
    debug!("Updating Set");
    let result = db
        .db()
        .collection::<Set>("sets")
        .update_one(
            doc! {"_id":set_id,"user_id":user_id},
            doc! {"$set":bson::to_bson(set)?},
            None,
        )
        .await?;
    debug!("Updated Set");
    Ok(result.matched_count == 1)
}

pub async fn get_set(
    db: &MongoDatabase,
    id: &ObjectId,
    include_users: bool,
    include_cards: bool,
) -> Result<Option<OptionSet>, mongodb::error::Error> {
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
    trace!("Running Query");

    let Some(result) = db
        .db()
        .collection::<OptionSet>("sets")
        .aggregate(query, None)
        .await?
        .next()
        .await
    else {
        debug!("No Result");
        return Ok(None);
    };
    debug!("Recieved Results");

    let a = bson::from_document::<OptionSet>(result?)?;
    debug!("Parsed as OptionSet");

    Ok(Some(a))
}

pub async fn get_most_recent_public_sets(
    db: &MongoDatabase,
    count: usize,
) -> Result<Vec<Set>, mongodb::error::Error> {
    debug!("Getting most recent public sets");
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
    debug!("Recieved most recent public sets");

    Ok(result)
}

pub async fn delete_set(
    db: &MongoDatabase,
    id: &ObjectId,
    user_id: &ObjectId,
) -> Result<bool, mongodb::error::Error> {
    debug!("Deleting Set");
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
    include_user: bool,
    include_cards: bool,
) -> Result<Vec<OptionSet>, mongodb::error::Error> {
    let mut query = if include_user {
        debug!("Including user");
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
        debug!("Not including user");

        vec![doc! {
            "$match": {
                "user_id":id,
            }
        }]
    };

    if include_cards {
        debug!("Including cards");
        query.push(doc! {
            "$project": {
                "cards": 0
            }
        });
    }
    debug!("Running Query");
    trace!("Query: {query:?}");

    let result: Vec<OptionSet> = db
        .db()
        .collection::<OptionSet>("sets")
        .aggregate(query, None)
        .await?
        .filter_map(|a| future::ready(a.ok().and_then(|a| bson::from_document(a).ok())))
        .take(count as usize)
        .collect()
        .await;
    debug!("Recieved Result");
    Ok(result)
}
