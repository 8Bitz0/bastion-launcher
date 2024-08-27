<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';

  import { launch } from '../scripts/Launch';

  import BottomBar from './BottomBar.svelte';
  import { ButtonState } from './LaunchButtonState';

  let currentState: ButtonState = ButtonState.Ready;

  async function launchGame() {
    currentState = ButtonState.Running;

    await launch().catch((e) => {
      console.error(e);
      currentState = ButtonState.Ready;
    });

    currentState = ButtonState.Ready;
  }
</script>

<div>
  <BottomBar onLaunch={launchGame} currentState={currentState} class='absolute bottom-0' />
</div>
