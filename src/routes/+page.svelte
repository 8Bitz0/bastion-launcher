<script lang='ts'>
  import { onMount } from 'svelte';
  import '../app.css';

  import { SetupPage } from './SetupPage';

  let setupFinished = false;

  import SetupScreen from './SetupScreen.svelte';
  import Home from './Home.svelte';

  let width = window.innerWidth;
  let height = window.innerHeight;

  onMount(() => {
    const handleResize = () => {
      width = window.innerWidth;
      height = window.innerHeight;
    };

    window.addEventListener('resize', handleResize);

    return () => {
      window.removeEventListener('resize', handleResize);
    };
  });

  $: scaleFactor = Math.min(width / 800, height / 500);
</script>

<div class='w-full h-full font-sans text-slate-200' style='zoom: {scaleFactor}'>
  {#if setupFinished}
    <Home />
  {:else}
    <SetupScreen onFinish={() => setupFinished = true} />
  {/if}
</div>
