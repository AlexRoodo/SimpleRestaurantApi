use crate::{postgres_item_repository::PgItemRepository, repositories::Repositories};

#[derive(Clone)]
pub struct PgRepositories {
    pub item_repository: PgItemRepository,
}

impl Repositories for PgRepositories {
    type ItemRepository = PgItemRepository;

    fn item_repository(&self) -> &Self::ItemRepository {
        &self.item_repository
    }
}

impl PgRepositories {
    pub async fn init_prod() -> PgRepositories {
        let item_repository = PgItemRepository::init_prod().await;
        PgRepositories { item_repository }
    }

    pub async fn init_test() -> PgRepositories {
        let item_repository = PgItemRepository::init_test().await;
        PgRepositories { item_repository }
    }
}
