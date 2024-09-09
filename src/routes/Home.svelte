<script lang='ts'>
  import { type Config, defaultGameSettings, type GameSettings, getConfig, getGameSettings, setConfig, setGameSettings } from '../scripts/Config';
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
  let gameSettings: GameSettings = defaultGameSettings();

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

      // Get the index of the current launch method
      launchMethodIndex = launchMethods.indexOf(launchMethod);

      // If the launch method is not found, default to the first one
      if (launchMethodIndex === -1) launchMethodIndex = 0;
    });
  }

  processCurrentLaunchMethod();

  // Get the game settings from the config and set variables accordingly
  getGameSettings().then((s) => {
    gameSettings = s;

    setGameSettings(gameSettings);
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

  // Requests the game settings and updates the config
  function updateSettings(newSettings: GameSettings) {
    console.log(`Game settings updated (${newSettings})`);

    setGameSettings(newSettings);
  }

  // Matches the launch method enum to the index and updates the config
  function indexLaunchMethod(index: number) {
    if (launchMethods.length === 0) {
      return;
    }

    launchMethod = launchMethods[index];

    // Update the config with the new launch method
    getConfig().then((c) => {
      let config: Config = {
        ...c,
        launchMethod: launchMethod,
      };

      setConfig(config);
    });
  }

  $: updateSettings(gameSettings);

  $: indexLaunchMethod(launchMethodIndex);
</script>

<style>
  .sb-bg-hidden {
    visibility: hidden;
    opacity: 0;
  }

  .sb-hidden {
    opacity: 0;
    transform: translate(128px, 0);
  }
</style>

<div class='overflow-hidden'>
  <BottomBar bind:currentMethod={launchMethodIndex} onLaunch={launchGame} onSettings={openSidebar} currentState={currentState} launchMethods={launchMethods} class='absolute bottom-0' />
  <div class='absolute w-full h-full bg-[#00000030] backdrop-blur-2xl transition-all {additionalSBBGClasses}'>
    {#if gameSettings === undefined}
      <!-- Waiting -->
    {:else}
      <div class='h-full absolute right-0 transition-all {additionalSBClasses}'>
        <SettingsSidebar bind:gameSettings onClose={closeSidebar} launchMethod={launchMethod} />
      </div>
    {/if}
  </div>
</div>
