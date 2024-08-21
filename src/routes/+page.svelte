<script lang="ts">
  import "../app.css";

  let setupFinished = false;

  import SetupScreen from "./SetupScreen.svelte";
  import Home from "./Home.svelte";
</script>

<div class="w-full h-full font-sans text-slate-200">
  {#if setupFinished}
    <Home />
  {:else}
    <SetupScreen />
  {/if}
</div>
