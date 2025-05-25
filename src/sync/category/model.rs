use serde::{Deserialize, Serialize};
use crate::entity::categories::Model as Category;

#[derive(Serialize, Deserialize)]
pub struct CategoryList {
    pub categories: Vec<Category>,
}
