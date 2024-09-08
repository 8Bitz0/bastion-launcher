<script lang='ts'>
  import { type Config, type GameSettings, getConfig, getGameSettings, setConfig, setGameSettings } from '../scripts/Config';
  import { launch } from '../scripts/Launch';
  import { getLaunchMethods, LaunchMethod } from '../scripts/LaunchMethod';

  import { ButtonState } from './LaunchButtonState';
  import BottomBar from './BottomBar.svelte';
  import SettingsSidebar from './SettingsSidebar.svelte';

  let currentState: ButtonState = ButtonState.Ready;

  const SB_BG_HIDDEN: string = 'sb-bg-hidden';
  const SB_HIDDEN: string = 'sb-hidden';
  let additionalSBBGClasses: string = SB_BG_HIDDEN;
  let additionalSBClasses: string = SB_HIDDEN;

  let launchMethod: LaunchMethod;
  let gameSettings: GameSettings | undefined = undefined;

  let launchMethods: LaunchMethod[] = [];
  let launchMethodIndex: number = 0;

  // Updates the `launchMethods` array
  async function updateLaunchMethods() {
    launchMethods = await getLaunchMethods();
  }

  function processCurrentLaunchMethod() {
    updateLaunchMethods();

    // Get the launch method from the config
    getConfig().then((c) => {
      launchMethod = c.launchMethod;

      console.log(c.launchMethod)

      // Get the index of the current launch method
      launchMethodIndex = launchMethods.indexOf(launchMethod);

      console.log(launchMethods.indexOf(launchMethod));
      console.log(launchMethods);

      // If the launch method is not found, default to the first one
      if (launchMethodIndex === -1) launchMethodIndex = 0;

      console.log(launchMethodIndex);
    });
  }

  processCurrentLaunchMethod();

  getGameSettings().then((s) => {
    gameSettings = s;
  });

  async function launchGame() {
    currentState = ButtonState.Running;

    await launch().catch((e) => {
      console.error(e);
      currentState = ButtonState.Ready;
    });

    currentState = ButtonState.Ready;
  }

  function openSidebar() {
    additionalSBClasses = '';
    additionalSBBGClasses = '';
  }

  function closeSidebar() {
    additionalSBClasses = SB_HIDDEN;
    additionalSBBGClasses = SB_BG_HIDDEN;
  }

  function updateSettings() {
    if (gameSettings === undefined) {
      return;
    }

    setGameSettings(gameSettings);
  }

  function indexLaunchMethod(index: number) {
    if (launchMethods.length === 0) {
      return;
    }

    launchMethod = launchMethods[launchMethodIndex];

    // Update the config with the new launch method
    getConfig().then((c) => {
      let config: Config = {
        ...c,
        launchMethod: launchMethod,
      };

      setConfig(config);
    });
  }

  $: indexLaunchMethod(launchMethodIndex);
</script>

<style>
  .sb-bg-hidden {
    visibility: hidden;
    opacity: 0;
  }

  .sb-hidden {
    opacity: 0;
  }

  @media (prefers-reduced-motion: no-preference) {
    .sb-hidden {
      transform: translate(128px, 0);
    }
  }
</style>

<div class='overflow-hidden'>
  <BottomBar bind:currentMethod={launchMethodIndex} onLaunch={launchGame} onSettings={openSidebar} currentState={currentState} launchMethods={launchMethods} class='absolute bottom-0' />
  <div class='absolute w-full h-full bg-[#00000030] backdrop-blur-2xl transition-all {additionalSBBGClasses}'>
    {#if gameSettings === undefined}
      <!-- Waiting -->
    {:else}
      <div class='h-full absolute right-0 transition-all {additionalSBClasses}'>
        <SettingsSidebar bind:gameSettings updateSettings={updateSettings} onClose={closeSidebar} launchMethod={launchMethod} />
      </div>
    {/if}
  </div>
</div>
