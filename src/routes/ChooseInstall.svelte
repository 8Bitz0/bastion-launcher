<script lang='ts'>
  // Prevents duplicate `Icon` component
  import IconifyIcon from '@iconify/svelte';
  import { open as openDialog } from '@tauri-apps/plugin-dialog';
  
  import { getInstallPaths, InstallPathType } from '../scripts/InstallPath';
  import InstallPathEntry from './InstallPathEntry.svelte';

  let selectedInstall: string | null = null;
  let currentCustomInstall: string | null = null;

  $: if (selectedInstall === null) {
    changeReadyState(false);
  } else {
    changeReadyState(true);
  }

  let installPaths = getInstallPaths().then(paths => {
    if (paths && paths.length > 0) {
      selectedInstall = paths[0].path;

      // Ensures that the initial install choice is saved
      onUpdate(selectedInstall, []);
  
      return paths;
    } else {
      return [];
    }
  });

  export let onUpdate: (currentInstall: string, custom: string[] ) => void;
  export let changeReadyState: (ready: boolean) => void;

  function switchPathChoice(path: string) {
    console.log('Switching to', path);

    selectedInstall = path;

    let custom: string[] = [];

    if (currentCustomInstall) {
      custom = [currentCustomInstall]
    }

    onUpdate(path, custom);
  }

  async function addInstall() {
    let newInstall = await openDialog({
      directory: true,
      multiple: false,
    })

    if (newInstall !== null) {
      currentCustomInstall = newInstall;
      switchPathChoice(currentCustomInstall)
    }
  }

  function removeCustom() {
    selectedInstall = null;
    currentCustomInstall = null;
  }
</script>

<div class='w-[700px] h-full grid grid-cols-2 gap-8 mx-auto text-slate-300'>
  <!-- Contains large icon on the side -->
  <div class='h-full flex flex-col justify-center items-center -translate-y-8'>
    <h1 class='text-[28px] font-bold text-slate-300'>Installation</h1>
    <p class='text-lg'>Required to launch BeamNG.drive</p>
  </div>
  <div class='h-full flex flex-col overflow-x-clip overflow-y-auto p-2 gap-2 scrollbar scrollbar-track-transparent scrollbar-thumb-rounded-full scrollbar-thumb-slate-700 scrollbar-w-2'>
    {#await installPaths}
      <!-- Waiting -->
    {:then installPaths}
      {#each installPaths as path, i}
        <div class='w-full'>
          <InstallPathEntry
            active={(path.path == selectedInstall)}
            label={path.label || path.path}
            installType={path.type}
            removable={false}
            onClick={() => switchPathChoice(path.path)}
          />
        </div>
      {/each}
      {#if installPaths.length == 0}
        <p class='text-center text-slate-500 font-bold'>No installations found</p>
      {/if}
      <hr class='border-0 h-[2px] border-slate-800 bg-gradient-to-r from-slate-900 via-slate-800 to-slate-900'>
      {#if currentCustomInstall === null}
        <button on:click={addInstall} class='text-left cursor-default'>
          <div class='flex flex-row gap-2 w-full p-2 pl-3 items-center text-slate-400 bg-slate-800 hover:bg-slate-700 hover:text-slate-300 hover:scale-[1.02] duration-100 hover:duration-100 active:scale-[0.98] active:ease-out rounded-lg'>
            <IconifyIcon icon='mdi:plus' class='h-full w-8' />
            <p class='font-bold flex-grow'>Add Install</p>
          </div>
        </button>
      {:else}
        <InstallPathEntry
          active={(selectedInstall == currentCustomInstall)}
          label={currentCustomInstall}
          installType={InstallPathType.Other}
          removable={true}
          onClick={() => {if (currentCustomInstall) {switchPathChoice(currentCustomInstall)}}}
          onRemove={removeCustom}
        />
      {/if}
    {/await}
  </div>
</div>
