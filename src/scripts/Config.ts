import { invoke } from '@tauri-apps/api/core';

import { LaunchMethod } from './LaunchMethod';

export interface GameSettings {
  console: boolean;
  gfxApi?: string;
}

export interface Config {
  setupFinished: boolean;
  currentInstallPath?: string;
  customInstallPaths: string[];
  gameSettings: Record<string, GameSettings>;
  launchMethod: LaunchMethod;
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

export function getGameSettings(): Promise<GameSettings> {
  return invoke('get_game_settings');
}

export function setGameSettings(settings: GameSettings): Promise<void> {
  return invoke('set_game_settings', { settings: settings });
}

export function defaultGameSettings(): GameSettings {
  return {
    console: false,
    gfxApi: undefined,
  };
}
