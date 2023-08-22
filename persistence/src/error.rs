use derive_more::Display;
use sqlx::migrate::MigrateError;
use sqlx::Error;

#[derive(Debug, Display)]
pub enum DbError {
    MigrateError(MigrateError),
    SqlxError(Error),
}
impl std::error::Error for DbError {}

impl DbError {
    pub fn from_sqlx_error(error: Error) -> DbError {
        DbError::SqlxError(error)
    }

    pub fn from_migrate_error(error: MigrateError) -> DbError {
        DbError::MigrateError(error)
    }
}
