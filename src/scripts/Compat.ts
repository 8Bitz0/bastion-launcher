import { invoke } from "@tauri-apps/api/core";

export function useGPTK(): Promise<boolean> {
    return invoke('use_gptk');
}

export function selectGPTK(): Promise<string | null> {
    return invoke('select_gptk');
}
