<script>
  import { ArrowRight } from "svelte-hero-icons";

  import RadialSetupButton from "./RadialSetupButton.svelte";
</script>

<div class="w-full h-full absolute">
  <!-- Create a column for centered elements -->
  <div class="h-full flex flex-col items-center justify-center font-black font-intro">
    <div class="flex flex-col items-center">
      <h1 class="text-7xl">Welcome to</h1>
      <h1 class="text-7xl text-transparent bg-clip-text bg-gradient-to-t from-orange-600 to-orange-300">Bastion Launcher</h1>
    </div>
    <RadialSetupButton class="absolute top-3/4" label="Next" icon={ArrowRight} />
  </div>
</div>
