import { invoke } from '@tauri-apps/api/core';

export async function launch(): Promise<void> {
  await invoke('launch');
}
