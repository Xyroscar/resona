mod api;
mod models;
mod storage;
mod commands;

use storage::Storage;
use tauri::Manager;
use std::sync::Mutex;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_dir = app.path().app_data_dir()
                .expect("Failed to get app data dir");
            
            let storage = Storage::new(app_dir)
                .expect("Failed to initialize storage");
            
            app.manage(Mutex::new(storage));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::create_workspace,
            commands::get_workspaces,
            commands::get_workspace,
            commands::update_workspace,
            commands::delete_workspace,
            commands::create_collection,
            commands::get_workspace_collections,
            commands::update_collection,
            commands::delete_collection,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
