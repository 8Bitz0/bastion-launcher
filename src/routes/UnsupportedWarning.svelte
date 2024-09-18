<script lang='ts'>
  let className: string = '';
  export let changeReadyState: (ready: boolean) => void;

  export { className as class };

  changeReadyState(false);

  setTimeout(() => {
    changeReadyState(true);
  }, 3000);
</script>

<div class='w-full h-full absolute {className}'>
  <div class='h-full flex flex-col items-center justify-center font-intro -translate-y-8'>
    <div class='flex flex-col items-center p-16 gap-4 font-bold'>
      <h1 class='text-4xl text-center'>Notice</h1>
      <p class='text-md text-center text-slate-300'>Bastion Launcher is not affiliated with BeamNG GmbH. As a third-party project, Bastion Launcher does not receive any endorsement or support from the BeamNG GmbH team.</p>
      <p class='text-md text-center text-slate-300'>Additionally, BeamNG.drive currently only supports the most recent version of Microsoft Windows and does not support any other platforms. Native Linux support is considered experimental by BeamNG.drive. Compatibility layers are not supported by BeamNG.drive and related issues must be reported to Bastion Launcher.</p>
      <p class='text-md text-center text-slate-300'>Do not go to BeamNG.drive for support with Bastion Launcher. If you come across an issue within BeamNG.drive, use the official launcher before reporting an issue.</p>
    </div>
  </div>
</div>
