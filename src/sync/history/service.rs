use crate::entity::manga::{ActiveModel, Column, Entity, Model};
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, ColumnTrait, Set};
use sea_orm::{DatabaseConnection, EntityTrait};

/// returns an account with the matching email
pub async fn sync_manga_list(
    user_id: i32,
    manga_list: &Vec<Model>,
    db: &DatabaseConnection,
) -> Vec<Model> {
    for manga in manga_list {
        upsert_manga(user_id, manga, db).await;
    }
    find_all_manga(user_id, db).await
}

/// returns an account with the matching email
async fn upsert_manga(user_id: i32, manga: &Model, db: &DatabaseConnection) -> Option<Model> {
    let item = Entity::find()
        .filter(Column::User.eq(user_id).and(Column::Id.eq(manga.id)))
        .one(db)
        .await
        .map_or(None, |manga| manga);
    if item.is_none() {
        let active_item = build_active_model(user_id, manga, None);
        return match active_item.insert(db).await {
            Ok(model) => Some(model),
            Err(err) => {
                log::error!("{}", err);
                None
            }
        };
    }
    if item.clone().unwrap().updated_at < manga.updated_at {
        let active_item = build_active_model(user_id, manga, item);
        return match active_item.update(db).await {
            Ok(model) => Some(model),
            Err(err) => {
                log::error!("{}", err);
                None
            }
        };
    }
    None
}

fn build_active_model(user_id: i32, manga: &Model, active_item: Option<Model>) -> ActiveModel {
    let mut model = if active_item.is_none() {
        ActiveModel::new()
    } else {
        ActiveModel::from(active_item.unwrap())
    };
    model.id = Set(manga.id);
    model.name = Set(manga.name.to_owned());
    model.artist = Set(manga.artist.to_owned());
    model.author = Set(manga.author.to_owned());
    model.custom_cover_from_tracker = Set(manga.custom_cover_from_tracker.to_owned());
    model.custom_cover_image = Set(manga.custom_cover_image.to_owned());
    model.date_added = Set(manga.date_added.to_owned());
    model.description = Set(manga.description.to_owned());
    model.favorite = Set(manga.favorite.to_owned());
    model.genres = Set(manga.genres.to_owned());
    model.image_url = Set(manga.image_url.to_owned());
    model.is_local_archive = Set(manga.is_local_archive.to_owned());
    model.item_type = Set(manga.item_type.to_owned());
    model.lang = Set(manga.lang.to_owned());
    model.last_read = Set(manga.last_read.to_owned());
    model.last_update = Set(manga.last_update.to_owned());
    model.link = Set(manga.link.to_owned());
    model.source = Set(manga.source.to_owned());
    model.status_index = Set(manga.status_index.to_owned());
    model.user = Set(user_id);
    model.updated_at = Set(manga.updated_at);
    model
}

/// returns all manga of a specific user
async fn find_all_manga(user_id: i32, db: &DatabaseConnection) -> Vec<Model> {
    Entity::find()
        .filter(Column::User.eq(user_id))
        .all(db)
        .await
        .unwrap()
}
