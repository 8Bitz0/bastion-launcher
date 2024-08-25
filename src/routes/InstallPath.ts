import { invoke } from '@tauri-apps/api/core';

export enum InstallPathType {
  Steam = 'Steam',
  SteamFlatpak = 'SteamFlatpak',
  Other = 'Other',
}

export interface InstallPath {
  path: string;
  exists: boolean;
  label?: string;
  path_type: InstallPathType;
}

export function getInstallPaths(): Promise<InstallPath[]> {
  return invoke('get_install_paths');
}
