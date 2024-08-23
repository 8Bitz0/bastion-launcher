<script lang='ts'>
  let className = '';
  export { className as class };

  import { ButtonState } from './ButtonState';
  import LaunchButton from './LaunchButton.svelte';
</script>

<div class='w-full flex flex-col p-4 {className}'>
  <LaunchButton label='Play' state={ButtonState.NotReady} />
</div>
