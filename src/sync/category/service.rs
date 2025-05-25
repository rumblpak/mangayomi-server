use crate::entity::categories::{ActiveModel, Column, Entity, Model};
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, ColumnTrait, Set};
use sea_orm::{DatabaseConnection, EntityTrait};

/// returns an account with the matching email
pub async fn sync_category_list(
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
