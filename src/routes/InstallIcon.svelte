<script lang='ts'>
  import IconifyIcon from '@iconify/svelte';

  import { InstallPathType } from './InstallPath';

  let className = '';
  export let type: InstallPathType;

  export { className as class };
</script>

{#if type === InstallPathType.Steam}
  <IconifyIcon icon='mdi:steam' class='h-full w-8 {className}' />
{:else if type === InstallPathType.SteamFlatpak}
  <IconifyIcon icon='mdi:steam' class='h-full w-8 {className}' />
{:else}
  <IconifyIcon icon='mdi:folder-outline' class='h-full w-8 {className}' />
{/if}
