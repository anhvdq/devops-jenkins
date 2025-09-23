use sqlx::Row;
use web_service::config::{
    pg_database::{PgDatabase, PgDatabaseTrait},
    settings,
};

#[tokio::test]
async fn test_database_connection_success() {
    settings::init();
    let db_conn = PgDatabase::init()
        .await
        .unwrap_or_else(|e| panic!("Database error: {e}"));

    let pool = db_conn.get_pool();

    let result = sqlx::query("SELECT 1 as test_value").fetch_one(pool).await;

    assert!(result.is_ok(), "Database connection should be successful");
    assert_eq!(result.unwrap().get::<i32, _>("test_value"), 1);

    pool.close().await;
}

#[tokio::test]
async fn test_database_migration() {
    settings::init();
    let db_conn = PgDatabase::init()
        .await
        .unwrap_or_else(|e| panic!("Database error: {e}"));

    let pool = db_conn.get_pool();

    sqlx::migrate!("./migrations").run(pool).await.unwrap();
}
