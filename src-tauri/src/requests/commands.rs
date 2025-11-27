use tauri::State;

use crate::db::Database;

use super::service::RequestService;
use super::types::{CreateRequestInput, Request, UpdateRequestInput};

#[tauri::command]
pub fn get_request(db: State<Database>, id: String) -> Result<Request, String> {
    let service = RequestService::new(db.inner().clone());
    service.get(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_requests_by_collection(
    db: State<Database>,
    collection_id: String,
) -> Result<Vec<Request>, String> {
    let service = RequestService::new(db.inner().clone());
    service.get_by_collection(&collection_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_standalone_requests_by_workspace(
    db: State<Database>,
    workspace_id: String,
) -> Result<Vec<Request>, String> {
    let service = RequestService::new(db.inner().clone());
    service.get_standalone_by_workspace(&workspace_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_all_requests_by_workspace(
    db: State<Database>,
    workspace_id: String,
) -> Result<Vec<Request>, String> {
    let service = RequestService::new(db.inner().clone());
    service.get_all_by_workspace(&workspace_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_request(
    db: State<Database>,
    input: CreateRequestInput,
) -> Result<Request, String> {
    let service = RequestService::new(db.inner().clone());
    service.create(input).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_request(
    db: State<Database>,
    input: UpdateRequestInput,
) -> Result<Request, String> {
    let service = RequestService::new(db.inner().clone());
    service.update(input).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_request(db: State<Database>, id: String) -> Result<(), String> {
    let service = RequestService::new(db.inner().clone());
    service.delete(&id).map_err(|e| e.to_string())
}
