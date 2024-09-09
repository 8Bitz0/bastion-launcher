<script lang='ts'>
  import { type GameSettings } from '../scripts/Config';
  import { GameArgSupport, type LaunchMethod, supportsGameArgs } from '../scripts/LaunchMethod';
  import { getGfxApis, GfxApi } from '../scripts/GfxApi';
  import ToggleBox from "./ToggleBox.svelte";
  import CloseButton from "./CloseButton.svelte";
  import DropdownBox from './DropdownBox.svelte';

  let className: string = '';
  export let onClose: () => void = () => {};
  export let gameSettings: GameSettings;
  export let launchMethod: LaunchMethod;

  export { className as class };

  let rendererIndex: number = 0;
  let rendererEntries: { label: string }[] = [];
  let renderers: GfxApi[];

  $: gameSettingsSupport = supportsGameArgs(launchMethod);
  $: rendererEntries = getRenderers(launchMethod);
  $: getInitialRenderer(gameSettings);
  $: setRenderer(rendererIndex);

  function getInitialRenderer(gameSettings: GameSettings) {
    let api = gameSettings.gfxApi;

    let newIndex: number;

    if (api === GfxApi.Dx11) {
      newIndex = renderers.indexOf(GfxApi.Dx11) + 1;
    } else if (api === GfxApi.Vulkan) {
      newIndex = renderers.indexOf(GfxApi.Vulkan) + 1;
    } else {
      newIndex = 0;
    }

    if (newIndex === -1) {
      newIndex = 0;
    }

    rendererIndex = newIndex;
  }

  function getRenderers(launchMethod: LaunchMethod): { label: string }[] {
    renderers = getGfxApis(launchMethod);

    let newEntries: { label: string }[] = [];

    newEntries.push({ label: 'Default' });

    for (let api of renderers) {
      let label: string;

      if (api === GfxApi.Dx11) {
        label = 'DirectX 11'
      } else if (api === GfxApi.Vulkan) {
        label = 'Vulkan'
      } else {
        continue;
      }

      newEntries.push({ label: label });
    }

    return newEntries;
  }

  function setRenderer(index: number) {
    let api: GfxApi | undefined;

    console.log(`Setting renderer to index ${index} which is label ${rendererEntries[index].label} and renderer ${renderers[index - 1]}`);

    if (index === 0) {
      api = undefined;
    } else {
      // Subtract 1 to account for the "Default" entry
      api = renderers[index - 1];
    }

    console.log(`Renderer set to ${api}`);
    gameSettings.gfxApi = api;
  }

  function setConsole(state: boolean) {
    console.log(`Console set to ${state ? 'enabled' : 'disabled'}`);
    gameSettings.console = state;
  }
</script>

<div class='h-full w-72 bg-slate-800 text-slate-300 font-semibold border-l-[1px] border-slate-700 p-6 drop-shadow-[0_0_32px_rgb(0,0,0,0.3)] {className}'>
  <div class='flex w-full flex-col gap-2'>  
    <div class='flex w-full items-center'>
      <CloseButton onClick={onClose} class='min-w-10 w-10 h-10' />
      <h1 class='text-2xl font-bold text-center w-full'>Game Settings</h1>
    </div>
    {#if gameSettingsSupport !== GameArgSupport.False}
      <div class='w-full'>
        {#if gameSettingsSupport === GameArgSupport.True}
          <ToggleBox switchState={gameSettings.console} onSwitch={(state) => setConsole(state)} label='Console' />
        {/if}
        <DropdownBox bind:index={rendererIndex} label='Renderer' entries={rendererEntries} />
      </div>
    {:else}
      <p class='text-slate-500 text-sm text-center w-[90%] mx-auto'>Game settings not supported by this launch method</p>
    {/if}
  </div>
</div>
