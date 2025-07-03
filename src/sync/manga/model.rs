use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub trait Model {
    fn get_id(&self) -> i32;
    fn get_updated_at(&self) -> i64;
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Category {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub oid: Option<ObjectId>,
    pub id: i32,
    pub name: String,
    #[serde(rename = "forItemType")]
    pub for_item_type: i32,
    pub pos: Option<i32>,
    pub hide: Option<bool>,
    pub user: Option<ObjectId>,
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
}

impl Model for Category {
    fn get_id(&self) -> i32 {
        self.id
    }
    fn get_updated_at(&self) -> i64 {
        self.updated_at
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Manga {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub oid: Option<ObjectId>,
    pub id: i32,
    pub name: String,
    pub link: String,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub artist: Option<String>,
    pub status: i32,
    pub favorite: bool,
    pub source: String,
    pub lang: String,
    #[serde(rename = "dateAdded")]
    pub date_added: i64,
    #[serde(rename = "lastUpdate")]
    pub last_update: Option<i64>,
    #[serde(rename = "lastRead")]
    pub last_read: Option<i64>,
    #[serde(rename = "isLocalArchive")]
    pub is_local_archive: Option<bool>,
    #[serde(rename = "customCoverImage")]
    pub custom_cover_image: Option<String>,
    #[serde(rename = "customCoverFromTracker")]
    pub custom_cover_from_tracker: Option<String>,
    #[serde(rename = "itemType")]
    pub item_type: i32,
    pub genres: Option<String>,
    pub user: Option<ObjectId>,
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
}

impl Model for Manga {
    fn get_id(&self) -> i32 {
        self.id
    }
    fn get_updated_at(&self) -> i64 {
        self.updated_at
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Chapter {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub oid: Option<ObjectId>,
    pub id: i32,
    pub name: String,
    pub url: String,
    pub date_upload: String,
    pub scanlator: Option<String>,
    pub is_bookmarked: bool,
    pub is_read: bool,
    pub last_page_read: String,
    pub archive_path: Option<String>,
    pub manga: i32,
    pub user: Option<ObjectId>,
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
}

impl Model for Chapter {
    fn get_id(&self) -> i32 {
        self.id
    }
    fn get_updated_at(&self) -> i64 {
        self.updated_at
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Track {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub oid: Option<ObjectId>,
    pub id: i32,
    pub library_id: i32,
    pub media_id: i32,
    pub manga_id: i32,
    pub score: Option<i32>,
    pub started_reading_date: Option<i64>,
    pub finished_reading_date: Option<i64>,
    pub last_chapter_read: Option<i32>,
    pub track_status_index: i32,
    pub sync_id: i32,
    pub title: String,
    pub total_chapter: Option<i32>,
    pub tracking_url: String,
    pub is_manga: Option<bool>,
    pub user: Option<ObjectId>,
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
}

impl Model for Track {
    fn get_id(&self) -> i32 {
        self.id
    }
    fn get_updated_at(&self) -> i64 {
        self.updated_at
    }
}

#[derive(Serialize, Deserialize)]
pub struct MangaList {
    pub categories: Vec<Category>,
    pub deleted_categories: Vec<i32>,
    pub manga: Vec<Manga>,
    pub deleted_manga: Vec<i32>,
    pub chapters: Vec<Chapter>,
    pub deleted_chapters: Vec<i32>,
    pub tracks: Vec<Track>,
    pub deleted_tracks: Vec<i32>,
}
