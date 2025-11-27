use chrono::Utc;
use redb::ReadableTable;

use crate::db::{
    Database, DbError, DbResult, WORKSPACES, WORKSPACE_SYNC_GROUPS, WORKSPACES_BY_SYNC_GROUP,
};

use super::types::{
    CreateSyncGroupInput, CreateWorkspaceInput, UpdateSyncGroupInput, UpdateWorkspaceInput,
    Workspace, WorkspaceSyncGroup,
};

pub struct WorkspaceService {
    db: Database,
}

impl WorkspaceService {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub fn get_all(&self) -> DbResult<Vec<Workspace>> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(WORKSPACES)?;

        let mut workspaces = Vec::new();
        for entry in table.iter()? {
            let (_, value) = entry?;
            let workspace: Workspace = serde_json::from_str(value.value())
                .map_err(|e| DbError::Serialization(e.to_string()))?;
            workspaces.push(workspace);
        }

        workspaces.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(workspaces)
    }

    pub fn get(&self, id: &str) -> DbResult<Workspace> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(WORKSPACES)?;

        let value = table
            .get(id)?
            .ok_or_else(|| DbError::NotFound(format!("Workspace not found: {}", id)))?;

        let workspace: Workspace = serde_json::from_str(value.value())
            .map_err(|e| DbError::Serialization(e.to_string()))?;

        Ok(workspace)
    }

    pub fn create(&self, input: CreateWorkspaceInput) -> DbResult<Workspace> {
        let mut workspace = Workspace::new(input.name, input.description);
        workspace.tags = input.tags;

        let json = serde_json::to_string(&workspace)
            .map_err(|e| DbError::Serialization(e.to_string()))?;

        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(WORKSPACES)?;
            table.insert(workspace.id.as_str(), json.as_str())?;
        }
        write_txn.commit()?;

        Ok(workspace)
    }

    pub fn update(&self, input: UpdateWorkspaceInput) -> DbResult<Workspace> {
        let mut workspace = self.get(&input.id)?;

        if let Some(name) = input.name {
            workspace.name = name;
        }
        if let Some(description) = input.description {
            workspace.description = description;
        }
        if let Some(tags) = input.tags {
            workspace.tags = tags;
        }
        if let Some(sync_group_id) = input.sync_group_id {
            workspace.sync_group_id = Some(sync_group_id);
        }
        workspace.updated_at = Utc::now();

        // Write back
        let json = serde_json::to_string(&workspace)
            .map_err(|e| DbError::Serialization(e.to_string()))?;

        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(WORKSPACES)?;
            table.insert(workspace.id.as_str(), json.as_str())?;
        }
        write_txn.commit()?;

        Ok(workspace)
    }

    /// Delete a workspace
    pub fn delete(&self, id: &str) -> DbResult<()> {
        // First get the workspace to check sync_group_id
        let workspace = self.get(id)?;
        let sync_group_id = workspace.sync_group_id.clone();

        let write_txn = self.db.begin_write()?;

        // Remove from workspaces table
        {
            let mut table = write_txn.open_table(WORKSPACES)?;
            table.remove(id)?;
        }

        // Remove from sync group index if applicable
        if let Some(group_id) = sync_group_id {
            self.remove_from_sync_index(&write_txn, &group_id, id)?;
        }

        write_txn.commit()?;

        Ok(())
    }

    /// Helper to remove a workspace ID from the sync group index
    fn remove_from_sync_index(
        &self,
        write_txn: &redb::WriteTransaction,
        group_id: &str,
        workspace_id: &str,
    ) -> DbResult<()> {
        let mut idx_table = write_txn.open_table(WORKSPACES_BY_SYNC_GROUP)?;

        // Read current IDs
        let ids_json = match idx_table.get(group_id)? {
            Some(value) => value.value().to_string(),
            None => return Ok(()),
        };

        let mut ids: Vec<String> = serde_json::from_str(&ids_json)
            .map_err(|e| DbError::Serialization(e.to_string()))?;

        ids.retain(|i| i != workspace_id);

        let new_json = serde_json::to_string(&ids)
            .map_err(|e| DbError::Serialization(e.to_string()))?;

        idx_table.insert(group_id, new_json.as_str())?;

        Ok(())
    }

    /// Helper to add a workspace ID to the sync group index
    fn add_to_sync_index(
        &self,
        write_txn: &redb::WriteTransaction,
        group_id: &str,
        workspace_id: &str,
    ) -> DbResult<()> {
        let mut idx_table = write_txn.open_table(WORKSPACES_BY_SYNC_GROUP)?;

        // Read current IDs or start with empty
        let ids_json = match idx_table.get(group_id)? {
            Some(value) => value.value().to_string(),
            None => "[]".to_string(),
        };

        let mut ids: Vec<String> = serde_json::from_str(&ids_json)
            .map_err(|e| DbError::Serialization(e.to_string()))?;

        if !ids.contains(&workspace_id.to_string()) {
            ids.push(workspace_id.to_string());
        }

        let new_json = serde_json::to_string(&ids)
            .map_err(|e| DbError::Serialization(e.to_string()))?;

        idx_table.insert(group_id, new_json.as_str())?;

        Ok(())
    }

    // ==================== Sync Group Operations ====================

    /// Get all sync groups
    pub fn get_all_sync_groups(&self) -> DbResult<Vec<WorkspaceSyncGroup>> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(WORKSPACE_SYNC_GROUPS)?;

        let mut groups = Vec::new();
        for entry in table.iter()? {
            let (_, value) = entry?;
            let group: WorkspaceSyncGroup = serde_json::from_str(value.value())
                .map_err(|e| DbError::Serialization(e.to_string()))?;
            groups.push(group);
        }

        Ok(groups)
    }

    /// Get a sync group by ID
    pub fn get_sync_group(&self, id: &str) -> DbResult<WorkspaceSyncGroup> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(WORKSPACE_SYNC_GROUPS)?;

        let value = table
            .get(id)?
            .ok_or_else(|| DbError::NotFound(format!("Sync group not found: {}", id)))?;

        let group: WorkspaceSyncGroup = serde_json::from_str(value.value())
            .map_err(|e| DbError::Serialization(e.to_string()))?;

        Ok(group)
    }

    /// Get sync group for a workspace
    pub fn get_sync_group_for_workspace(
        &self,
        workspace_id: &str,
    ) -> DbResult<Option<WorkspaceSyncGroup>> {
        let workspace = self.get(workspace_id)?;

        match workspace.sync_group_id {
            Some(group_id) => Ok(Some(self.get_sync_group(&group_id)?)),
            None => Ok(None),
        }
    }

    /// Create a new sync group
    pub fn create_sync_group(&self, input: CreateSyncGroupInput) -> DbResult<WorkspaceSyncGroup> {
        let mut group = WorkspaceSyncGroup::new(input.name, input.workspace_ids.clone());
        group.synced_variable_names = input.synced_variable_names;
        group.sync_secrets = input.sync_secrets;

        let json = serde_json::to_string(&group)
            .map_err(|e| DbError::Serialization(e.to_string()))?;

        let write_txn = self.db.begin_write()?;

        // Insert sync group
        {
            let mut table = write_txn.open_table(WORKSPACE_SYNC_GROUPS)?;
            table.insert(group.id.as_str(), json.as_str())?;
        }

        // Update index
        {
            let mut idx_table = write_txn.open_table(WORKSPACES_BY_SYNC_GROUP)?;
            let ids_json = serde_json::to_string(&input.workspace_ids)
                .map_err(|e| DbError::Serialization(e.to_string()))?;
            idx_table.insert(group.id.as_str(), ids_json.as_str())?;
        }

        // Update each workspace's sync_group_id
        {
            let mut ws_table = write_txn.open_table(WORKSPACES)?;
            for ws_id in &input.workspace_ids {
                // Read workspace
                let ws_json = match ws_table.get(ws_id.as_str())? {
                    Some(value) => value.value().to_string(),
                    None => continue,
                };

                let mut workspace: Workspace = serde_json::from_str(&ws_json)
                    .map_err(|e| DbError::Serialization(e.to_string()))?;

                workspace.sync_group_id = Some(group.id.clone());
                workspace.updated_at = Utc::now();

                let new_ws_json = serde_json::to_string(&workspace)
                    .map_err(|e| DbError::Serialization(e.to_string()))?;

                ws_table.insert(ws_id.as_str(), new_ws_json.as_str())?;
            }
        }

        write_txn.commit()?;

        Ok(group)
    }

    /// Update a sync group
    pub fn update_sync_group(&self, input: UpdateSyncGroupInput) -> DbResult<WorkspaceSyncGroup> {
        // Read existing
        let mut group = self.get_sync_group(&input.id)?;

        // Apply updates
        if let Some(name) = input.name {
            group.name = name;
        }
        if let Some(synced_variable_names) = input.synced_variable_names {
            group.synced_variable_names = synced_variable_names;
        }
        if let Some(sync_secrets) = input.sync_secrets {
            group.sync_secrets = sync_secrets;
        }
        group.updated_at = Utc::now();

        // Write back
        let json = serde_json::to_string(&group)
            .map_err(|e| DbError::Serialization(e.to_string()))?;

        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(WORKSPACE_SYNC_GROUPS)?;
            table.insert(group.id.as_str(), json.as_str())?;
        }
        write_txn.commit()?;

        Ok(group)
    }

    /// Delete a sync group
    pub fn delete_sync_group(&self, id: &str) -> DbResult<()> {
        // Get the sync group to find associated workspaces
        let group = self.get_sync_group(id)?;
        let workspace_ids = group.workspace_ids.clone();

        let write_txn = self.db.begin_write()?;

        // Remove sync_group_id from all associated workspaces
        {
            let mut ws_table = write_txn.open_table(WORKSPACES)?;
            for ws_id in &workspace_ids {
                let ws_json = match ws_table.get(ws_id.as_str())? {
                    Some(value) => value.value().to_string(),
                    None => continue,
                };

                let mut workspace: Workspace = serde_json::from_str(&ws_json)
                    .map_err(|e| DbError::Serialization(e.to_string()))?;

                workspace.sync_group_id = None;
                workspace.updated_at = Utc::now();

                let new_ws_json = serde_json::to_string(&workspace)
                    .map_err(|e| DbError::Serialization(e.to_string()))?;

                ws_table.insert(ws_id.as_str(), new_ws_json.as_str())?;
            }
        }

        // Remove from sync groups table
        {
            let mut table = write_txn.open_table(WORKSPACE_SYNC_GROUPS)?;
            table.remove(id)?;
        }

        // Remove from index
        {
            let mut idx_table = write_txn.open_table(WORKSPACES_BY_SYNC_GROUP)?;
            idx_table.remove(id)?;
        }

        write_txn.commit()?;

        Ok(())
    }

    /// Get workspaces by sync group
    pub fn get_workspaces_by_sync_group(&self, sync_group_id: &str) -> DbResult<Vec<Workspace>> {
        let read_txn = self.db.begin_read()?;
        let idx_table = read_txn.open_table(WORKSPACES_BY_SYNC_GROUP)?;

        let workspace_ids: Vec<String> = match idx_table.get(sync_group_id)? {
            Some(value) => serde_json::from_str(value.value())
                .map_err(|e| DbError::Serialization(e.to_string()))?,
            None => return Ok(Vec::new()),
        };

        drop(idx_table);
        drop(read_txn);

        let mut workspaces = Vec::new();
        for ws_id in workspace_ids {
            if let Ok(workspace) = self.get(&ws_id) {
                workspaces.push(workspace);
            }
        }

        Ok(workspaces)
    }

    /// Add a workspace to a sync group
    pub fn add_workspace_to_sync_group(
        &self,
        sync_group_id: &str,
        workspace_id: &str,
    ) -> DbResult<()> {
        // Read existing data
        let mut group = self.get_sync_group(sync_group_id)?;
        let mut workspace = self.get(workspace_id)?;

        // Update in memory
        if !group.workspace_ids.contains(&workspace_id.to_string()) {
            group.workspace_ids.push(workspace_id.to_string());
            group.updated_at = Utc::now();
        }
        workspace.sync_group_id = Some(sync_group_id.to_string());
        workspace.updated_at = Utc::now();

        // Serialize
        let group_json = serde_json::to_string(&group)
            .map_err(|e| DbError::Serialization(e.to_string()))?;
        let ws_json = serde_json::to_string(&workspace)
            .map_err(|e| DbError::Serialization(e.to_string()))?;

        // Write all changes
        let write_txn = self.db.begin_write()?;

        {
            let mut table = write_txn.open_table(WORKSPACE_SYNC_GROUPS)?;
            table.insert(sync_group_id, group_json.as_str())?;
        }

        self.add_to_sync_index(&write_txn, sync_group_id, workspace_id)?;

        {
            let mut ws_table = write_txn.open_table(WORKSPACES)?;
            ws_table.insert(workspace_id, ws_json.as_str())?;
        }

        write_txn.commit()?;

        Ok(())
    }

    /// Remove a workspace from a sync group
    pub fn remove_workspace_from_sync_group(
        &self,
        sync_group_id: &str,
        workspace_id: &str,
    ) -> DbResult<()> {
        // Read existing data
        let mut group = self.get_sync_group(sync_group_id)?;
        let mut workspace = self.get(workspace_id)?;

        // Update in memory
        group.workspace_ids.retain(|id| id != workspace_id);
        group.updated_at = Utc::now();
        workspace.sync_group_id = None;
        workspace.updated_at = Utc::now();

        // Serialize
        let group_json = serde_json::to_string(&group)
            .map_err(|e| DbError::Serialization(e.to_string()))?;
        let ws_json = serde_json::to_string(&workspace)
            .map_err(|e| DbError::Serialization(e.to_string()))?;

        // Write all changes
        let write_txn = self.db.begin_write()?;

        {
            let mut table = write_txn.open_table(WORKSPACE_SYNC_GROUPS)?;
            table.insert(sync_group_id, group_json.as_str())?;
        }

        self.remove_from_sync_index(&write_txn, sync_group_id, workspace_id)?;

        {
            let mut ws_table = write_txn.open_table(WORKSPACES)?;
            ws_table.insert(workspace_id, ws_json.as_str())?;
        }

        write_txn.commit()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::temp_dir;

    fn create_test_db() -> Database {
        let path = temp_dir().join(format!("resona_test_{}.redb", uuid::Uuid::new_v4()));
        Database::open_at(path).expect("Failed to create test database")
    }

    #[test]
    fn test_workspace_crud() {
        let db = create_test_db();
        let service = WorkspaceService::new(db);

        let workspace = service
            .create(CreateWorkspaceInput {
                name: "Test Workspace".to_string(),
                description: "A test workspace".to_string(),
                tags: vec!["Development".to_string()],
            })
            .expect("Failed to create workspace");

        assert_eq!(workspace.name, "Test Workspace");
        assert_eq!(workspace.tags, vec!["Development".to_string()]);

        let fetched = service.get(&workspace.id).expect("Failed to get workspace");
        assert_eq!(fetched.id, workspace.id);

        let updated = service
            .update(UpdateWorkspaceInput {
                id: workspace.id.clone(),
                name: Some("Updated Workspace".to_string()),
                description: None,
                tags: None,
                sync_group_id: None,
            })
            .expect("Failed to update workspace");

        assert_eq!(updated.name, "Updated Workspace");
        assert_eq!(updated.description, "A test workspace");

        let all = service.get_all().expect("Failed to get all workspaces");
        assert_eq!(all.len(), 1);

        service
            .delete(&workspace.id)
            .expect("Failed to delete workspace");
        let all = service.get_all().expect("Failed to get all workspaces");
        assert_eq!(all.len(), 0);
    }

    #[test]
    fn test_sync_groups() {
        let db = create_test_db();
        let service = WorkspaceService::new(db);

        let ws1 = service
            .create(CreateWorkspaceInput {
                name: "Workspace 1".to_string(),
                description: "First workspace".to_string(),
                tags: vec!["Development".to_string()],
            })
            .expect("Failed to create workspace 1");

        let ws2 = service
            .create(CreateWorkspaceInput {
                name: "Workspace 2".to_string(),
                description: "Second workspace".to_string(),
                tags: vec!["Production".to_string()],
            })
            .expect("Failed to create workspace 2");

        // Create sync group
        let group = service
            .create_sync_group(CreateSyncGroupInput {
                name: "Test Sync Group".to_string(),
                workspace_ids: vec![ws1.id.clone(), ws2.id.clone()],
                synced_variable_names: vec!["API_KEY".to_string()],
                sync_secrets: false,
            })
            .expect("Failed to create sync group");

        // Verify workspaces are linked
        let ws1_updated = service.get(&ws1.id).expect("Failed to get workspace 1");
        assert_eq!(ws1_updated.sync_group_id, Some(group.id.clone()));

        // Get workspaces by sync group
        let grouped = service
            .get_workspaces_by_sync_group(&group.id)
            .expect("Failed to get workspaces by sync group");
        assert_eq!(grouped.len(), 2);

        // Delete sync group
        service
            .delete_sync_group(&group.id)
            .expect("Failed to delete sync group");

        // Verify workspaces are unlinked
        let ws1_final = service.get(&ws1.id).expect("Failed to get workspace 1");
        assert_eq!(ws1_final.sync_group_id, None);
    }
}
