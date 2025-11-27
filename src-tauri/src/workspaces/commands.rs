//! Tauri commands for operations on workspace

use tauri::State;

use crate::db::Database;

use super::types::{
    CreateSyncGroupInput, CreateWorkspaceInput, UpdateSyncGroupInput, UpdateWorkspaceInput,
    Workspace, WorkspaceSyncGroup,
};
use super::workspace::WorkspaceService;

/// Get all workspaces
#[tauri::command]
pub fn get_workspaces(db: State<Database>) -> Result<Vec<Workspace>, String> {
    let service = WorkspaceService::new(db.inner().clone());
    service.get_all().map_err(|e| e.to_string())
}

/// Get a workspace by ID
#[tauri::command]
pub fn get_workspace(db: State<Database>, id: String) -> Result<Workspace, String> {
    let service = WorkspaceService::new(db.inner().clone());
    service.get(&id).map_err(|e| e.to_string())
}

/// Create a new workspace
#[tauri::command]
pub fn create_workspace(
    db: State<Database>,
    input: CreateWorkspaceInput,
) -> Result<Workspace, String> {
    let service = WorkspaceService::new(db.inner().clone());
    service.create(input).map_err(|e| e.to_string())
}

/// Update an existing workspace
#[tauri::command]
pub fn update_workspace(
    db: State<Database>,
    input: UpdateWorkspaceInput,
) -> Result<Workspace, String> {
    let service = WorkspaceService::new(db.inner().clone());
    service.update(input).map_err(|e| e.to_string())
}

/// Delete a workspace
#[tauri::command]
pub fn delete_workspace(db: State<Database>, id: String) -> Result<(), String> {
    let service = WorkspaceService::new(db.inner().clone());
    service.delete(&id).map_err(|e| e.to_string())
}

/// Get all sync groups
#[tauri::command]
pub fn get_sync_groups(db: State<Database>) -> Result<Vec<WorkspaceSyncGroup>, String> {
    let service = WorkspaceService::new(db.inner().clone());
    service.get_all_sync_groups().map_err(|e| e.to_string())
}

/// Get a sync group by ID
#[tauri::command]
pub fn get_sync_group(db: State<Database>, id: String) -> Result<WorkspaceSyncGroup, String> {
    let service = WorkspaceService::new(db.inner().clone());
    service.get_sync_group(&id).map_err(|e| e.to_string())
}

/// Get sync group for a workspace
#[tauri::command]
pub fn get_sync_group_for_workspace(
    db: State<Database>,
    workspace_id: String,
) -> Result<Option<WorkspaceSyncGroup>, String> {
    let service = WorkspaceService::new(db.inner().clone());
    service
        .get_sync_group_for_workspace(&workspace_id)
        .map_err(|e| e.to_string())
}

/// Create a new sync group
#[tauri::command]
pub fn create_sync_group(
    db: State<Database>,
    input: CreateSyncGroupInput,
) -> Result<WorkspaceSyncGroup, String> {
    let service = WorkspaceService::new(db.inner().clone());
    service.create_sync_group(input).map_err(|e| e.to_string())
}

/// Update an existing sync group
#[tauri::command]
pub fn update_sync_group(
    db: State<Database>,
    input: UpdateSyncGroupInput,
) -> Result<WorkspaceSyncGroup, String> {
    let service = WorkspaceService::new(db.inner().clone());
    service.update_sync_group(input).map_err(|e| e.to_string())
}

/// Delete a sync group
#[tauri::command]
pub fn delete_sync_group(db: State<Database>, id: String) -> Result<(), String> {
    let service = WorkspaceService::new(db.inner().clone());
    service.delete_sync_group(&id).map_err(|e| e.to_string())
}

/// Get workspaces by sync group
#[tauri::command]
pub fn get_workspaces_by_sync_group(
    db: State<Database>,
    sync_group_id: String,
) -> Result<Vec<Workspace>, String> {
    let service = WorkspaceService::new(db.inner().clone());
    service
        .get_workspaces_by_sync_group(&sync_group_id)
        .map_err(|e| e.to_string())
}

/// Add a workspace to a sync group
#[tauri::command]
pub fn add_workspace_to_sync_group(
    db: State<Database>,
    sync_group_id: String,
    workspace_id: String,
) -> Result<(), String> {
    let service = WorkspaceService::new(db.inner().clone());
    service
        .add_workspace_to_sync_group(&sync_group_id, &workspace_id)
        .map_err(|e| e.to_string())
}

/// Remove a workspace from a sync group
#[tauri::command]
pub fn remove_workspace_from_sync_group(
    db: State<Database>,
    sync_group_id: String,
    workspace_id: String,
) -> Result<(), String> {
    let service = WorkspaceService::new(db.inner().clone());
    service
        .remove_workspace_from_sync_group(&sync_group_id, &workspace_id)
        .map_err(|e| e.to_string())
}
