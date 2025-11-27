use tauri::State;

use crate::db::Database;

use super::service::VariableService;
use super::types::{CreateVariableInput, ResolvedVariable, UpdateVariableInput, Variable};

#[tauri::command]
pub fn get_variable(db: State<Database>, id: String) -> Result<Variable, String> {
    let service = VariableService::new(db.inner().clone());
    service.get(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_global_variables(db: State<Database>) -> Result<Vec<Variable>, String> {
    let service = VariableService::new(db.inner().clone());
    service.get_global().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_workspace_variables(
    db: State<Database>,
    workspace_id: String,
) -> Result<Vec<Variable>, String> {
    let service = VariableService::new(db.inner().clone());
    service.get_by_workspace(&workspace_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_collection_variables(
    db: State<Database>,
    collection_id: String,
) -> Result<Vec<Variable>, String> {
    let service = VariableService::new(db.inner().clone());
    service.get_by_collection(&collection_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_request_variables(
    db: State<Database>,
    request_id: String,
) -> Result<Vec<Variable>, String> {
    let service = VariableService::new(db.inner().clone());
    service.get_by_request(&request_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_resolved_variables(
    db: State<Database>,
    workspace_id: Option<String>,
    collection_id: Option<String>,
    request_id: Option<String>,
) -> Result<Vec<ResolvedVariable>, String> {
    let service = VariableService::new(db.inner().clone());
    service
        .get_resolved(
            workspace_id.as_deref(),
            collection_id.as_deref(),
            request_id.as_deref(),
        )
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_variable(
    db: State<Database>,
    input: CreateVariableInput,
) -> Result<Variable, String> {
    let service = VariableService::new(db.inner().clone());
    service.create(input).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_variable(
    db: State<Database>,
    input: UpdateVariableInput,
) -> Result<Variable, String> {
    let service = VariableService::new(db.inner().clone());
    service.update(input).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_variable(db: State<Database>, id: String) -> Result<(), String> {
    let service = VariableService::new(db.inner().clone());
    service.delete(&id).map_err(|e| e.to_string())
}
