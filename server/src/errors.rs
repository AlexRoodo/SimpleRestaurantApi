use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, From};
use persistence::error::DbError;

#[derive(Debug, Display, From)]
pub enum ServerError {
    NotFound,
    DbError(DbError),
}
impl std::error::Error for ServerError {}

impl ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServerError::NotFound => HttpResponse::NotFound().finish(),
            ServerError::DbError(DbError::MigrateError(e)) => {
                HttpResponse::InternalServerError().body(e.to_string())
            }
            ServerError::DbError(DbError::SqlxError(e)) => {
                HttpResponse::InternalServerError().body(e.to_string())
            }
        }
    }
}
