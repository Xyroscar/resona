//! Database error types

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database error: {0}")]
    Database(#[from] redb::DatabaseError),

    #[error("Storage error: {0}")]
    Storage(#[from] redb::StorageError),

    #[error("Table error: {0}")]
    Table(#[from] redb::TableError),

    #[error("Transaction error: {0}")]
    Transaction(#[from] redb::TransactionError),

    #[error("Commit error: {0}")]
    Commit(#[from] redb::CommitError),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type DbResult<T> = Result<T, DbError>;

// Implement conversion to tauri::Error for command returns
impl From<DbError> for String {
    fn from(err: DbError) -> Self {
        err.to_string()
    }
}
