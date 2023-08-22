use chrono::{NaiveDate, NaiveTime};
use derive_new::new;
use sqlx::FromRow;

#[derive(new, FromRow, Debug, Clone)]
pub struct ItemDao {
    pub id: i64,
    pub name: String,
    pub table_id: i32,
    pub time_to_prepare: i32,
    pub quantity: i32,
    pub created_at: chrono::NaiveDateTime,
}

impl ItemDao {
    pub fn test() -> Self {
        ItemDao::new(
            1,
            "sushi".to_string(),
            1,
            10,
            1,
            chrono::NaiveDateTime::new(NaiveDate::MIN, NaiveTime::MIN),
        )
    }
}

#[derive(new, FromRow, Debug, Clone)]
pub struct InsertItemDao {
    pub name: String,
    pub table_id: i32,
    pub time_to_prepare: i32,
    pub quantity: i32,
}
