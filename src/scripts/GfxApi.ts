import { LaunchMethod } from "./LaunchMethod";

export enum GfxApi {
  Dx11 = 'dx11',
  Vulkan = 'vk',
}

export function getGfxApis(method: LaunchMethod): GfxApi[] {
  switch (method) {
    case LaunchMethod.Windows:
      return [GfxApi.Dx11, GfxApi.Vulkan];
    case LaunchMethod.WindowsLauncher:
      return [];
    case LaunchMethod.Linux:
      return [GfxApi.Vulkan];
    default:
      return [];
  }
}
