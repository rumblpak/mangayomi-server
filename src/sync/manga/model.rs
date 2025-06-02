use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Manga {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub link: String,
    pub image_url: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub artist: Option<String>,
    pub status_index: i32,
    pub favorite: i8,
    pub source: String,
    pub lang: String,
    pub date_added: i32,
    pub last_update: Option<i32>,
    pub last_read: Option<i32>,
    pub is_local_archive: Option<i8>,
    pub custom_cover_image: Option<String>,
    pub custom_cover_from_tracker: Option<String>,
    pub item_type: i32,
    pub user: i32,
    pub genres: Option<String>,
    pub updated_at: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Chapter {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub url: String,
    pub date_upload: String,
    pub scanlator: Option<String>,
    pub is_bookmarked: i8,
    pub is_read: i8,
    pub last_page_read: String,
    pub archive_path: Option<String>,
    pub manga: i32,
    pub updated_at: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Track {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,
    pub library_id: i32,
    pub media_id: i32,
    pub manga_id: i32,
    pub score: Option<i32>,
    pub started_reading_date: Option<i32>,
    pub finished_reading_date: Option<i32>,
    pub last_chapter_read: Option<i32>,
    pub track_status_index: i32,
    pub sync_id: i32,
    pub title: String,
    pub total_chapter: Option<i32>,
    pub tracking_url: String,
    pub is_manga: Option<i8>,
    pub updated_at: i32,
}

#[derive(Serialize, Deserialize)]
pub struct MangaList {
    pub manga: Vec<Manga>,
}
