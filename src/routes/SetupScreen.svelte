<script lang='ts'>
  import { ArrowRight } from 'svelte-hero-icons';

  import { type Config, type GameSettings, getConfig, setConfig } from '../scripts/Config';
  import RadialSetupButton from './RadialSetupButton.svelte';
  import WelcomePage from './WelcomePage.svelte';
  import ChooseInstall from './ChooseInstall.svelte';

  export let page: number = 0;
  export let onFinish: () => void;
  let lastPage: number = 1;

  let chosenInstall: string = '';
  let customInstalls: string[] = [];

  let nextButtonActive: boolean = false;
  let nextButtonLabel: string;

  $: nextButtonLabel = page === lastPage ? 'Finish' : 'Next';

  async function onNext() {
    if (page === lastPage) {
      let preConfig: Config = await getConfig();

      let config: Config = {
        setupFinished: true,
        currentInstallPath: chosenInstall,
        customInstallPaths: customInstalls,
        gameSettings: {} as Record<string, GameSettings>,
        launchMethod: preConfig.launchMethod,
      };

      setConfig(config);

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
      <ChooseInstall changeReadyState={(ready) => { nextButtonActive = ready }} onUpdate={(selected, custom) => {
        chosenInstall = selected;
        customInstalls = custom;
        }}
      />
    </div>
  {/if}
</div>
<div class='flex justify-center items-center'>
  <RadialSetupButton onClick={onNext} active={nextButtonActive} class='absolute top-3/4' label={nextButtonLabel} icon={ArrowRight} />
</div>
