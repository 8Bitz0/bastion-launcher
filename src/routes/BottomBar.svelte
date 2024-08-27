<script lang='ts'>
  import IconifyIcon from '@iconify/svelte';

  import { ButtonState } from './LaunchButtonState';
  import LaunchButton from './LaunchButton.svelte';

  let className = '';
  export let currentState: ButtonState;
  export let onLaunch: () => void = () => {};
  export let onCancel: () => void = () => {};

  export { className as class };;
</script>

<div class='w-full flex flex-row p-4 gap-8 {className}'>
  <LaunchButton onStart={onLaunch} onCancel={onCancel} startLabel='LAUNCH' cancelLabel='RUNNING' state={currentState} />
  <div class='flex-grow' />
</div>
