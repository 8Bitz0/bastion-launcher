// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

mod config;
mod paths;

#[derive(Default)]
pub struct AppState {
  config: Mutex<config::ConfigManager>,
}

fn main() {
  tauri::Builder::default()
    .manage(AppState::default())
    .invoke_handler(tauri::generate_handler![
      config::load_config,
      config::get_config,
      config::set_config,
      paths::install::get_install_paths,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
