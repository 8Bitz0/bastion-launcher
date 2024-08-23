<script lang='ts'>
  import { Icon, type IconSource } from 'svelte-hero-icons';

  let className: string = '';
  export let label: string;
  export let icon;

  export { className as class };
</script>

<div class='flex flex-col p-4 justify-center items-center gap-1 text-neutral-100 border-neutral-100 {className}'>
  <button class='w-10 h-10 rounded-full border-2 bg-transparent hover:bg-neutral-100 hover:text-slate-900 active:bg-neutral-300 active:text-slate-900 duration-200 hover:duration-100 active:duration-100 active:scale-90 cursor-default'>
    <Icon src='{icon}' class='w-6 h-6 mx-auto' />
  </button>
  <p class='font-sans font-semibold text-xs'>{label}</p>
</div>
