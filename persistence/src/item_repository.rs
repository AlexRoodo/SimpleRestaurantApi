use async_trait::async_trait;

use crate::{
    dao::{InsertItemDao, ItemDao},
    error::DbError,
};

#[async_trait]
pub trait ItemRepository {
    async fn add_item(&self, item: InsertItemDao) -> Result<i64, DbError>;
    async fn get_item(&self, item_id: i64) -> Result<Option<ItemDao>, DbError>;
    async fn get_items_for_table(&self, table_id: i32) -> Result<Vec<ItemDao>, DbError>;
    async fn get_all_items(&self) -> Result<Vec<ItemDao>, DbError>;
    async fn remove_item(&self, item_id: i64, quantity: i32) -> Result<(), DbError>;
}

impl dyn ItemRepository {
    pub async fn init_prod() -> Box<Self> {
        let pg_impl = crate::postgres_item_repository::PgItemRepository::init_prod().await;
        Box::new(pg_impl)
    }
}
