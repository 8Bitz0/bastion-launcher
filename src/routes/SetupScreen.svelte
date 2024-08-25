<script lang='ts'>
  import { ArrowRight } from 'svelte-hero-icons';

  import RadialSetupButton from './RadialSetupButton.svelte';
  import WelcomePage from './WelcomePage.svelte';
  import ChooseInstall from './ChooseInstall.svelte';
  import { SetupPage } from './SetupPage';

  export let page: number = 0;
  export let onFinish: () => void;

  let nextButtonActive: boolean;
  let lastPage: number = 1;

  let nextButtonLabel: string;

  $: nextButtonLabel = page === lastPage ? 'Finish' : 'Next';

  function onNext() {
    if (page === lastPage) {
      onFinish();
      return;
    }

    page += 1;
  }
</script>

<div class='w-full h-full absolute'>
  {#if page == 0}
    <WelcomePage changeReadyState={(ready) => nextButtonActive = ready} />
  {:else if page == 1}
    <div class='w-full h-full absolute p-14 pt-24'>
      <ChooseInstall />
    </div>
  {/if}
</div>
<div class='flex justify-center items-center'>
  <RadialSetupButton onClick={onNext} active={nextButtonActive} class='absolute top-3/4' label={nextButtonLabel} icon={ArrowRight} />
</div>
