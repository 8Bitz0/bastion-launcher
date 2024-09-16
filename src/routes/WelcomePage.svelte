<script lang='ts'>
  export let changeReadyState: (ready: boolean) => void;

  changeReadyState(true);
</script>

<div class='w-full h-full absolute'>
  <div class='h-full flex flex-col items-center justify-center font-black font-intro'>
    <div class='flex flex-col items-center'>
      <h1 class='text-7xl'>Welcome to</h1>
      <h1 class='text-7xl text-transparent bg-clip-text bg-gradient-to-t from-orange-600 to-orange-300 drop-shadow-[0_0_36px_rgba(255,145,75,0.3)]'>Bastion Launcher</h1>
    </div>
  </div>
</div>
