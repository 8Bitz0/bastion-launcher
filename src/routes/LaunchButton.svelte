<script lang='ts'>
  import { Icon, XMark } from 'svelte-hero-icons';
  import { ButtonState } from './LaunchButtonState';

  let className: string = '';
  export let startLabel: string;
  export let cancelLabel: string;
  export let state: ButtonState;
  export let onStart: () => void = () => {};
  export let onCancel: () => void = () => {};

  export { className as class };
</script>

<div class='w-52 h-16 {className}'>
  {#if state === ButtonState.Ready}
    <button on:click={onStart} class='w-full h-full bg-gradient-to-r from-[#5ee25e] to-[#47c647] rounded-lg duration-150 active:scale-95 hover:scale-[1.02] active:from-[#39bf39] active:to-[#39bf39] cursor-pointer'>
      <p class='text-white text-[26px] font-bold tracking-wide'>{startLabel}</p>
    </button>
  {:else if state === ButtonState.NotReady}
    <button disabled class='w-full h-full bg-gray-600 rounded-lg border-gray-500 border-solid border-2 cursor-default'>
      <Icon src='{XMark}' class='text-gray-400' />
    </button>
  {:else if state === ButtonState.Running}
  <button on:click={onCancel} class='w-full h-full bg-gradient-to-r from-[#ff8446] to-[#ee6b14] rounded-lg cursor-default'>
    <p class='text-white text-[26px] font-bold tracking-wide'>{cancelLabel}</p>
  </button>
  {/if}
</div>
