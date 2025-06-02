use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Category {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub for_item_type: i32,
    pub pos: i32,
    pub hide: i8,
    pub user: i32,
    pub updated_at: i32,
}

#[derive(Serialize, Deserialize)]
pub struct CategoryList {
    pub categories: Vec<Category>,
}
