//! Database initialization and management

use std::path::PathBuf;
use std::sync::Arc;

use directories::ProjectDirs;
use redb::{Database as RedbDatabase, ReadableDatabase};

use super::error::{DbError, DbResult};
use super::tables::*;

/// Main database wrapper
pub struct Database {
    db: Arc<RedbDatabase>,
}

impl Database {
    /// Create or open the database at the default application data directory
    pub fn open() -> DbResult<Self> {
        let path = Self::get_db_path()?;

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let db = RedbDatabase::create(&path)?;
        let database = Self { db: Arc::new(db) };

        // Initialize tables
        database.init_tables()?;

        Ok(database)
    }

    /// Open database at a specific path (useful for testing)
    #[allow(dead_code)]
    pub fn open_at(path: PathBuf) -> DbResult<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let db = RedbDatabase::create(&path)?;
        let database = Self { db: Arc::new(db) };
        database.init_tables()?;

        Ok(database)
    }

    /// Get the default database path
    fn get_db_path() -> DbResult<PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "xyroscar", "resona")
            .ok_or_else(|| DbError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not determine application data directory",
            )))?;

        Ok(proj_dirs.data_dir().join("resona.redb"))
    }

    /// Initialize all tables
    fn init_tables(&self) -> DbResult<()> {
        let write_txn = self.db.begin_write()?;

        // Create main tables
        write_txn.open_table(WORKSPACES)?;
        write_txn.open_table(WORKSPACE_SYNC_GROUPS)?;
        write_txn.open_table(COLLECTIONS)?;
        write_txn.open_table(REQUESTS)?;
        write_txn.open_table(VARIABLES)?;
        write_txn.open_table(APP_SETTINGS)?;

        // Create index tables
        write_txn.open_table(COLLECTIONS_BY_WORKSPACE)?;
        write_txn.open_table(REQUESTS_BY_COLLECTION)?;
        write_txn.open_table(REQUESTS_BY_WORKSPACE)?;
        write_txn.open_table(VARIABLES_BY_SCOPE)?;
        write_txn.open_table(WORKSPACES_BY_SYNC_GROUP)?;

        write_txn.commit()?;

        Ok(())
    }

    /// Get a reference to the underlying redb database
    #[allow(dead_code)]
    pub fn inner(&self) -> &RedbDatabase {
        &self.db
    }

    /// Begin a read transaction
    pub fn begin_read(&self) -> DbResult<redb::ReadTransaction> {
        Ok(self.db.begin_read()?)
    }

    /// Begin a write transaction
    pub fn begin_write(&self) -> DbResult<redb::WriteTransaction> {
        Ok(self.db.begin_write()?)
    }
}

impl Clone for Database {
    fn clone(&self) -> Self {
        Self {
            db: Arc::clone(&self.db),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::temp_dir;

    #[test]
    fn test_database_creation() {
        let path = temp_dir().join("resona_test.redb");
        let _ = std::fs::remove_file(&path); // Clean up any previous test

        let db = Database::open_at(path.clone()).expect("Failed to create database");

        // Verify tables exist by attempting to read from them
        let read_txn = db.begin_read().expect("Failed to begin read transaction");
        let _ = read_txn.open_table(WORKSPACES).expect("Workspaces table should exist");

        // Clean up
        drop(db);
        let _ = std::fs::remove_file(&path);
    }
}
