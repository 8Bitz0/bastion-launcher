use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use crate::launch::{LaunchMethod, LaunchMethodInterface};

const CONFIG_DIR_NAME: &str = "Bastion Launcher";
const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
  #[error("Unable to determine config directory")]
  ConfigDir,
  #[error("Unable to create config directory ({0}): {1}")]
  Config(PathBuf, std::io::Error),
  #[error("Filesystem error ({0}): {1}")]
  Fs(PathBuf, std::io::Error),
  #[error("JSON deserialization error: {0}")]
  JsonDeserialize(serde_json::Error),
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct GameSettings {
  pub console: bool,
  #[serde(rename = "gfx-api")]
  pub gfx_api: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GameSettingsInterface {
  pub console: bool,
  #[serde(rename = "gfxApi")]
  pub gfx_api: Option<String>,
}

impl From<GameSettings> for GameSettingsInterface {
  fn from(value: GameSettings) -> Self {
    Self {
      console: value.console,
      gfx_api: value.gfx_api,
    }
  }
}

impl From<GameSettingsInterface> for GameSettings {
  fn from(value: GameSettingsInterface) -> Self {
    Self {
      console: value.console,
      gfx_api: value.gfx_api,
    }
  }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Config {
  #[serde(rename = "setup-finished")]
  pub setup_finished: bool,
  #[serde(rename = "install-paths")]
  pub custom_install_paths: Vec<PathBuf>,
  #[serde(rename = "current-install")]
  pub current_install: Option<PathBuf>,
  #[serde(rename = "game-settings")]
  pub game_settings: HashMap<PathBuf, GameSettings>,
  #[serde(rename = "launch-method")]
  pub launch_method: LaunchMethod,
}

/// Uses serde names identical to those in the frontend, otherwise
/// identical to `Config`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigInterface {
  #[serde(rename = "setupFinished")]
  pub setup_finished: bool,
  #[serde(rename = "customInstallPaths")]
  pub custom_install_paths: Vec<PathBuf>,
  #[serde(rename = "currentInstallPath")]
  pub current_install: Option<PathBuf>,
  #[serde(rename = "gameSettings")]
  pub game_settings: HashMap<PathBuf, GameSettingsInterface>,
  #[serde(rename = "launchMethod")]
  pub launch_method: LaunchMethodInterface,
}

impl From<Config> for ConfigInterface {
  fn from(value: Config) -> Self {
    Self {
      setup_finished: value.setup_finished,
      current_install: value.current_install,
      custom_install_paths: value.custom_install_paths,
      game_settings: value.game_settings.iter().map(|(k, v)| (k.clone(), v.clone().into())).collect(),
      launch_method: value.launch_method.into(),
    }
  }
}

impl From<ConfigInterface> for Config {
  fn from(value: ConfigInterface) -> Self {
    Self {
      setup_finished: value.setup_finished,
      current_install: value.current_install,
      custom_install_paths: value.custom_install_paths,
      game_settings: value.game_settings.iter().map(|(k, v)| (k.clone(), v.clone().into())).collect(),
      launch_method: value.launch_method.into(),
    }
  }
}

#[derive(Debug)]
pub struct ConfigManager {
  path: PathBuf,
  config: Config,
}

impl Default for ConfigManager {
  fn default() -> Self {
    Self {
      path: dirs::config_local_dir().unwrap().join(CONFIG_DIR_NAME),
      config: Config::default(),
    }
  }
}

impl ConfigManager {
  pub fn load(&mut self) -> Result<(), ConfigError> {
    let file_path = self.path.join(CONFIG_FILE_NAME);

    if file_path.is_file() {
      let mut file = File::open(file_path).map_err(|e| ConfigError::Fs(self.path.clone(), e))?;

      let mut json = String::new();
      file.read_to_string(&mut json).map_err(|e| ConfigError::Fs(self.path.clone(), e))?;

      self.config = serde_json::from_str(&json).map_err(ConfigError::JsonDeserialize)?;
    }

    Ok(())
  }

  pub fn get(&self) -> &Config {
    &self.config
  }
  
  pub fn set(&mut self, config: Config) {
    self.config = config;
  }

  pub fn update(&mut self) -> Result<(), ConfigError> {
    if !self.path.is_dir() {
      println!("Creating config directory: {:?}", self.path);

      std::fs::create_dir_all(&self.path).map_err(|e| ConfigError::Config(self.path.clone(), e))?;
    }

    println!("Writing config file: {:?}", self.path.join(CONFIG_FILE_NAME));

    let mut file = File::create(self.path.join(CONFIG_FILE_NAME)).map_err(|e| ConfigError::Fs(self.path.clone(), e))?;
    let json = serde_json::to_string(&self.config).map_err(ConfigError::JsonDeserialize)?;
    file.write_all(json.as_bytes()).map_err(|e| ConfigError::Fs(self.path.clone(), e))?;

    Ok(())
  }
}

#[tauri::command]
pub fn load_config(state: tauri::State<'_, crate::AppState>) -> Result<(), String> {
  let mut config_state = state.config.lock().unwrap();

  config_state.load().map_err(|e| e.to_string())?;

  Ok(())
}

#[tauri::command]
pub fn get_config(state: tauri::State<'_, crate::AppState>) -> ConfigInterface {
  // `unwrap` works here as an error is only returned if
  // another thread panicked while holding the lock
  state.config.lock().unwrap().get().clone().into()
}

#[tauri::command]
pub fn set_config(state: tauri::State<'_, crate::AppState>, config: ConfigInterface) -> Result<(), String> {
  let mut config_state = state.config.lock().unwrap();

  config_state.set(config.into());
  config_state.update().map_err(|e| e.to_string())?;

  Ok(())
}

#[tauri::command]
pub fn get_game_settings(state: tauri::State<'_, crate::AppState>) -> GameSettingsInterface {
  let config_lock = state.config.lock().unwrap();
  let config = config_lock.get();

  let current_install = match config.current_install.clone() {
    Some(p) => p,
    None => {
      return GameSettings::default().into();
    },
  };

  match config.game_settings.get(&current_install) {
    Some(settings) => settings.clone().into(),
    None => GameSettings::default().into(),
  }
}

#[tauri::command]
pub fn set_game_settings(state: tauri::State<'_, crate::AppState>, settings: GameSettingsInterface) -> Result<(), String> {
  let mut config_lock = state.config.lock().unwrap();
  let mut config = config_lock.get().clone();

  let current_install = match config.current_install.clone() {
    Some(p) => p,
    None => return Err("No install path is currently set".to_string()),
  };

  config.game_settings.insert(current_install, settings.into());
  config_lock.set(config);
  config_lock.update().map_err(|e| e.to_string())?;

  Ok(())
}
