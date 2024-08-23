// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

mod paths;

#[derive(Default)]
pub struct AppState {
  installs: std::sync::Mutex<(Vec<paths::install::InstallPath>, u32)>,
}

fn main() {
  tauri::Builder::default()
    .manage(Mutex::new(AppState::default()))
    .invoke_handler(tauri::generate_handler![
      paths::install::get_install_paths
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
