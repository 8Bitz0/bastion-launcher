<script lang='ts'>
  import Icon from '@iconify/svelte';

  import { type DropdownEntry } from "./DropdownEntry";

  let className: string = '';
  export let entries: DropdownEntry[] = [];
  export let index: number;

  let open: boolean = false;

  export { className as class };

  function changeMethod(i: number) {
    index = i;
    open = false;
  }

  $: addArrowClasses = open ? '' : 'ar-active';
  $: addDropdownClasses = open ? '' : 'dd-hidden';
</script>

<style>
  .ar-active {
    transform: rotate(180deg);
  }

  .dd-hidden {
    opacity: 0;
    visibility: hidden;
  }

  @media (prefers-reduced-motion: no-preference) {
    .dd-hidden {
      transform: translate(0, 10px);
    }
  }
</style>

<div class='w-52 h-16 flex flex-col-reverse gap-1 {className}'>
  <button
    on:click={() => open = !open}
    class='w-52 min-h-16 flex items-center bg-white bg-opacity-0 rounded-lg text-left text-nowrap p-2 transition-all hover:bg-opacity-10 active:scale-95 aria-expanded:bg-opacity-10 aria-expanded:scale-95 hover:transition-all cursor-pointer'
    aria-haspopup="true"
    aria-expanded="{open}"
  >
    {#if entries.length > 0}
      <p class='text-slate-300 flex-grow'>{entries[index].label}</p>
      <Icon icon='mdi:chevron-down' class='w-6 h-6 text-slate-300 transition-all {addArrowClasses}' />
    {:else}
      <p class='text-center text-slate-500 font-bold'>Loading...</p>
    {/if}
  </button>

  <div class='block w-full bg-slate-800 drop-shadow-2xl rounded-lg transition-all {addDropdownClasses}'>
    {#each entries as entry, i}
      <button
        class='w-52 h-16 flex flex-row items-center bg-white bg-opacity-0 rounded-lg text-left text-nowrap p-2 transition-all hover:bg-opacity-10 hover:transition-all cursor-pointer'
        on:click={() => changeMethod(i)}
      >
        <div>
          <p class='text-slate-300 flex-grow'>{entry.label}</p>
          {#if entry.subtitle !== undefined}
            <p class='text-slate-400 text-xs font-light'>{entry.subtitle}</p>
          {/if}
        </div>
      </button>
    {/each}
  </div>
</div>
