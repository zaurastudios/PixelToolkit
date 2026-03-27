use std::path::PathBuf;

use tauri::{AppHandle, Manager, Runtime};

pub fn get_config_dir<R: Runtime>(app: &AppHandle<R>) -> PathBuf {
    app.path().config_dir().unwrap()
}
