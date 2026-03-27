use std::path::PathBuf;

use tauri::{AppHandle, Error, Manager, Runtime};

pub fn get_config_dir<R: Runtime>(app: &AppHandle<R>) -> Result<PathBuf, Error> {
    app.path().config_dir()
}
