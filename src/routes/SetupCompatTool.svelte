<script lang='ts'>
  // Prevents duplicate `Icon` component
  import IconifyIcon from '@iconify/svelte';

  import { selectGPTK } from '../scripts/Compat';

  let currentCompatPath: string | null = null;

  $: if (currentCompatPath === null) {
    changeReadyState(false);
  } else {
    changeReadyState(true);
  }

  export let onUpdate: (compatToolPath: string) => void;
  export let changeReadyState: (ready: boolean) => void;

  function switchPathChoice(path: string) {
    console.log('Switching to', path);

    onUpdate(path);
  }

  async function selectCompatTool() {
    let newInstall = await selectGPTK();

    if (newInstall !== null) {
      currentCompatPath = newInstall;
      switchPathChoice(currentCompatPath);
    }
  }
</script>

<div class='w-[700px] h-full grid grid-cols-2 gap-8 mx-auto text-slate-300'>
  <!-- Contains large icon on the side -->
  <div class='h-full flex flex-col justify-center items-center -translate-y-8'>
    <h1 class='text-[28px] font-bold text-slate-300'>Compatibility Tool</h1>
    <p class='text-md text-center'>BeamNG.drive does not natively support macOS. Please select a copy of Apple's Game Porting Toolkit.</p>
  </div>
  <div class='h-full flex flex-col justify-center p-2 -translate-y-8'>
    <div class='flex flex-row w-full'>
      <div class='flex flex-row w-[90%] gap-2 p-2 pl-3 items-center bg-slate-800 rounded-l-lg'>
        {#if currentCompatPath}
          <p class='font-bold flex-grow text-nowrap overflow-hidden overflow-ellipsis'>{currentCompatPath}</p>
        {:else}
          <p class='font-bold flex-grow text-slate-400 text-nowrap overflow-hidden overflow-ellipsis'>Select an app...</p>
        {/if}
      </div>
      <button on:click={selectCompatTool} class='flex h-full min-w-11 w-11 items-center justify-center bg-slate-800 border-l-2 border-slate-700 rounded-r-lg transition-all hover:bg-slate-300 hover:text-slate-800 hover:transition-all cursor-default'>
        <IconifyIcon icon='mdi:folder-outline' class='w-6 h-6' />
      </button>
    </div>
  </div>
</div>
