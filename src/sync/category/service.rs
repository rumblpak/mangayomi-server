use crate::entity::categories::{ActiveModel, Column, Entity, Model};
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, ColumnTrait, Set};
use sea_orm::{DatabaseConnection, EntityTrait};

/// returns an account with the matching email
pub async fn sync_category_list(
    user_id: i32,
    category_list: &Vec<Model>,
    db: &DatabaseConnection,
) -> Vec<Model> {
    for category in category_list {
        upsert_category(user_id, category, db).await;
    }
    find_all_categories(user_id, db).await
}

/// returns an account with the matching email
async fn upsert_category(user_id: i32, category: &Model, db: &DatabaseConnection) -> Option<Model> {
    let item = Entity::find()
        .filter(Column::User.eq(user_id).and(Column::Id.eq(category.id)))
        .one(db)
        .await
        .map_or(None, |category| category);
    if item.is_none() {
        let active_item = build_active_model(user_id, category, None);
        return match active_item.insert(db).await {
            Ok(model) => Some(model),
            Err(err) => {
                log::error!("{}", err);
                None
            }
        };
    }
    if item.clone().unwrap().updated_at < category.updated_at {
        let active_item = build_active_model(user_id, category, item);
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

fn build_active_model(user_id: i32, category: &Model, active_item: Option<Model>) -> ActiveModel {
    let mut model = if active_item.is_none() {
        ActiveModel::new()
    } else {
        ActiveModel::from(active_item.unwrap())
    };
    model.id = Set(category.id);
    model.name = Set(category.name.to_owned());
    model.for_item_type = Set(category.for_item_type.to_owned());
    model.pos = Set(category.pos.to_owned());
    model.hide = Set(category.hide.to_owned());
    model.user = Set(user_id);
    model.updated_at = Set(category.updated_at);
    model
}

/// returns all categories of a specific user
async fn find_all_categories(user_id: i32, db: &DatabaseConnection) -> Vec<Model> {
    Entity::find()
        .filter(Column::User.eq(user_id))
        .all(db)
        .await
        .unwrap()
}
