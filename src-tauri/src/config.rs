use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::paths::install::InstallPath;

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

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Config {
  #[serde(rename = "setup-finished")]
  pub setup_finished: bool,
  #[serde(rename = "install-paths")]
  pub custom_install_paths: Vec<PathBuf>,
  #[serde(rename = "current-install")]
  pub current_install: Option<PathBuf>,
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
}

impl From<Config> for ConfigInterface {
  fn from(value: Config) -> Self {
    Self {
      setup_finished: value.setup_finished,
      current_install: value.current_install,
      custom_install_paths: value.custom_install_paths,
    }
  }
}

impl From<ConfigInterface> for Config {
  fn from(value: ConfigInterface) -> Self {
    Self {
      setup_finished: value.setup_finished,
      current_install: value.current_install,
      custom_install_paths: value.custom_install_paths,
    }
  }
}

#[derive(Debug)]
pub struct ConfigManager {
  path: PathBuf,
  config: Config,
}

impl std::default::Default for ConfigManager {
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
