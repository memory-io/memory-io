use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
pub enum FolderPrivacy {
    Public,
    Private,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Folder {
    #[serde(alias = "_id")]
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub id: bson::oid::ObjectId,
    pub title: String,
    pub privacy: FolderPrivacy,
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub user_id: bson::oid::ObjectId,
}
