use crate::launch::LaunchMethodInterface;

#[tauri::command]
pub fn get_gfx_apis(launch_method: LaunchMethodInterface) -> Vec<String> {
  match launch_method {
    LaunchMethodInterface::Windows => vec!["dx11".to_string(), "vulkan".to_string()],
    LaunchMethodInterface::WindowsLauncher => vec!["dx11".to_string(), "vulkan".to_string()],
  }
}
