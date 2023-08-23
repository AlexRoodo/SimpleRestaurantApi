# Simple Restaurant API

## Introduction
The project is developed for the [interview problem](https://github.com/paidy/interview/blob/master/SimpleRestaurantApi.md) of [Paidy Inc.](https://paidy.com/), in order to realize a simple food ordering system in a restaurant.
This is the client application for the project.

## Requirements for server
"Please try to spend around 4-6 hours on this test. The focus will be on data structure choice, API design and implementation, internal implementation"
**Running on a “server” and accepting calls from devices carried by restaurant staff to process guest’s menu orders. This is where the bulk of time should be spent.**
1. The application MUST, upon creation request, store the item, the table number, and how long the item will take to cook.
2. The application MUST, upon deletion request, remove a specified item for a specified table number.
3. The application MUST, upon query request, show all items for a specified table number.
4. The application MUST, upon query request, show a specified item for a specified table number.
5. The application MUST accept at least 10 simultaneous incoming add/remove/query requests.
6. The application MAY assign a length of time for the item to prepare as a random time between 5-15 minutes.
7. The application MAY keep the length of time for the item to prepare static (in other words, the time does not have to be counted down in real time, only upon item creation and then removed with the item upon item deletion).

Also:
- The time to prepare does not have to be kept up-to-date. It can also just be generated as some random amount of time between 5 and 15 minutes and kept static from then on.
- The table and items can be identified in any chosen manner, but it has to be consistent. So if a request comes in for table "4", for example, any other requests for table "4" must refer to the same table.
- The API is up to the developer. HTTP REST is acceptable, but direct API calls are also acceptable if they mimic an HTTP REST-like API (e.g. api_call1(string id, string resource), etc.).

## Architecture

### Data Storage
As the DB was chosen PostreSQL due to my familiarity with it. The DB has single table with following structure:
```sql
CREATE TABLE IF NOT EXISTS tbl_item (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    table_id INT NOT NULL,
    time_to_prepare INT NOT NULL,
    quantity INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX ON tbl_item(name, table_id);
```
That simple structure take advantage from requrements:
- The table as a distinct entity was never mentioned. Becase of that it can be simply column in the table.
- In the requirements was also never mentioned 'uniquness' of an single item. Because of that, identical items (severals order from same table) can be expresses as a single column `quantity`. This solution imposes only one restriction on the system: update of `quantity` must be atomic.
- Also for better performance was added index on `name` and `table_id` columns.
- As a better practice, the `created_at` column and comments to the table and columns were added to the table.
- The migration script is located in `./migrations` folder.

## Deployment
For better develeopment expirince and portability, the application is dockerized. The docker-compose is located in the root of the project.

## Structure
#### domain module
Module contains single domain structure `Item` with helper functions for Domain-Dao transitions.
#### persistence module
Contains definition of repository operations set and implementation of that operations for PostreSQL. As well as a Dao structure and Error type. Repository implementation was covered by unit tests.
#### server module
Contains an API call handlers function, structures for API call serialization and deserialization, Error type and a server initialization.

## Prerequisites

Rust and Cargo must be installed on the system
Docker

To build and run this application, apply theses commands in the project folder:
```
cargo build --release
docker-compose up -d
cargo run --bin server
```

## Exploration
To explore the API, you can use following commands:
1. Add item. Returns id of the item
```curl
curl --location 'localhost:8080/item' \
--header 'Content-Type: application/json' \
--data '{
    "name": "Sushi",
    "table_id": 1,
    "quantity": 4
}'
```
2. Get item by id. Returns item if it saved.
```curl
curl --location 'localhost:8080/item/{id}'
```
3. Get items by table id. Returns all items with specified table id.
```curl
curl --location 'localhost:8080/table/{table_id}'
```
4. Get all saved items. Returns all items.
```curl
curl --location 'localhost:8080/items'
```
5. Delete item by id. Reduces quantity of the item by specified quantity. If quantity is greater than current quantity, the item will be deleted.
```curl
curl --location --request DELETE 'localhost:8080/item/{item_id}/{quantity}'
```

## License

MIT
