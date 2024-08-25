<script lang='ts'>
  import { Icon, type IconSource } from 'svelte-hero-icons';

  let className: string = '';
  export let active: boolean;
  export let label: string;
  export let icon;
  export let onClick: () => void = () => {};

  export { className as class };
</script>

{#if active}
  <div class='flex flex-col p-4 justify-center items-center gap-1 text-slate-100 {className}'>
    <button on:click={onClick} class='w-10 h-10 rounded-full border-2 bg-transparent border-slate-100 hover:bg-slate-100 hover:text-slate-900 active:bg-slate-300 active:text-slate-900 duration-200 hover:duration-100 active:duration-100 active:scale-90 cursor-default'>
      <Icon src='{icon}' class='w-6 h-6 mx-auto' />
    </button>
    <p class='font-sans font-semibold text-xs'>{label}</p>
  </div>
{:else}
  <div class='flex flex-col p-4 justify-center items-center gap-1 text-slate-100 opacity-30 {className}'>
    <button disabled class='w-10 h-10 rounded-full border-2 border-slate-100 cursor-default'>
      <Icon src='{icon}' class='w-6 h-6 mx-auto' />
    </button>
    <p class='font-sans font-semibold text-xs'>{label}</p>
  </div>
{/if}
