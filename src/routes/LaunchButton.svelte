<script lang='ts'>
  import { Icon, type IconSource, XMark } from 'svelte-hero-icons';
  import { ButtonState } from './ButtonState';

  let className: string = '';
  export let label: string;
  export let state: ButtonState;

  export { className as class };
</script>

<div class='w-48 h-16'>
  <!-- Active button, needs other variants -->
  {#if state === ButtonState.Ready}
    <button class='w-full h-full bg-gradient-to-r from-[#4ef34e] to-[#1fc71f] rounded-lg {className}'>
      <p class='text-white text-2xl font-black'>{label}</p>
    </button>
  {:else if state === ButtonState.NotReady}
    <button disabled class='w-full h-full bg-gray-600 rounded-lg border-gray-500 border-solid border-2 {className}'>
      <Icon src='{XMark}' class='text-gray-400' />
    </button>
  {/if}
</div>

