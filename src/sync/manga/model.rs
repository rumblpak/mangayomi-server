use serde::{Deserialize, Serialize};
use crate::entity::manga::Model as Manga;

#[derive(Serialize, Deserialize)]
pub struct MangaList {
    manga: Vec<Manga>,
}
