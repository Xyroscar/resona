// Resona - API Client Application

mod db;
mod workspaces;

use db::Database;

// Re-export workspace commands for generate_handler macro
use workspaces::{
    add_workspace_to_sync_group, create_sync_group, create_workspace, delete_sync_group,
    delete_workspace, get_sync_group, get_sync_group_for_workspace, get_sync_groups,
    get_workspace, get_workspaces, get_workspaces_by_sync_group, remove_workspace_from_sync_group,
    update_sync_group, update_workspace,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initializing the database
    let db = Database::open().expect("Failed to initialize database");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(db)
        .invoke_handler(tauri::generate_handler![
            // Workspace commands
            get_workspaces,
            get_workspace,
            create_workspace,
            update_workspace,
            delete_workspace,
            // Sync group commands
            get_sync_groups,
            get_sync_group,
            get_sync_group_for_workspace,
            create_sync_group,
            update_sync_group,
            delete_sync_group,
            get_workspaces_by_sync_group,
            add_workspace_to_sync_group,
            remove_workspace_from_sync_group,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
