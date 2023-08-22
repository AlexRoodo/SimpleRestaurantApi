use crate::item_repository::ItemRepository;

pub trait Repositories {
    type ItemRepository: ItemRepository;
    fn item_repository(&self) -> &Self::ItemRepository;
}
