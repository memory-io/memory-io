
use futures_util::{StreamExt, TryStreamExt};
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::FindOptions,
    results::DeleteResult,
};
use serde::{Deserialize, Serialize};
use tracing::warn;

use super::{set::{self, get_set_with_cards, Set}, MongoDatabase};


#[derive(Deserialize, Serialize, Debug)]
pub struct Card {
    #[serde(alias = "_id")]
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub front: String,
    pub back: String,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct NewCard {
    pub front: String,
    pub back: String,
}

pub async fn add_card_to_set(
    db: &MongoDatabase,
    set_id: &ObjectId,
    user_id: &ObjectId,
    card: NewCard
) -> Result<(), mongodb::error::Error> {
    let card = db.db().collection("cards").insert_one(card, None).await?;

    db.db().collection::<Set>("sets").update_one(
        doc! {"_id":set_id,"user_id":user_id},
        doc! {"$push": {"cards": card.inserted_id}},
        None
    ).await?;
    Ok(())
}

pub async fn update_card_in_set(
    db: &MongoDatabase,
    set_id: &ObjectId,
    user_id: &ObjectId,
    card_id: &ObjectId,
    card: NewCard
) -> Result<bool, mongodb::error::Error> {

    let Some(set) = get_set_with_cards(&db, set_id).await? else{
        warn!("Set not found");
        return Ok(false);
    };

    if set.user_id == *user_id && set.cards.iter().find(|a|a.id == *card_id).is_some() {
        db.db().collection::<Card>("cards").update_one(
            doc! {"_id":card_id},
            doc! {"$set": {"front": card.front, "back": card.back}},
            None
        ).await?;
        return Ok(true);
    }
    
    Ok(false)
}

pub async fn remove_card_from_set(
    db: &MongoDatabase,
    set_id: &ObjectId,
    user_id: &ObjectId,
    card_id: &ObjectId
) -> Result<bool, mongodb::error::Error> {
    let Some(set) = get_set_with_cards(&db, set_id).await? else{
        warn!("Set not found");
        return Ok(false);
    };

    if set.user_id == *user_id && set.cards.iter().find(|a|a.id == *card_id).is_some() {
        db.db().collection::<Set>("sets").update_one(
            doc! {"_id":set_id},
            doc! {"$pull": {"cards": card_id}},
            None
        ).await?;
        db.db().collection::<Card>("cards").delete_one(
            doc! {"_id":card_id},
            None
        ).await?;
        return Ok(true);
    }
    
    Ok(false)
}
