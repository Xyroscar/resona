// Resona - API Client Application

mod collections;
mod db;
mod http;
mod requests;
mod settings;
mod variables;
mod workspaces;

use db::Database;
use http::{HttpRequest, HttpResponse};

use collections::{
    create_collection, delete_collection, get_collection, get_collections,
    get_collections_by_workspace, update_collection,
};
use requests::{
    create_request, delete_request, get_all_requests_by_workspace, get_request,
    get_requests_by_collection, get_standalone_requests_by_workspace, update_request,
};
use settings::{get_settings, reset_settings, update_settings};
use variables::{
    create_variable, delete_variable, get_collection_variables, get_global_variables,
    get_request_variables, get_resolved_variables, get_variable, get_workspace_variables,
    update_variable,
};
use workspaces::{
    add_workspace_to_sync_group, create_sync_group, create_workspace, delete_sync_group,
    delete_workspace, get_sync_group, get_sync_group_for_workspace, get_sync_groups,
    get_workspace, get_workspaces, get_workspaces_by_sync_group, remove_workspace_from_sync_group,
    update_sync_group, update_workspace,
};

#[tauri::command]
async fn send_http_request(request: HttpRequest) -> Result<HttpResponse, String> {
    http::execute_request(request).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
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
            // Collection commands
            get_collections,
            get_collection,
            get_collections_by_workspace,
            create_collection,
            update_collection,
            delete_collection,
            // Request commands
            get_request,
            get_requests_by_collection,
            get_standalone_requests_by_workspace,
            get_all_requests_by_workspace,
            create_request,
            update_request,
            delete_request,
            // Variable commands
            get_variable,
            get_global_variables,
            get_workspace_variables,
            get_collection_variables,
            get_request_variables,
            get_resolved_variables,
            create_variable,
            update_variable,
            delete_variable,
            // HTTP client
            send_http_request,
            // Settings commands
            get_settings,
            update_settings,
            reset_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
