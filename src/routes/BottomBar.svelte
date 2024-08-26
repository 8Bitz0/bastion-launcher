<script lang='ts'>
  import IconifyIcon from '@iconify/svelte';

  import { ButtonState } from './ButtonState';
  import LaunchButton from './LaunchButton.svelte';

  let className = '';
  export { className as class };;
</script>

<div class='w-full flex flex-row p-4 gap-8 {className}'>
  <LaunchButton label='Play' state={ButtonState.NotReady} />
  <div class='flex-grow' />
</div>
