mod core;

use tauri_plugin_tracing::{Builder, LevelFilter};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(
            Builder::new()
                .with_max_level(LevelFilter::DEBUG)
                .with_default_subscriber()
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            // Home
            core::home::get_projects,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
