<script lang='ts'>
  import WelcomePage from './WelcomePage.svelte';
  import ChooseInstall from './ChooseInstall.svelte';
  import { SetupPage } from './SetupPage';

  export let page: SetupPage
</script>

<div class='w-full h-full absolute'>
  {#if page === SetupPage.Welcome}
    <WelcomePage />
  {:else if page === SetupPage.ChooseInstall}
    <div class='w-full h-full absolute p-14 pt-24'>
      <ChooseInstall />
    </div>
  {/if}
</div>
