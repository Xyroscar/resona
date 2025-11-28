use tauri::State;

use crate::db::Database;

use super::service::SettingsService;
use super::types::{AppSettings, UpdateSettingsInput};

#[tauri::command]
pub fn get_settings(db: State<Database>) -> Result<AppSettings, String> {
    let service = SettingsService::new(db.inner().clone());
    service.get().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_settings(
    db: State<Database>,
    input: UpdateSettingsInput,
) -> Result<AppSettings, String> {
    let service = SettingsService::new(db.inner().clone());
    service.update(input).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn reset_settings(db: State<Database>) -> Result<AppSettings, String> {
    let service = SettingsService::new(db.inner().clone());
    service.reset().map_err(|e| e.to_string())
}
