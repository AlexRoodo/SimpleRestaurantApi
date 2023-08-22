use postgres_item_repository::PgItemRepository;
use sqlx::{Pool, Postgres};

pub mod dao;
pub mod error;
pub mod item_repository;
pub mod postgres_item_repository;
pub mod postgres_repositories;
pub mod repositories;

pub async fn truncate_table(connection_pool: Pool<Postgres>) {
    sqlx::query("DELETE FROM tbl_item")
        .execute(&connection_pool)
        .await
        .and(
            sqlx::query("ALTER SEQUENCE tbl_item_id_seq RESTART WITH 1")
                .execute(&connection_pool)
                .await,
        )
        .unwrap();
}

pub async fn init_test_db() -> PgItemRepository {
    let result = PgItemRepository::init_test().await;
    truncate_table(result.connection_pool.clone()).await;
    result
}
