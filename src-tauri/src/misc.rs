use tauri_plugin_dialog::DialogExt;

#[tauri::command]
pub fn use_gptk() -> bool {
  cfg!(target_os = "macos")
}

#[tauri::command]
pub async fn select_gptk(app: tauri::AppHandle) -> Option<std::path::PathBuf> {
  let chosen_file = app.dialog()
    .file()
    .add_filter("Game Porting Toolkit", &["app"])
    .blocking_pick_file();

  if let Some(file) = chosen_file {
    Some(file.path)
  } else {
    None
  }
}
