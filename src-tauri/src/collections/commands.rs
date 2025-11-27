use tauri::State;

use crate::db::Database;

use super::service::CollectionService;
use super::types::{Collection, CreateCollectionInput, UpdateCollectionInput};

#[tauri::command]
pub fn get_collections(db: State<Database>) -> Result<Vec<Collection>, String> {
    let service = CollectionService::new(db.inner().clone());
    service.get_all().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_collection(db: State<Database>, id: String) -> Result<Collection, String> {
    let service = CollectionService::new(db.inner().clone());
    service.get(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_collections_by_workspace(
    db: State<Database>,
    workspace_id: String,
) -> Result<Vec<Collection>, String> {
    let service = CollectionService::new(db.inner().clone());
    service.get_by_workspace(&workspace_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_collection(
    db: State<Database>,
    input: CreateCollectionInput,
) -> Result<Collection, String> {
    let service = CollectionService::new(db.inner().clone());
    service.create(input).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_collection(
    db: State<Database>,
    input: UpdateCollectionInput,
) -> Result<Collection, String> {
    let service = CollectionService::new(db.inner().clone());
    service.update(input).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_collection(db: State<Database>, id: String) -> Result<(), String> {
    let service = CollectionService::new(db.inner().clone());
    service.delete(&id).map_err(|e| e.to_string())
}
