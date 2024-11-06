mod models;
mod storage;
mod commands;
mod client;

use commands::handlers as h;
use storage::Storage;
use tauri::Manager;
use std::sync::Mutex;

fn main() {
    let handlers = h();
    tauri::Builder::default()
        .setup(|app| {
            let app_dir = app.path().app_data_dir()
                .expect("Failed to get app data dir");
            
            let storage = Storage::new(app_dir)
                .expect("Failed to initialize storage");
            
            app.manage(Mutex::new(storage));
            Ok(())
        })
        .invoke_handler(handlers)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}