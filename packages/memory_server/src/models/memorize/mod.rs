use std::mem;

use bson::{doc, oid::ObjectId};
use tracing::debug;
use tracing_subscriber::field::debug;
use anyhow::bail;
use crate::models::{memorize::model::Memorize, set::get_set};

use super::MongoDatabase;

pub mod model;

pub async fn get_memorize_data(
    db: &MongoDatabase,
    user_id: ObjectId,
    set_id: ObjectId,
) -> Result<Option<Memorize>, mongodb::error::Error> {
    debug!("Getting memorize data");
    let memorize = db
        .db()
        .collection::<Memorize>("memorize")
        .find_one(doc! {"set_id":set_id,"user_id":user_id}, None)
        .await?;
    debug!("Got memorize data");
    return Ok(memorize);
}

pub async fn add_memorize_data(
    db: &MongoDatabase,
    user_id: ObjectId,
    set_id: ObjectId,
    mut answers: Vec<model::MemorizeCardQuestionData>,
) -> Result<(), mongodb::error::Error> {
    debug!("Adding memorise data");
    let memorize = get_memorize_data(db, user_id, set_id).await?;
    if let Some(memorize) = memorize {
        debug!("Updating memorize data");
        answers.extend(memorize.answers);
        db.db()
            .collection::<Memorize>("memorize")
            .update_one(
                doc! {"_id":memorize.id},
                doc! {"$set":doc!{"answers":bson::to_bson(&answers).unwrap(),"last_answered":bson::DateTime::now()}},
                None,
            )
            .await?;
    } else {
        debug!("Creating memorize data");
        let memorize = Memorize {
            id: ObjectId::new(),
            set_id,
            user_id,
            answers,
            last_answered: bson::DateTime::now(),
        };
        db.db()
            .collection::<Memorize>("memorize")
            .insert_one(memorize, None)
            .await?;
    }
    debug!("Finished adding memorize data");
    return Ok(());
}

