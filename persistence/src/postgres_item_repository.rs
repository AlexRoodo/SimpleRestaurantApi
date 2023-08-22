use crate::dao::{InsertItemDao, ItemDao};
use crate::error::DbError;
use crate::item_repository::ItemRepository;
use async_trait::async_trait;
use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct PgItemRepository {
    pub connection_pool: Pool<Postgres>,
}

impl PgItemRepository {
    async fn run_migrations(connection_pool: &Pool<Postgres>) {
        sqlx::migrate!("../migrations")
            .run(connection_pool)
            .await
            .map_err(|e| DbError::from_migrate_error(e))
            .unwrap();
    }

    pub async fn init_prod() -> PgItemRepository {
        let connection_url = "postgresql://docker:docker@localhost/restaurant";
        let connection_pool = Pool::connect(connection_url)
            .await
            .map_err(|e| DbError::from_sqlx_error(e))
            .unwrap();

        PgItemRepository::run_migrations(&connection_pool).await;

        PgItemRepository { connection_pool }
    }

    pub async fn init_test() -> PgItemRepository {
        let connection_url = "postgresql://docker:docker@localhost:5432/restaurant_test";
        let connection_pool = Pool::connect(connection_url)
            .await
            .map_err(|e| DbError::from_sqlx_error(e))
            .unwrap();

        PgItemRepository::run_migrations(&connection_pool).await;

        PgItemRepository { connection_pool }
    }
}

#[async_trait]
impl ItemRepository for PgItemRepository {
    async fn add_item(&self, item: InsertItemDao) -> Result<i64, DbError> {
        let tx = self.connection_pool.begin().await.unwrap();

        let item_from_db = sqlx::query_as::<_, ItemDao>(
            r#"
            SELECT *
            FROM tbl_item
            WHERE name = $1 AND table_id = $2
            "#,
        )
        .bind(item.name.clone())
        .bind(item.table_id)
        .fetch_optional(&self.connection_pool)
        .await;

        let result = match item_from_db {
            Err(e) => return Err(DbError::SqlxError(e)),
            Ok(None) => {
                sqlx::query_as::<_, ItemDao>(
                    r#"
                    INSERT INTO tbl_item (name, table_id, time_to_prepare, quantity)
                    VALUES ($1, $2, $3, $4)
                    RETURNING *;
                    "#,
                )
                .bind(item.name)
                .bind(item.table_id)
                .bind(item.time_to_prepare)
                .bind(item.quantity)
                .fetch_one(&self.connection_pool)
                .await
            }
            Ok(Some(existind_item)) => {
                sqlx::query_as::<_, ItemDao>(
                    r#"
                    UPDATE tbl_item
                    SET quantity = quantity + $1
                    WHERE id = $2
                    RETURNING *;
                    "#,
                )
                .bind(item.quantity)
                .bind(existind_item.id)
                .fetch_one(&self.connection_pool)
                .await
            }
        };

        tx.commit().await.unwrap();

        match result {
            Ok(item) => Ok(item.id),
            Err(e) => Err(DbError::from_sqlx_error(e)),
        }
    }

    async fn get_item(&self, item_id: i64) -> Result<Option<ItemDao>, DbError> {
        let result = sqlx::query_as::<_, ItemDao>(
            r#"
            SELECT *
            FROM tbl_item WHERE id = $1
            "#,
        )
        .bind(item_id)
        .fetch_optional(&self.connection_pool)
        .await;

        match result {
            Ok(item) => Ok(item),
            Err(e) => Err(DbError::from_sqlx_error(e)),
        }
    }

    async fn get_items_for_table(&self, table_id: i32) -> Result<Vec<ItemDao>, DbError> {
        let result = sqlx::query_as::<_, ItemDao>(
            r#"
            SELECT *
            FROM tbl_item
            WHERE table_id = $1
            ORDER BY id ASC
            "#,
        )
        .bind(table_id)
        .fetch_all(&self.connection_pool)
        .await;

        match result {
            Ok(item) => Ok(item),
            Err(e) => Err(DbError::from_sqlx_error(e)),
        }
    }

    async fn get_all_items(&self) -> Result<Vec<ItemDao>, DbError> {
        let result = sqlx::query_as::<_, ItemDao>(
            r#"
            SELECT *
            FROM tbl_item
            ORDER BY table_id, id ASC
            "#,
        )
        .fetch_all(&self.connection_pool)
        .await;

        match result {
            Ok(item) => Ok(item),
            Err(e) => Err(DbError::from_sqlx_error(e)),
        }
    }

    async fn remove_item(&self, item_id: i64, quantity: i32) -> Result<(), DbError> {
        let tx = self.connection_pool.begin().await.unwrap();

        let item_from_db = sqlx::query_as::<_, ItemDao>(
            r#"
            SELECT *
            FROM tbl_item
            WHERE id = $1
            "#,
        )
        .bind(item_id)
        .fetch_optional(&self.connection_pool)
        .await;

        let result = match item_from_db {
            Err(e) => return Err(DbError::from_sqlx_error(e)),
            Ok(None) => Result::Ok(()),
            Ok(Some(existind_item)) => {
                if existind_item.quantity <= quantity {
                    sqlx::query(
                        r#"
                        DELETE FROM tbl_item
                        WHERE id = $1
                        "#,
                    )
                    .bind(item_id)
                    .execute(&self.connection_pool)
                    .await
                    .map(|_| ())
                } else {
                    sqlx::query(
                        r#"
                        UPDATE tbl_item
                        SET quantity = quantity - $1
                        WHERE id = $2
                    "#,
                    )
                    .bind(quantity)
                    .bind(item_id)
                    .execute(&self.connection_pool)
                    .await
                    .map(|_| ())
                }
            }
        };

        tx.commit().await.unwrap();

        match result {
            Ok(item) => Ok(item),
            Err(e) => Err(DbError::from_sqlx_error(e)),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::init_test_db;
    use crate::truncate_table;

    use super::*;

    #[tokio::test]
    #[serial_test::serial]
    async fn test_create_and_get_item() {
        let repository = init_test_db().await;
        let expected_item = InsertItemDao::new("sushi".to_string(), 1, 5, 2);
        let id = repository.add_item(expected_item.clone()).await.unwrap();
        let result_item = repository.get_item(id).await.unwrap().unwrap();

        assert_eq!(&result_item.id, &id);
        assert_eq!(&result_item.name, &expected_item.name);
        assert_eq!(&result_item.table_id, &expected_item.table_id);
        assert_eq!(&result_item.quantity, &2);

        repository.add_item(expected_item.clone()).await.unwrap();
        let result_item_updated = repository.get_item(id).await.unwrap().unwrap();
        assert_eq!(&result_item_updated.quantity, &4);

        truncate_table(repository.connection_pool).await;
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_get_item_for_table_and_get_all() {
        let repository = PgItemRepository::init_test().await;
        let table_id_1 = 1;
        let table_id_2 = 2;
        let item_1 = InsertItemDao::new("sushi".to_string(), table_id_1, 5, 1);
        let item_2 = InsertItemDao::new("onigiri".to_string(), table_id_1, 10, 1);
        let item_3 = InsertItemDao::new("onigiri".to_string(), table_id_2, 10, 1);
        let id_1 = repository.add_item(item_1).await.unwrap();
        let id_2 = repository.add_item(item_2).await.unwrap();
        let id_3 = repository.add_item(item_3).await.unwrap();
        let result_items_for_table = repository.get_items_for_table(table_id_1).await.unwrap();

        assert_eq!(result_items_for_table.len(), 2);
        result_items_for_table.iter().for_each(|item| {
            assert_eq!(&item.table_id, &table_id_1);
        });
        let result_item_1 = result_items_for_table.first().unwrap();
        assert_eq!(&result_item_1.id, &id_1);
        assert_eq!(&result_item_1.name, &"sushi".to_string());
        assert_eq!(&result_item_1.time_to_prepare, &5);
        let result_item_2 = result_items_for_table.last().unwrap();
        assert_eq!(&result_item_2.id, &id_2);
        assert_eq!(&result_item_2.table_id, &table_id_1);
        assert_eq!(&result_item_2.time_to_prepare, &10);

        let result_all = repository.get_all_items().await.unwrap();

        assert_eq!(result_all.len(), 3);
        let result_all_1 = result_all.first().unwrap();
        assert_eq!(result_all_1.id, id_1);
        let result_all_2 = result_all.get(1).unwrap();
        assert_eq!(result_all_2.id, id_2);
        let result_all_3 = result_all.last().unwrap();
        assert_eq!(result_all_3.id, id_3);

        truncate_table(repository.connection_pool).await;
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn test_remove_item() {
        let repository = PgItemRepository::init_test().await;
        let table_id = 1;
        let item_to_remove = InsertItemDao::new("sushi".to_string(), table_id, 5, 3);
        let item_to_stay = InsertItemDao::new("onigiri".to_string(), table_id, 10, 1);
        let id_to_remove = repository.add_item(item_to_remove).await.unwrap();
        let id_to_stay = repository.add_item(item_to_stay).await.unwrap();
        let result_all_before_remove = repository.get_all_items().await.unwrap();

        assert_eq!(result_all_before_remove.len(), 2);

        repository.remove_item(id_to_remove, 1).await.unwrap();
        let item_after_remove = repository.get_item(id_to_remove).await.unwrap().unwrap();
        assert_eq!(item_after_remove.quantity, 2);

        repository.remove_item(id_to_remove, 2).await.unwrap();
        let result_all_after_remove = repository.get_all_items().await.unwrap();
        assert_eq!(result_all_after_remove.len(), 1);
        assert_eq!(result_all_after_remove.first().unwrap().id, id_to_stay);

        truncate_table(repository.connection_pool).await;
    }
}
