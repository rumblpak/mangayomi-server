use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub salt: String,
    pub roles_id: i32,
    pub created_at: i32,
    pub updated_at: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Backup {
    pub id: i32,
    pub created_at: Option<i32>,
    pub backup_path: Option<String>,
    pub user: i32,
}

#[derive(Deserialize)]
pub struct BasicUser {
    pub email: String,
    pub(crate) password: String,
}
