<script lang='ts'>
  import { Icon, ArrowRightCircle, Square2Stack } from 'svelte-hero-icons';
  // Prevents duplicate `Icon` component
  import IconifyIcon from '@iconify/svelte';
  import { getInstallPaths } from './InstallPath';

  let nums = [1, 2];

  let install_paths = getInstallPaths();
</script>

<h1 class='absolute text-3xl font-bold text-slate-300 -translate-y-12'>Choose Installation</h1>
<div class='w-[700px] h-full grid grid-cols-2 gap-8 mx-auto text-slate-300'>
  <!-- Contains large icon on the side -->
  <div class='h-full p-6'>
    <IconifyIcon icon="fluent:hard-drive-48-regular" class='mx-auto w-full h-full stroke-[0.2]' />
  </div>
  <div class='h-full flex flex-col overflow-x-clip overflow-y-auto p-2 gap-1.5 scrollbar scrollbar-track-transparent scrollbar-thumb-rounded-full scrollbar-thumb-slate-700 scrollbar-w-2'>
    {#each nums as num}
      {#if num == 1}
      <div class='flex flex-row gap-2 w-full p-2 pl-3 items-center scale-[1.03] bg-slate-300 text-slate-950 rounded-lg'>
        <IconifyIcon icon='mdi:steam' class='h-full w-8' />
        <p class='font-bold flex-grow'>Steam</p>
        <IconifyIcon icon='material-symbols:check-rounded' class='w-5 h' />
      </div>
      {:else}
      <div class='flex flex-row gap-2 w-full p-2 pl-3 items-center bg-slate-800 hover:bg-slate-700 hover:scale-[1.02] duration-100 hover:duration-100 active:scale-[0.98] active:ease-out rounded-lg'>
        <IconifyIcon icon='mdi:steam' class='h-full w-8' />
        <p class='font-bold flex-grow'>Steam</p>
      </div>
      {/if}
    {/each}
    <div class='absolute z-10 h-full bg-gradient-to-b from-slate-800 to-transparent'></div>
  </div>
</div>
