use crate::storage::{Storage, Workspace};
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub async fn create_workspace(
    state: State<'_, Mutex<Storage>>,
    name: String,
    description: Option<String>,
) -> Result<Workspace, String> {
    let mut storage = state.lock().map_err(|e| e.to_string())?;
    storage
        .create_workspace(&name, description.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_workspaces(
    state: State<'_, Mutex<Storage>>,
) -> Result<Vec<Workspace>, String> {
    let mut storage = state.lock().map_err(|e| e.to_string())?;
    storage.get_workspaces().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_workspace(
    state: State<'_, Mutex<Storage>>,
    id: String,
) -> Result<Option<Workspace>, String> {
    let mut storage = state.lock().map_err(|e| e.to_string())?;
    storage.get_workspace(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_workspace(
    state: State<'_, Mutex<Storage>>,
    id: String,
    name: String,
    description: Option<String>,
) -> Result<Workspace, String> {
    let mut storage = state.lock().map_err(|e| e.to_string())?;
    storage
        .update_workspace(&id, &name, description.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_workspace(
    state: State<'_, Mutex<Storage>>,
    id: String,
) -> Result<(), String> {
    let mut storage = state.lock().map_err(|e| e.to_string())?;
    storage.delete_workspace(&id).map_err(|e| e.to_string())
} 
