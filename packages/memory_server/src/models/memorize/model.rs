use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Memorize {
    #[serde(alias = "_id")]
    pub id: ObjectId,
    pub set_id: ObjectId,
    pub user_id: ObjectId,
    pub last_answered: bson::DateTime,
    pub answers: Vec<MemorizeCardData>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MemorizeCardData {
    pub card_id: bson::Uuid,
    pub correct: bool,
    pub when: bson::DateTime,
    
}
