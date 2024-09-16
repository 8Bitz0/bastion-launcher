<script lang='ts'>
  import IconifyIcon from '@iconify/svelte';

  import InstallIcon from './InstallIcon.svelte';
  import { InstallPathType } from '../scripts/InstallPath';

  let className = '';

  export let active: boolean;
  export let label: string;
  export let removable: boolean = false;
  export let installType: InstallPathType;
  export let onClick: () => void = () => {};
  export let onRemove: () => void = () => {};

  export { className as class };
</script>

{#if active}
  <div class='flex flex-row gap-2 w-full p-2 pl-3 items-center scale-[1.03] bg-slate-300 text-slate-950 rounded-lg'>
    <InstallIcon type={installType} class='h-full w-8' />
    <p class='font-bold flex-grow text-nowrap overflow-hidden overflow-ellipsis'>{label}</p>
    {#if removable}
      <button on:click={onRemove} class='w-7 h-7 bg-white bg-opacity-0 rounded-md transition-all hover:bg-opacity-10 hover:transition-all cursor-default'>
        <IconifyIcon icon='bi:dash' class='w-full h-full' />
      </button>
    {:else}
      <IconifyIcon icon='material-symbols:check-rounded' class='w-5 h' />
    {/if}
  </div>
{:else}
  <button on:click={onClick} class='w-full text-left cursor-default {className}'>
    <div class='flex flex-row gap-2 w-full p-2 pl-3 items-center bg-slate-800 hover:bg-slate-700 hover:scale-[1.02] duration-100 hover:duration-100 active:scale-[0.98] active:ease-out rounded-lg'>
      <InstallIcon type={installType} class='h-full w-8' />
      <p class='font-bold flex-grow text-nowrap overflow-hidden overflow-ellipsis'>{label}</p>
      {#if removable}
        <button on:click={onRemove} class='w-7 min-w-7 h-7 min-h-7 bg-white bg-opacity-0 rounded-md transition-all hover:bg-opacity-10 hover:transition-all cursor-default'>
          <IconifyIcon icon='bi:dash' class='w-full h-full' />
        </button>
      {/if}
    </div>
  </button>
{/if}
