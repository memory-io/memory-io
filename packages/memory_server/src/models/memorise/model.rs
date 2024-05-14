#[derive(Deserialize, Serialize, Debug)]
pub struct Memorize {
    #[serde(alias = "_id")]
    pub id: bson::ObjectId,
    pub set_id: bson::ObjectId,
    pub user_id: bson::ObjectId,

    pub answers: Vec<MemorizeCardData>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MemorizeCardData {
    pub card_id: bson::Uuid,
    pub answers: Vec<bool>,
}
