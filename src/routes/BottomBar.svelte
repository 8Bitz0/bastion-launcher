<script lang='ts'>
  import IconifyIcon from '@iconify/svelte';

  import { type DropdownEntry } from './DropdownEntry';
  import { ButtonState } from './LaunchButtonState';
  import { LaunchMethod } from '../scripts/LaunchMethod';
  import BottomBarDropdown from './BottomBarDropdown.svelte';
  import LaunchButton from './LaunchButton.svelte';

  let className = '';
  export let currentState: ButtonState;
  export let launchMethods: LaunchMethod[];
  export let currentMethod: number | undefined;
  export let onLaunch: () => void = () => {};
  export let onCancel: () => void = () => {};
  export let onSettings: () => void = () => {};

  export { className as class };

  let methodDropdownEntries: DropdownEntry[] = [];

  function getMethodLabel(method: LaunchMethod): string {
    switch (method.toString()) {
      case LaunchMethod.Windows:
        return 'Windows';
      case LaunchMethod.WindowsLauncher:
        return 'Windows (Launcher)';
      default:
        return 'Unknown';
    }
  }

  function getMethodSubtitle(method: LaunchMethod): string | undefined {
    switch (method.toString()) {
      case LaunchMethod.Windows:
        return 'Launch the game directly';
      case LaunchMethod.WindowsLauncher:
        return 'Launch via the game\'s launcher';
      default:
        return undefined;
    }
  }

  $: for (let method of launchMethods) {
    let label = getMethodLabel(method);
    let subtitle = getMethodSubtitle(method);

    methodDropdownEntries = [...methodDropdownEntries, {
      label: label,
      subtitle: subtitle,
    }];
  }
</script>

<div class='w-full flex flex-row items-center p-4 gap-6 {className}'>
  <LaunchButton onStart={onLaunch} onCancel={onCancel} startLabel='LAUNCH' cancelLabel='RUNNING' state={currentState} />
  {#if currentMethod === undefined}
    <!-- Waiting -->
  {:else}
    <div class='w-52'>
      <BottomBarDropdown bind:index={currentMethod} entries={methodDropdownEntries} />
    </div>
  {/if}
  <div class='flex-grow' />
  <button on:click={onSettings} class='w-14 h-14 bg-slate-800 text-slate-300 rounded-lg duration-150 hover:bg-slate-300 hover:text-slate-800 hover:duration-150 active:scale-95 cursor-pointer'>
    <IconifyIcon icon='mdi:settings' class='h-full w-9 mx-auto' />
  </button>
</div>
