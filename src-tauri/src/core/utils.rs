use tauri::Manager;
#[cfg(target_os = "linux")]
use std::{fs::metadata, path::PathBuf};
use std::process::Command;

#[tauri::command]
pub fn get_config_dir(app: &tauri::AppHandle) -> String {
    let config_dir = app.path().app_data_dir().unwrap_or(PathBuf::new());
    let config_dir_str = config_dir.to_string_lossy().into_owned();

    config_dir_str
}


#[tauri::command]
pub fn show_in_folder(path: String) {
  #[cfg(target_os = "windows")]
  {
    Command::new("explorer")
        .args(["/select,", &path]) // The comma after select is not a typo
        .spawn()
        .unwrap();
  }

  #[cfg(target_os = "linux")]
  {
      // see https://gitlab.freedesktop.org/dbus/dbus/-/issues/76
      let new_path = match metadata(&path).unwrap().is_dir() {
        true => path,
        false => {
          let mut path2 = PathBuf::from(path);
          path2.pop();
          path2.into_os_string().into_string().unwrap()
        }
      };
      Command::new("xdg-open")
          .arg(&new_path)
          .spawn()
          .unwrap();
  }

  #[cfg(target_os = "macos")]
  {
    Command::new("open")
        .args(["-R", &path])
        .spawn()
        .unwrap();
  }
}