// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

mod config;
mod gfx_api;
mod launch;
mod misc;
mod paths;

#[derive(Default)]
pub struct AppState {
  config: Mutex<config::ConfigManager>,
}

fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .manage(AppState::default())
    .invoke_handler(tauri::generate_handler![
      config::load_config,
      config::get_config,
      config::set_config,
      config::get_game_settings,
      config::set_game_settings,
      gfx_api::get_gfx_apis,
      launch::get_launch_methods,
      launch::launch,
      misc::use_gptk,
      misc::select_gptk,
      paths::install::get_install_paths,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
