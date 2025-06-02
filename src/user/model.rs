use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,
    pub email: String,
    pub password: String,
    pub salt: String,
    pub role: String,
    pub created_at: i32,
    pub updated_at: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Backup {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,
    pub created_at: Option<i32>,
    pub backup_path: Option<String>,
    pub user: i32,
}

#[derive(Deserialize)]
pub struct BasicUser {
    pub email: String,
    pub(crate) password: String,
}
