use bastion::ExecMethod;
use serde::{Deserialize, Serialize};
use tauri_plugin_dialog::DialogExt;
use tauri::async_runtime::spawn_blocking;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum LaunchMethod {
  #[serde(rename = "windows")]
  Windows,
  #[serde(rename = "windows-launcher")]
  WindowsLauncher,
  #[serde(rename = "linux")]
  Linux,
  #[serde(rename = "mac-gptk")]
  MacGPTK,
  #[serde(rename = "mac-gptk-launcher")]
  MacGPTKLauncher,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum LaunchMethodInterface {
  #[serde(rename = "windows")]
  Windows,
  #[serde(rename = "windows-launcher")]
  WindowsLauncher,
  #[serde(rename = "linux")]
  Linux,
  #[serde(rename = "mac-gptk")]
  MacGPTK,
  #[serde(rename = "mac-gptk-launcher")]
  MacGPTKLauncher,
}

impl From<LaunchMethod> for LaunchMethodInterface {
  fn from(value: LaunchMethod) -> Self {
    match value {
      LaunchMethod::Windows => LaunchMethodInterface::Windows,
      LaunchMethod::WindowsLauncher => LaunchMethodInterface::WindowsLauncher,
      LaunchMethod::Linux => LaunchMethodInterface::Linux,
      LaunchMethod::MacGPTK => LaunchMethodInterface::MacGPTK,
      LaunchMethod::MacGPTKLauncher => LaunchMethodInterface::MacGPTKLauncher,
    }
  }
}

impl From<LaunchMethodInterface> for LaunchMethod {
  fn from(value: LaunchMethodInterface) -> Self {
    match value {
      LaunchMethodInterface::Windows => LaunchMethod::Windows,
      LaunchMethodInterface::WindowsLauncher => LaunchMethod::WindowsLauncher,
      LaunchMethodInterface::Linux => LaunchMethod::Linux,
      LaunchMethodInterface::MacGPTK => LaunchMethod::MacGPTK,
      LaunchMethodInterface::MacGPTKLauncher => LaunchMethod::MacGPTKLauncher,
    }
  }
}

impl Default for LaunchMethod {
  fn default() -> Self {
    Self::Windows
  }
}

impl Default for LaunchMethodInterface {
  fn default() -> Self {
    Self::Windows
  }
}

#[tauri::command]
pub fn get_launch_methods() -> Vec<LaunchMethod> {
  #[cfg(target_os = "windows")]
  {vec![LaunchMethod::Windows, LaunchMethod::WindowsLauncher]}
  #[cfg(target_os = "linux")]
  {vec![LaunchMethod::Linux]}
  #[cfg(target_os = "macos")]
  {vec![LaunchMethod::MacGPTK, LaunchMethod::MacGPTKLauncher]}
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum LaunchError {
  Exec(String),
}

fn spawn_beam(method: ExecMethod) -> Result<(), LaunchError> {
  bastion::exec(method).map_err(|e| LaunchError::Exec(e.to_string()))
}

#[tauri::command]
pub async fn launch(app: tauri::AppHandle, state: tauri::State<'_, crate::AppState>) -> Result<(), String> {
  println!("Launching BeamNG.drive");

  let path = match state.config.lock().unwrap().get().current_install.clone() {
    Some(p) => p,
    None => {
      app.dialog()
        .message("No install path is currently set")
        .title("Launch Error")
        .show(|_| {});
      return Err("No install path is currently set".to_string())
    },
  };

  let install = bastion::BeamNGInstall::init(&path);

  let game_settings_map = state.config.lock().unwrap().get().game_settings.clone();
  let compat_tool_path = state.config.lock().unwrap().get().compat_tool_path.clone();

  let game_settings = match game_settings_map.get(&path) {
    Some(s) => s.clone(),
    None => {
      app.dialog()
        .message("No game settings are currently set")
        .title("Launch Error")
        .blocking_show();
      return Err("No game settings are currently set".to_string())
    },
  };

  let method = match state.config.lock().unwrap().get().launch_method.clone() {
    LaunchMethod::Windows => ExecMethod::Windows {
      install,
      args: bastion::CommonArgs {
        console: game_settings.console,
        gfx_api: game_settings.gfx_api,
      },
    },
    LaunchMethod::WindowsLauncher => ExecMethod::WindowsIndirect {
      install,
    },
    LaunchMethod::Linux => ExecMethod::Linux {
      install,
      args: bastion::LinuxArgs {
        gfx_api: game_settings.gfx_api,
      },
    },
    LaunchMethod::MacGPTK => {
      let gptk_path = if let Some(p) = compat_tool_path {
        println!("Using GPTK path: {:?}", p);
        p
      } else {
        app.dialog()
          .message("No Game Porting Toolkit path is currently set")
          .title("Launch Error")
          .blocking_show();
        return Err("No Game Porting Toolkit path is currently set".to_string())
      };

      ExecMethod::MacGPTK {
        install,
        args: bastion::CommonArgs {
          console: game_settings.console,
          gfx_api: game_settings.gfx_api,
        },
        gptk_path,
      }
    },
    LaunchMethod::MacGPTKLauncher => {
      let gptk_path = if let Some(p) = compat_tool_path {
        p
      } else {
        app.dialog()
          .message("No Game Porting Toolkit path is currently set")
          .title("Launch Error")
          .blocking_show();
        return Err("No Game Porting Toolkit path is currently set".to_string())
      };

      ExecMethod::MacGPTKIndirect {
        install,
        gptk_path,
      }
    },
  };

  let r = spawn_blocking(move || spawn_beam(method)).await;

  match r {
    Ok(_) => (),
    Err(e) => {
      app.dialog()
        .message(e.to_string())
        .title("Game Crashed")
        .show(|_| {});
    },
  }

  Ok(())
}
