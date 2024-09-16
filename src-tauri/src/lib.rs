mod core;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::LogDir {
                        file_name: Some("logs".to_string()),
                    },
                ))
                .build(),
        )
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![])
        .invoke_handler(tauri::generate_handler![
            // utils
            core::utils::show_in_folder,
            // home
            core::home::get_projects,
            core::home::remove_project,
            core::home::create_project,
            core::home::create_project_existing,
            // project
            core::project::get_dirs,
            // interface
            core::interface::select_texture,
            core::interface::select_texture_file,
            core::interface::update_defaults_grayscale,
        ])
        .setup(|_app| {
            #[cfg(desktop)]
            {}
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
