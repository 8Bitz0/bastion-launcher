<script lang='ts'>
  import Icon from '@iconify/svelte';

  let className: string = '';
  export let label: string = '';
  export let entries: { label: string; }[] = [];
  export let index: number = 0;

  export { className as class };

  let open: boolean = false;

  $: addArrowClasses = open ? '' : 'arr-active';
  $: addDropdownClasses = open ? '' : 'dd-hidden';

  function handleChoice(i: number) {
    index = i;

    open = false;
  }
</script>

<style>
  .arr-active {
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

<div class='{className}'>
  <div class='inline-flex w-full flex-row items-center py-2'>
    <p class='flex-grow'>{label}</p>
    <div class='flex flex-col w-28 h-10'>
      <button
        on:click={() => open = !open}
        class='w-28 min-h-10 h-10 flex items-center bg-slate-700 rounded-lg text-left text-nowrap p-2 transition-all hover:bg-slate-600 active:scale-95 aria-expanded:bg-slate-600 aria-expanded:scale-95 hover:transition-all cursor-pointer'
        aria-haspopup="true"
        aria-expanded="{open}"
      >
        {#if entries.length > 0}
          <p class='text-slate-300 flex-grow'>{entries[index].label}</p>
          <div class='w-6 h-6 text-slate-300 transition-all {addArrowClasses}'>
            <Icon icon='mdi:chevron-up' class='w-full h-full' />
          </div>
        {:else}
          <p class='text-center text-slate-500 font-bold'>Loading...</p>
        {/if}
      </button>

      {#if open}
        <div on:click={() => open = false} role='none' class='fixed inset-0'></div>
      {/if}
      <div class='block w-28 bg-slate-700 drop-shadow-2xl rounded-lg transition-all {addDropdownClasses}'>
        {#each entries as entry, i}
          <button
            class='w-full h-8 flex flex-row items-center bg-white bg-opacity-0 rounded-lg text-left text-nowrap text-sm p-2 transition-all hover:bg-opacity-10 hover:transition-all cursor-pointer'
            on:click={() => handleChoice(i)}
          >
            <div>
              <p class='text-slate-300 flex-grow'>{entry.label}</p>
            </div>
          </button>
        {/each}
      </div>
    </div>
  </div>
</div>
  