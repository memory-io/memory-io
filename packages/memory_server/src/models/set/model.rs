use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::models::{card::Card, user::model::User};

#[derive(Deserialize, Serialize, Debug)]
pub struct Set {
    #[serde(alias = "_id")]
    pub id: ObjectId,
    pub visibility: SetVisibility,
    pub user_id: ObjectId,
    pub title: String,
    pub description: Option<String>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateSet {
    pub visibility: SetVisibility,
    pub user_id: ObjectId,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OptionSet {
    #[serde(alias = "_id")]
    pub id: ObjectId,
    pub visibility: SetVisibility,
    pub user_id: ObjectId,
    pub user: Option<User>,
    pub title: String,
    pub description: Option<String>,
    pub cards: Option<Vec<Card>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SetWithCards {
    #[serde(alias = "_id")]
    pub id: ObjectId,
    pub visibility: SetVisibility,
    pub folder_id: Option<ObjectId>,
    pub user_id: ObjectId,
    pub title: String,
    pub description: Option<String>,
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
    pub description: Option<String>,
    pub visibility: SetVisibility,
    pub cards: Vec<Card>,
}

#[derive(Deserialize, Debug, Serialize)]
pub enum PatchSet {
    AddCard { front: String, back: String },
    UpdateCard(Card),
    UpdateSet(UpdateSet),
    RemoveCard { id: bson::Uuid },
}
