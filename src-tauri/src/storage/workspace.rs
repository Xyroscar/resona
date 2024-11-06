use super::*;
use rusqlite::{params, Result, OptionalExtension};

impl Storage {
    pub fn create_workspace(&mut self, name: &str, description: Option<&str>) -> Result<Workspace> {
        let now = Self::now();
        let id = uuid::Uuid::new_v4().to_string();

        self.conn.get_mut().unwrap().execute(
            "INSERT INTO workspaces (id, name, description, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, name, description, now, now],
        )?;

        Ok(Workspace {
            id,
            name: name.to_string(),
            description: description.map(String::from),
            created_at: now,
            updated_at: now,
        })
    }

    pub fn get_workspaces(&mut self) -> Result<Vec<Workspace>> {
        let mut stmt = self.conn.get_mut().unwrap().prepare(
            "SELECT id, name, description, created_at, updated_at 
             FROM workspaces 
             ORDER BY created_at DESC"
        )?;

        let workspace_iter = stmt.query_map([], |row| {
            Ok(Workspace {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })?;

        workspace_iter.collect()
    }

    pub fn get_workspace(&mut self, id: &str) -> Result<Option<Workspace>> {
        let mut stmt = self.conn.get_mut().unwrap().prepare(
            "SELECT id, name, description, created_at, updated_at 
             FROM workspaces 
             WHERE id = ?"
        )?;

        stmt.query_row(params![id], |row| {
            Ok(Workspace {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })
        .optional()
    }

    pub fn update_workspace(&mut self, id: &str, name: &str, description: Option<&str>) -> Result<Workspace> {
        let now = Self::now();

        self.conn.get_mut().unwrap().execute(
            "UPDATE workspaces 
             SET name = ?1, description = ?2, updated_at = ?3
             WHERE id = ?4",
            params![name, description, now, id],
        )?;

        Ok(Workspace {
            id: id.to_string(),
            name: name.to_string(),
            description: description.map(String::from),
            created_at: self.get_workspace(id)?.map(|w| w.created_at).unwrap_or(now),
            updated_at: now,
        })
    }

    pub fn delete_workspace(&mut self, id: &str) -> Result<()> {
        // Start a transaction to ensure data consistency
        let tx = self.conn.get_mut().unwrap().transaction()?;

        // Delete variables
        tx.execute(
            "DELETE FROM variables WHERE workspace_id = ?",
            params![id],
        )?;

        // Delete requests from collections in this workspace
        tx.execute(
            "DELETE FROM requests 
             WHERE collection_id IN (
                SELECT id FROM collections WHERE workspace_id = ?
             )",
            params![id],
        )?;

        // Delete collections
        tx.execute(
            "DELETE FROM collections WHERE workspace_id = ?",
            params![id],
        )?;

        // Finally delete the workspace
        tx.execute("DELETE FROM workspaces WHERE id = ?", params![id])?;

        // Commit the transaction
        tx.commit()?;

        Ok(())
    }
} 