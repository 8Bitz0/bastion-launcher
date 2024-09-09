<script lang='ts'>
  import { onMount } from 'svelte';

  import '../app.css';
  import { getConfig, loadConfig } from '../scripts/Config';

  import SetupScreen from './SetupScreen.svelte';
  import Home from './Home.svelte';

  let loadingFinished = false;

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

  function processScaleFactor(width: number, height: number): number {
    let scaleFactor = Math.min(width / 800, height / 500)
    let postScaleDifference = 0;

    if (width > 1000 || height > 625) {
      postScaleDifference = -0.5;
    }

    return scaleFactor + postScaleDifference;
  }

  $: scaleFactor = processScaleFactor(width, height);

  let setupFinished = false;

  async function init() {
    await loadConfig();

    setupFinished = (await getConfig()).setupFinished;

    loadingFinished = true;
  }

  init().then(() => {console.log('Init finished')});
</script>

<div class='w-full h-full font-sans text-slate-200 overflow-hidden' style='zoom: {scaleFactor}'>
  {#if loadingFinished}
    {#if setupFinished}
      <Home />
    {:else}
      <SetupScreen onFinish={() => setupFinished = true} />
    {/if}
  {/if}
</div>
