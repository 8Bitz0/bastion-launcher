<script lang='ts'>
  import IconifyIcon from '@iconify/svelte';
  import { blur } from 'svelte/transition';

  import InstallIcon from './InstallIcon.svelte';
  import { InstallPathType } from '../scripts/InstallPath';

  let className = '';

  export let active: boolean;
  export let label: string;
  export let installType: InstallPathType;
  export let onClick: () => void = () => {};

  export { className as class };
</script>

{#if active}
  <div class='flex flex-row gap-2 w-full p-2 pl-3 items-center scale-[1.03] bg-slate-300 text-slate-950 rounded-lg'>
    <InstallIcon type={installType} class='h-full w-8' />
    <p class='font-bold flex-grow'>{label}</p>
    <IconifyIcon icon='material-symbols:check-rounded' class='w-5 h' />
  </div>
{:else}
  <button on:click={onClick} class='w-full text-left cursor-default {className}'>
    <div class='flex flex-row gap-2 w-full p-2 pl-3 items-center bg-slate-800 hover:bg-slate-700 hover:scale-[1.02] duration-100 hover:duration-100 active:scale-[0.98] active:ease-out rounded-lg'>
      <InstallIcon type={installType} class='h-full w-8' />
      <p class='font-bold flex-grow'>{label}</p>
    </div>
  </button>
{/if}
