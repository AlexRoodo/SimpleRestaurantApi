use crate::dto::*;
use crate::errors::ServerError;
use actix_web::web::{self, Json};
use actix_web::{delete, get, post, HttpResponse};
use domain::item::Item;

use persistence::item_repository::ItemRepository;
use persistence::postgres_repositories::PgRepositories;
use rand::Rng;

#[post("/item")]
pub async fn add_item(
    item: Json<AddItemRequest>,
    repositories: web::Data<PgRepositories>,
) -> Result<HttpResponse, ServerError> {
    let mut rnd = rand::thread_rng();
    let item = Item::new(
        item.name.clone(),
        item.table_id,
        rnd.gen_range(5..=15),
        item.quantity,
    );

    let result = repositories
        .item_repository
        .add_item(item.to_insert_dao())
        .await;

    match result {
        Ok(item_id) => Ok(HttpResponse::Ok().json(AddItemResponse::new(item_id))),
        Err(e) => Err(ServerError::from(e)),
    }
}

#[get("/item/{item_id}")]
pub async fn get_item(
    item_id: web::Path<i64>,
    repositories: web::Data<PgRepositories>,
) -> Result<HttpResponse, ServerError> {
    let result = repositories.item_repository.get_item(*item_id).await;

    match result {
        Ok(Some(item)) => {
            Ok(HttpResponse::Ok().json(GetItemResponse::from_domain_item(Item::from_dao(item))))
        }
        Ok(None) => Err(ServerError::NotFound),
        Err(e) => Err(ServerError::from(e)),
    }
}

#[get("/table/{table_id}")]
pub async fn get_items_for_table(
    table_id: web::Path<i32>,
    repositories: web::Data<PgRepositories>,
) -> Result<HttpResponse, ServerError> {
    let result = repositories
        .item_repository
        .get_items_for_table(*table_id)
        .await;
    match result {
        Ok(items) => {
            let items = items.into_iter().map(|item| Item::from_dao(item)).collect();
            Ok(HttpResponse::Ok().json(GetItemForTableResponse::from_domain_items(items)))
        }
        Err(e) => Err(ServerError::from(e)),
    }
}

#[get("/items")]
pub async fn get_all_items(
    repositories: web::Data<PgRepositories>,
) -> Result<HttpResponse, ServerError> {
    let result = repositories.item_repository.get_all_items().await;
    match result {
        Ok(items) => {
            let items = items.into_iter().map(|item| Item::from_dao(item)).collect();
            Ok(HttpResponse::Ok().json(GetAllItemsResponse::from_domain_items(items)))
        }
        Err(e) => Err(ServerError::from(e)),
    }
}

#[delete("/item/{item_id}/{quantity}")]
pub async fn remove_item(
    path: web::Path<(i64, i32)>,
    repositories: web::Data<PgRepositories>,
) -> Result<HttpResponse, ServerError> {
    let (item_id, quantity) = path.into_inner();
    let result = repositories
        .item_repository
        .remove_item(item_id, quantity)
        .await;
    match result {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) => Err(ServerError::from(e)),
    }
}

#[cfg(test)]
mod test {
    use crate::dto::GetItemForTableResponse;

    use super::*;
    use actix_web::{test, App};
    use persistence::{init_test_db, truncate_table};

    #[actix_web::test]
    #[serial_test::serial]
    async fn test_add_and_get_item() {
        let item_repository = init_test_db().await;
        let repositories = persistence::postgres_repositories::PgRepositories {
            item_repository: item_repository.clone(),
        };
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(repositories))
                .service(add_item)
                .service(get_item),
        )
        .await;
        let request_dto = AddItemRequest {
            name: "sushi".to_string(),
            table_id: 1,
            quantity: 1,
        };

        let request = test::TestRequest::post()
            .uri("/item")
            .set_json(request_dto)
            .to_request();
        let result = test::call_service(&app, request).await;
        assert_eq!(result.status(), 200);

        let add_item_response: AddItemResponse = test::read_body_json(result).await;
        assert_eq!(add_item_response.added_item_id, 1);

        let request = test::TestRequest::get().uri("/item/1").to_request();
        let result = test::call_service(&app, request).await;
        assert_eq!(result.status(), 200);

        let get_item_response: GetItemResponse = test::read_body_json(result).await;
        assert_eq!(get_item_response.name, "sushi");
        assert_eq!(get_item_response.table_id, 1);
        assert!(get_item_response.time_to_prepare >= 5 && get_item_response.time_to_prepare <= 15);
        assert_eq!(get_item_response.quantity, 1);

        truncate_table(item_repository.connection_pool).await;
    }

    #[actix_web::test]
    #[serial_test::serial]
    async fn test_get_item_for_table() {
        let item_repository = init_test_db().await;
        let repositories = persistence::postgres_repositories::PgRepositories {
            item_repository: item_repository.clone(),
        };
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(repositories))
                .service(get_items_for_table)
                .service(get_all_items),
        )
        .await;

        let item_1 = Item::new("sushi".to_string(), 1, 10, 1);
        let item_2 = Item::new("onigiri".to_string(), 1, 10, 3);
        let item_3 = Item::new("ramen".to_string(), 2, 10, 1);
        item_repository
            .add_item(item_1.to_insert_dao())
            .await
            .unwrap();
        item_repository
            .add_item(item_2.to_insert_dao())
            .await
            .unwrap();
        item_repository
            .add_item(item_3.to_insert_dao())
            .await
            .unwrap();

        let request_for_table = test::TestRequest::get().uri("/table/1").to_request();
        let result = test::call_service(&app, request_for_table).await;
        assert_eq!(result.status(), 200);

        let table_response: GetItemForTableResponse = test::read_body_json(result).await;
        assert_eq!(table_response.items.len(), 2);
        assert!(
            0 == table_response
                .items
                .iter()
                .filter(|item| item.name == item_3.name)
                .count()
        );

        let request_for_all_items = test::TestRequest::get().uri("/items").to_request();
        let result = test::call_service(&app, request_for_all_items).await;
        assert_eq!(result.status(), 200);

        let table_response: GetAllItemsResponse = test::read_body_json(result).await;
        assert_eq!(table_response.items.len(), 3);
    }

    #[actix_web::test]
    #[serial_test::serial]
    async fn test_remove_item() {
        let item_repository = init_test_db().await;
        let repositories = persistence::postgres_repositories::PgRepositories {
            item_repository: item_repository.clone(),
        };
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(repositories))
                .service(remove_item)
                .service(get_all_items),
        )
        .await;
        let item = Item::new("sushi".to_string(), 1, 10, 1);
        let item_id = item_repository
            .add_item(item.to_insert_dao())
            .await
            .unwrap();
        let request_for_remove = test::TestRequest::delete()
            .uri(format!("/item/{}", item_id).as_str())
            .to_request();
        let result = test::call_service(&app, request_for_remove).await;
        assert_eq!(result.status(), 200);

        let request_for_all_items = test::TestRequest::get().uri("/items").to_request();
        let result = test::call_service(&app, request_for_all_items).await;
        assert_eq!(result.status(), 200);

        let table_response: GetAllItemsResponse = test::read_body_json(result).await;
        assert_eq!(table_response.items.len(), 0);
    }
}
