<script lang='ts'>
  let className: string = '';
  export let label: string = '';
  export let switchState: boolean = false;
  export let onSwitch: (state: boolean) => void = () => {};

  export { className as class };

  function handleSwitch() {
    switchState = !switchState;

    onSwitch(switchState);
  }
</script>

<div class='{className}'>
  <div class='inline-flex w-full flex-row items-center py-2'>
    <p class='flex-grow'>{label}</p>
    <button role='switch' aria-checked={switchState} on:click={handleSwitch} class='relative w-12 h-6 bg-slate-300 rounded-full transition-all aria-checked:after:translate-x-full rtl:aria-checked:after:-translate-x-full aria-checked:bg-blue-600 aria-checked:transition-all after:content-[""] after:absolute after:top-0 after:start-0 after:bg-white after:border-gray-300 after:border after:rounded-full after:h-6 after:w-6 after:transition-all'></button>
  </div>
</div>
