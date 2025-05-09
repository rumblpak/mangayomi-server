use tokio::sync::OnceCell;
use lazy_static::lazy_static;
use sea_orm::DatabaseConnection;

lazy_static! {
    /// Global variable for the database connection
    pub static ref CONN: OnceCell<DatabaseConnection> = OnceCell::const_new();
}
