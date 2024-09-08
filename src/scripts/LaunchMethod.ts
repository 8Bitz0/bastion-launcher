import { invoke } from "@tauri-apps/api/core";

export enum LaunchMethod {
  Windows = "windows",
  WindowsLauncher = "windows-launcher",
}

export function getLaunchMethods(): Promise<LaunchMethod[]> {
  return invoke('get_launch_methods');
}

export function supportsGameArgs(method: LaunchMethod): boolean {
  switch (method.toString()) {
    case LaunchMethod.Windows:
      return true;
    default:
      return false;
  }
}
