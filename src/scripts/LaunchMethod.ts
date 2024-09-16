import { invoke } from "@tauri-apps/api/core";

export enum LaunchMethod {
  Windows = "windows",
  WindowsLauncher = "windows-launcher",
  Linux = "linux",
  MacGPTK = "mac-gptk",
  MacGPTKLauncher = "mac-gptk-launcher",
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
    case LaunchMethod.MacGPTK:
      return GameArgSupport.NoConsole;
    default:
      return GameArgSupport.False;
  }
}
