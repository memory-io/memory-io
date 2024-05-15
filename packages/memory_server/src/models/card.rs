use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use tracing::{debug, instrument};

use super::{set::model::SetWithCards, MongoDatabase};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Card {
    pub id: bson::Uuid,
    pub front: String,
    pub back: String,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CardNoID {
    pub front: String,
    pub back: String,
}

#[instrument(skip(db), fields(user_id = %user_id, set_id = %set_id))]
pub async fn add_card_to_set(
    db: &MongoDatabase,
    set_id: &ObjectId,
    user_id: &ObjectId,
    card: Card,
) -> Result<(), mongodb::error::Error> {
    debug!("Adding card");

    db.db()
        .collection::<SetWithCards>("sets")
        .update_one(
            doc! {"_id":set_id,"user_id":user_id},
            doc! {"$push": {"cards": bson::to_bson(&card).unwrap()}},
            None,
        )
        .await?;
    debug!("Card added");

    Ok(())
}
#[instrument(skip(db), fields(user_id = %user_id, set_id = %set_id))]
pub async fn update_card_in_set(
    db: &MongoDatabase,
    set_id: &ObjectId,
    user_id: &ObjectId,
    card: &Card,
) -> Result<bool, anyhow::Error> {
    debug!("Updating card");
    let result = db
        .db()
        .collection::<SetWithCards>("sets")
        .update_one(
            doc! {"_id":set_id,"user_id":user_id,"cards.id":card.id},
            doc! {"$set": {"cards.$": bson::to_bson(card)?}},
            None,
        )
        .await?;
    debug!("Updated card");

    Ok(result.matched_count == 1)
}
#[instrument(skip(db))]

pub async fn remove_card_from_set(
    db: &MongoDatabase,
    set_id: &ObjectId,
    user_id: &ObjectId,
    card_id: &bson::Uuid,
) -> Result<bool, mongodb::error::Error> {
    debug!("Removing card");

    let result = db
        .db()
        .collection::<SetWithCards>("sets")
        .update_one(
            doc! {"_id":set_id,"user_id":user_id},
            doc! {"$pull": {"cards":{
                "id":card_id
            }}},
            None,
        )
        .await?;
    debug!("Removed card");

    Ok(result.modified_count == 1)
}
