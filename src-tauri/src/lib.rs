mod commands;
mod db;
mod models;

use commands::collections::*;
use commands::notes::*;
use commands::tags::*;
use tauri::Manager;

/// 桌面端显示主窗口
///
/// 应用启动时窗口默认隐藏（`visible: false`），
/// 前端加载完成后调用此命令显示窗口，避免白屏闪烁。
#[tauri::command]
fn release_window(app: tauri::AppHandle) {
    #[cfg(not(mobile))]
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let db_state = db::init_db(app).expect("failed to initialize database");
            app.manage(db_state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Notes
            create_note,
            get_notes,
            get_note,
            update_note,
            delete_note,
            // Tags
            create_tag,
            get_tags,
            update_tag,
            delete_tag,
            // Collections
            create_collection,
            get_collections,
            update_collection,
            delete_collection,
            query_collection,
            // Window Management
            release_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
