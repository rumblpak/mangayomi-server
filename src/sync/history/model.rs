use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct History {
    pub id: i32,
    pub date: String,
    pub chapter: i32,
    pub item_type: i32,
    pub updated_at: i32,
}
