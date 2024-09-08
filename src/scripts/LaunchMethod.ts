import { invoke } from "@tauri-apps/api/core";

export enum LaunchMethod {
  Windows = "windows",
  WindowsLauncher = "windows-launcher",
  Linux = "linux",
}

export enum GameArgSupport {
  True = "true",
  False = "false",
  NoConsole = "no-console",
}

export function getLaunchMethods(): Promise<LaunchMethod[]> {
  return invoke('get_launch_methods');
}

export function supportsGameArgs(method: LaunchMethod): GameArgSupport {
  switch (method) {
    case LaunchMethod.Windows:
      return GameArgSupport.True;
    case LaunchMethod.Linux:
      return GameArgSupport.NoConsole;
    default:
      return GameArgSupport.False;
  }
}
