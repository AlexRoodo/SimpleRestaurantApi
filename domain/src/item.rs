use derive_new::new;
use persistence::dao::{InsertItemDao, ItemDao};

#[derive(Debug, new)]
pub struct Item {
    pub name: String,
    pub table_id: i32,
    pub time_to_prepare: i32,
    pub quantity: i32,
}

impl Item {
    pub fn from_dao(item_dao: ItemDao) -> Item {
        Item {
            name: item_dao.name,
            table_id: item_dao.table_id,
            time_to_prepare: item_dao.time_to_prepare,
            quantity: item_dao.quantity,
        }
    }

    pub fn to_insert_dao(&self) -> InsertItemDao {
        InsertItemDao {
            name: self.name.clone(),
            table_id: self.table_id,
            time_to_prepare: self.time_to_prepare,
            quantity: self.quantity,
        }
    }
}
