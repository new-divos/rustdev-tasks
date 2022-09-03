use sqlx::SqlitePool;

///
/// Создать схему базы данных.
///
pub(crate) async fn create_schema(db_url: &str) -> Result<(), sqlx::Error> {
    let pool = SqlitePool::connect(db_url).await?;
    sqlx::query(
        "
        PRAGMA foreign_keys = ON ;

        CREATE TABLE IF NOT EXISTS houses
        (
            id   BLOB(16) PRIMARY KEY NOT NULL,
            name TEXT NOT NULL UNIQUE
        );

        CREATE TABLE IF NOT EXISTS rooms
        (
            id       BLOB(16) PRIMARY KEY NOT NULL,
            name     TEXT NOT NULL,
            house_id BLOB(16) NOT NULL,

            CONSTRAINT fk_houses
                FOREIGN KEY (house_id)
                REFERENCES houses(id)
                ON DELETE CASCADE
        );
    ",
    )
    .execute(&pool)
    .await?;

    pool.close().await;
    Ok(())
}
