use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{DatabaseConnection, EntityTrait};

/// returns an account with the matching email
async fn find_all_manga(
    email: &String,
    db: &DatabaseConnection,
) -> Vec<crate::entity::manga::Model> {
    crate::entity::manga::Entity::find()
        .filter(crate::entity::accounts::Column::Email.eq(email))
        .all(db)
        .await.unwrap()
}
