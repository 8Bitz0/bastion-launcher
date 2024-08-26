<script lang='ts'>
  // Prevents duplicate `Icon` component
  import IconifyIcon from '@iconify/svelte';
  
  import { getInstallPaths } from '../scripts/InstallPath';
  import InstallPathEntry from './InstallPathEntry.svelte';

  let selectedInstall: number | null = null;

  let installPaths = getInstallPaths().then(paths => {
    if (paths && paths.length > 0) {
      selectedInstall = 0 ;
  
      return paths;
    } else {
      return [];
    }
  });

  export let onUpdate: (currentInstall: string, custom: string[] ) => void;

  function switchPathChoice(path: string) {
    onUpdate(path, []);
  }
</script>

<h1 class='absolute text-3xl font-bold text-slate-300 -translate-y-12'>Choose Installation</h1>
<div class='w-[700px] h-full grid grid-cols-2 gap-8 mx-auto text-slate-300'>
  <!-- Contains large icon on the side -->
  <div class='h-full p-6'>
    <IconifyIcon icon='fluent:hard-drive-48-regular' class='mx-auto w-full h-full stroke-[0.2]' />
  </div>
  <div class='h-full flex flex-col overflow-x-clip overflow-y-auto p-2 gap-2 scrollbar scrollbar-track-transparent scrollbar-thumb-rounded-full scrollbar-thumb-slate-700 scrollbar-w-2'>
    {#await installPaths}
      <!-- Waiting -->
    {:then installPaths}
      {#each installPaths as path, i}
        <div class='w-full'>
          <InstallPathEntry
            active={(i == selectedInstall)}
            label={path.label || path.path}
            installType={path.type}
            onClick={() => switchPathChoice(path.path)}
          />
        </div>
      {/each}
      {#if installPaths.length == 0}
        <p class='text-center text-slate-500 font-bold'>No installations found</p>
      {/if}
      <hr class='border-0 h-[2px] border-slate-800 bg-gradient-to-r from-transparent via-slate-800 to-transparent'>
      <button class='text-left cursor-default'>
        <div class='flex flex-row gap-2 w-full p-2 pl-3 items-center text-slate-400 bg-slate-800 hover:bg-slate-700 hover:text-slate-300 hover:scale-[1.02] duration-100 hover:duration-100 active:scale-[0.98] active:ease-out rounded-lg'>
          <IconifyIcon icon='mdi:plus' class='h-full w-8' />
          <p class='font-bold flex-grow'>Add Install</p>
        </div>
      </button>
    {/await}
  </div>
</div>

