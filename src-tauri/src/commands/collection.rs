use crate::storage::{Storage, Collection};
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub async fn create_collection(
    state: State<'_, Mutex<Storage>>,
    workspace_id: String,
    name: String,
    description: Option<String>,
) -> Result<Collection, String> {
    let mut storage = state.lock().map_err(|e| e.to_string())?;
    storage
        .create_collection(&workspace_id, &name, description.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_workspace_collections(
    state: State<'_, Mutex<Storage>>,
    workspace_id: String,
) -> Result<Vec<Collection>, String> {
    let mut storage = state.lock().map_err(|e| e.to_string())?;
    storage
        .get_workspace_collections(&workspace_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_collection(
    state: State<'_, Mutex<Storage>>,
    id: String,
    name: String,
    description: Option<String>,
) -> Result<Collection, String> {
    let mut storage = state.lock().map_err(|e| e.to_string())?;
    storage
        .update_collection(&id, &name, description.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_collection(
    state: State<'_, Mutex<Storage>>,
    id: String,
) -> Result<(), String> {
    let mut storage = state.lock().map_err(|e| e.to_string())?;
    storage.delete_collection(&id).map_err(|e| e.to_string())
} 
