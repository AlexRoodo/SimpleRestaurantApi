mod dto;
mod errors;
mod handlers;

use actix_web::{middleware, web, App, HttpServer};
use persistence::postgres_repositories::PgRepositories;
use server::handlers::*;
use std::io::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let repositories = PgRepositories::init_prod().await;

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(repositories.clone()))
            .wrap(middleware::Logger::default())
            .service(add_item)
            .service(get_item)
            .service(get_items_for_table)
            .service(get_all_items)
            .service(remove_item)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
