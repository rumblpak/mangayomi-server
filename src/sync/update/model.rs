use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Update {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub oid: Option<ObjectId>,
    pub id: i32,
    pub chapter_name: String,
    pub date: String,
    pub chapter: i32,
    pub user: Option<ObjectId>,
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateList {
    pub updates: Vec<Update>,
    pub deleted_updates: Vec<i32>,
}
