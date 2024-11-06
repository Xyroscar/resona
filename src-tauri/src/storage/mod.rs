mod workspace;
mod collection;

pub use workspace::*;
pub use collection::*;

use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::RwLock;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Collection {
    pub id: String,
    pub workspace_id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Variable {
    pub id: String,
    pub workspace_id: String,
    pub name: String,
    pub value: String,
    pub description: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub id: String,
    pub collection_id: String,
    pub name: String,
    pub method: String,
    pub url: String,
    pub created_at: i64,
    pub updated_at: i64,
}

pub struct Storage {
    conn: RwLock<Connection>,
    data_dir: PathBuf,
}

impl Storage {
    pub fn new(app_dir: PathBuf) -> rusqlite::Result<Self> {
        let db_path = app_dir.join("api-client.db");
        let data_dir = app_dir.join("data");

        fs::create_dir_all(&data_dir).map_err(|e| rusqlite::Error::InvalidPath(data_dir.clone()))?;

        let conn = Connection::open(db_path)?;
        Self::init_database(&conn)?;

        Ok(Storage { conn: RwLock::new(conn), data_dir })
    }

    fn init_database(conn: &Connection) -> rusqlite::Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS workspaces (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS collections (
                id TEXT PRIMARY KEY,
                workspace_id TEXT NOT NULL,
                name TEXT NOT NULL,
                description TEXT,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                FOREIGN KEY(workspace_id) REFERENCES workspaces(id)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS variables (
                id TEXT PRIMARY KEY,
                workspace_id TEXT NOT NULL,
                name TEXT NOT NULL,
                value TEXT NOT NULL,
                description TEXT,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                FOREIGN KEY(workspace_id) REFERENCES workspaces(id)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS requests (
                id TEXT PRIMARY KEY,
                collection_id TEXT NOT NULL,
                name TEXT NOT NULL,
                method TEXT NOT NULL,
                url TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                FOREIGN KEY(collection_id) REFERENCES collections(id)
            )",
            [],
        )?;

        // Create indexes
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_collections_workspace 
             ON collections(workspace_id)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_variables_workspace 
             ON variables(workspace_id)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_requests_collection 
             ON requests(collection_id)",
            [],
        )?;

        Ok(())
    }

    pub(crate) fn now() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }

    pub(crate) fn get_request_data_path(&self, request_id: &str) -> PathBuf {
        self.data_dir.join(format!("request_{}.json", request_id))
    }
} 