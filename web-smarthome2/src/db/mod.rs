use sqlx::{migrate::MigrateDatabase, Sqlite};

pub mod model;
pub mod schema;

///
/// Создать базу данных.
///
pub async fn create_database(db_url: &str) -> Result<(), sqlx::Error> {
    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        Sqlite::create_database(db_url).await?;
        return schema::create_schema(db_url).await.map(|_| ());
    }

    Ok(())
}
