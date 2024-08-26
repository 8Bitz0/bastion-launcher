import { invoke } from '@tauri-apps/api/core';

export interface Config {
  setupFinished: boolean;
  currentInstallPath: string;
  customInstallPaths: string[];
}

export function loadConfig(): Promise<Config> {
  return invoke('load_config');
}

export function getConfig(): Promise<Config> {
  return invoke('get_config');
}

export function setConfig(config: Config): Promise<void> {
  return invoke('set_config', { config: config });
}
