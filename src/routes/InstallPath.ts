import { invoke } from '@tauri-apps/api/core';

export function getInstallPaths() {
  invoke('get_install_paths').then((paths) => {
    console.log(paths);
  });
}
