use serde::{Deserialize, Serialize};
use tauri_plugin_dialog::DialogExt;
use std::path::PathBuf;
use tauri::async_runtime::spawn_blocking;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum LaunchError {
  Exec(String),
}

fn spawn_beam(install: bastion::BeamNGInstall) -> Result<(), LaunchError> {
  bastion::exec(bastion::ExecMethod::Windows {
    install,
    args: bastion::CommonArgs {
      console: true,
      gfx_api: None,
    }
  }).map_err(|e| LaunchError::Exec(e.to_string()))
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

  let install = bastion::BeamNGInstall::init(path);

  let r = spawn_blocking(move || spawn_beam(install)).await;

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
