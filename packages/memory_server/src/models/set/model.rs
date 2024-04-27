use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::models::{card::Card, user::model::User};

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
