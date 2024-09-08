<script lang='ts'>
  import { type GameSettings } from '../scripts/Config';
  import { type LaunchMethod, supportsGameArgs } from '../scripts/LaunchMethod';
  import ToggleBox from "./ToggleBox.svelte";
  import CloseButton from "./CloseButton.svelte";

  let className: string = '';
  export let onClose: () => void = () => {};
  export let gameSettings: GameSettings;
  export let launchMethod: LaunchMethod;

  export { className as class };

  $: gameSettingsAllowed = supportsGameArgs(launchMethod);

  async function setConsole(state: boolean) {
    console.log(`Console set to ${state ? 'enabled' : 'disabled'}`);
    gameSettings.console = state;
  }
</script>

<div class='h-full w-72 bg-slate-800 text-slate-300 font-semibold border-l-[1px] border-slate-700 p-6 drop-shadow-[0_0_32px_rgb(0,0,0,0.3)] {className}'>
  <div class='flex w-full flex-col gap-2'>
    <div class='flex w-full items-center'>
      <CloseButton onClick={onClose} class='min-w-10 h-10' />
      <h1 class='text-2xl font-bold text-center w-full'>Game Settings</h1>
    </div>
    {#if gameSettingsAllowed}
      <div class='w-full'>
        <ToggleBox switchState={gameSettings.console} onSwitch={(state) => setConsole(state)} label='Console' />
      </div>
    {:else}
      <p class='text-slate-500 text-sm text-center w-[90%] mx-auto'>Game settings not supported by this launch method</p>
    {/if}
  </div>
</div>
