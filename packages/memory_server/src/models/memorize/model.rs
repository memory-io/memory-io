use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Memorize {
    #[serde(alias = "_id")]
    #[serde(skip_serializing)]
    pub id: ObjectId,
    pub set_id: ObjectId,
    pub user_id: ObjectId,
    pub last_answered: bson::DateTime,
    pub answers: Vec<MemorizeCardQuestionData>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MemorizeCardQuestionData {
    pub card_id: bson::Uuid,
    pub correct: bool,
    pub answer: String,
    pub difficulty: Option<u32>,
}
