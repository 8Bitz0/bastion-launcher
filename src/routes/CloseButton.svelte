<script lang='ts'>
  import Icon from '@iconify/svelte';

  let className: string = '';
  export let onClick: () => void = () => {};

  export { className as class };
</script>

<button on:click={onClick} class='flex items-center justify-center bg-slate-700 rounded-md text-slate-300 duration-150 hover:bg-slate-300 hover:text-slate-800 hover:duration-150 active:scale-95 {className}'>
  <Icon icon='mdi:close' class='w-[70%] h-[70%]' />
</button>
