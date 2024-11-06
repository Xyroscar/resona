use super::*;
use rusqlite::{params, Result};

impl Storage {
    pub fn create_collection(
        &mut self,
        workspace_id: &str,
        name: &str,
        description: Option<&str>,
    ) -> Result<Collection> {
        let now = Self::now();
        let id = uuid::Uuid::new_v4().to_string();

        self.conn.get_mut().unwrap().execute(
            "INSERT INTO collections (id, workspace_id, name, description, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![id, workspace_id, name, description, now, now],
        )?;

        Ok(Collection {
            id,
            workspace_id: workspace_id.to_string(),
            name: name.to_string(),
            description: description.map(String::from),
            created_at: now,
            updated_at: now,
        })
    }

    pub fn get_workspace_collections(&mut self, workspace_id: &str) -> Result<Vec<Collection>> {
        let mut stmt = self.conn.get_mut().unwrap().prepare(
            "SELECT id, workspace_id, name, description, created_at, updated_at 
             FROM collections 
             WHERE workspace_id = ?
             ORDER BY created_at DESC"
        )?;

        let collections = stmt.query_map([workspace_id], |row| {
            Ok(Collection {
                id: row.get(0)?,
                workspace_id: row.get(1)?,
                name: row.get(2)?,
                description: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })?;

        collections.collect()
    }

    pub fn update_collection(
        &mut self,
        id: &str,
        name: &str,
        description: Option<&str>,
    ) -> Result<Collection> {
        let now = Self::now();

        self.conn.get_mut().unwrap().execute(
            "UPDATE collections 
             SET name = ?1, description = ?2, updated_at = ?3
             WHERE id = ?4",
            params![name, description, now, id],
        )?;

        let mut stmt = self.conn.get_mut().unwrap().prepare(
            "SELECT workspace_id, created_at FROM collections WHERE id = ?"
        )?;

        let (workspace_id, created_at) = stmt.query_row([id], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
        })?;

        Ok(Collection {
            id: id.to_string(),
            workspace_id,
            name: name.to_string(),
            description: description.map(String::from),
            created_at,
            updated_at: now,
        })
    }

    pub fn delete_collection(&mut self, id: &str) -> Result<()> {
        let tx = self.conn.get_mut().unwrap().transaction()?;

        // Delete all requests in this collection
        tx.execute(
            "DELETE FROM requests WHERE collection_id = ?",
            params![id],
        )?;

        // Delete the collection
        tx.execute(
            "DELETE FROM collections WHERE id = ?",
            params![id],
        )?;

        tx.commit()?;
        Ok(())
    }
} 