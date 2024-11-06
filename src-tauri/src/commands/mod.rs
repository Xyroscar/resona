mod workspace;
mod collection;

pub use workspace::*;
pub use collection::*;

pub fn handlers() -> impl Fn(tauri::ipc::Invoke) -> bool {
    tauri::generate_handler![
        create_workspace,
        get_workspaces,
        get_workspace,
        update_workspace,
        delete_workspace,
        create_collection,
        get_workspace_collections,
        update_collection,
        delete_collection
    ]
}