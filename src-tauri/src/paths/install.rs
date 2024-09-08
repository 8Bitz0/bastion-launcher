use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::State;

use crate::AppState;

#[cfg(target_os = "windows")]
const WIN_STEAM_PATH: &str = "C:\\Program Files (x86)\\Steam\\steamapps\\common\\BeamNG.drive";
#[cfg(target_os = "linux")]
const LINUX_STEAM_PATH: &str = "$HOME/.steam/steam/steamapps/common/BeamNG.drive";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallPath {
  pub path: PathBuf,
  pub exists: bool,
  pub label: Option<String>,
  #[serde(rename = "type")]
  pub path_type: InstallPathType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstallPathType {
  Steam,
  SteamFlatpak,
  Other,
}

#[cfg(target_os = "linux")]
fn replace_home(path: &str) -> String {
  path.replace("$HOME", std::env::var("HOME").unwrap().as_str())
}

fn default_install_paths() -> Vec<InstallPath> {
  let mut paths = vec![];

  #[cfg(target_os = "windows")]
  {
    paths.push(InstallPath {
      path: PathBuf::from(WIN_STEAM_PATH),
      exists: PathBuf::from(WIN_STEAM_PATH).is_dir(),
      label: Some("Steam".to_string()),
      path_type: InstallPathType::Steam,
    });
  }
  #[cfg(target_os = "linux")]
  {
    paths.push(InstallPath {
      path: PathBuf::from(replace_home(LINUX_STEAM_PATH)),
      exists: PathBuf::from(replace_home(LINUX_STEAM_PATH)).is_dir(),
      label: Some("Steam".to_string()),
      path_type: InstallPathType::Steam,
    });
  }

  paths
}

pub fn setup_install_paths(custom_paths: Option<Vec<PathBuf>>) -> Vec<InstallPath> {
  let mut paths = default_install_paths();
  if let Some(custom_paths) = custom_paths {
    paths.extend(custom_paths.into_iter().map(|path| InstallPath {
      path: path.clone(),
      exists: path.is_dir(),
      label: None,
      path_type: InstallPathType::Other,
    }).collect::<Vec<InstallPath>>());
  }
  
  paths.into_iter().filter(|path| path.path.is_dir()).collect()
}

#[tauri::command]
pub fn get_install_paths(state: State<'_, AppState>) -> Vec<InstallPath> {
  let custom = state.config.lock().unwrap().get().custom_install_paths.clone();

  setup_install_paths(Some(custom))
}
