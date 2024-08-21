<script lang="ts">
  import { Icon, type IconSource } from "svelte-hero-icons";

  let className: string = '';
  export let label: string;
  export let icon;

  export { className as class };
</script>

<div class="flex flex-col p-4 justify-center items-center {className}">
  <button class="w-10 h-10 rounded-full border-white border-2">
    <Icon src="{icon}" class="text-white scale-[70%]" />
  </button>
  <p class="font-sans text-xs">{label}</p>
</div>
