use derive_new::new;
use domain::item::Item;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AddItemRequest {
    pub name: String,
    pub table_id: i32,
    pub quantity: i32,
}

#[derive(Debug, Deserialize, Serialize, new)]
pub struct AddItemResponse {
    pub added_item_id: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetItemResponse {
    pub name: String,
    pub table_id: i32,
    pub time_to_prepare: i32,
    pub quantity: i32,
}

impl GetItemResponse {
    pub fn from_domain_item(item: Item) -> GetItemResponse {
        GetItemResponse {
            name: item.name,
            table_id: item.table_id,
            time_to_prepare: item.time_to_prepare,
            quantity: item.quantity,
        }
    }
}

fn map_domain_items_to_dto_items(items: Vec<Item>) -> Vec<GetItemResponse> {
    items
        .into_iter()
        .map(|item| GetItemResponse::from_domain_item(item))
        .collect()
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetItemForTableResponse {
    pub items: Vec<GetItemResponse>,
}

impl GetItemForTableResponse {
    pub fn from_domain_items(items: Vec<Item>) -> GetItemForTableResponse {
        GetItemForTableResponse {
            items: map_domain_items_to_dto_items(items),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetAllItemsResponse {
    pub items: Vec<GetItemResponse>,
}

impl GetAllItemsResponse {
    pub fn from_domain_items(items: Vec<Item>) -> GetAllItemsResponse {
        GetAllItemsResponse {
            items: map_domain_items_to_dto_items(items),
        }
    }
}
