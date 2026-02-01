<script lang="ts">
  export let name:
    | "settings"
    | "play"
    | "pause"
    | "reset"
    | "download"
    | "upload"
    | "trash2"
    | "sun"
    | "moon"
    | "back"
    | "bar-chart" = "settings";
  export let className: string = "";
  export let size: number = 20;
</script>

{#if name === "settings"}
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width={size}
    height={size}
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="2"
    stroke-linecap="round"
    stroke-linejoin="round"
    class={className}
    ><path
      d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"
    /><circle cx="12" cy="12" r="3" /></svg
  >
{:else if name === "play"}
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width={size}
    height={size}
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="2"
    stroke-linecap="round"
    stroke-linejoin="round"
    class={className}><polygon points="5 3 19 12 5 21 5 3" /></svg
  >
{:else if name === "pause"}
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width={size}
    height={size}
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="2"
    stroke-linecap="round"
    stroke-linejoin="round"
    class={className}
    ><rect x="6" y="4" width="4" height="16" /><rect
      x="14"
      y="4"
      width="4"
      height="16"
    /></svg
  >
{:else if name === "reset"}
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width={size}
    height={size}
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="2"
    stroke-linecap="round"
    stroke-linejoin="round"
    class={className}
    ><path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" /><path
      d="M3 3v5h5"
    /></svg
  >
{:else if name === "download"}
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width={size}
    height={size}
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="2"
    stroke-linecap="round"
    stroke-linejoin="round"
    class={className}
    ><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" /><polyline
      points="7 10 12 15 17 10"
    /><line x1="12" y1="15" x2="12" y2="3" /></svg
  >
{:else if name === "upload"}
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width={size}
    height={size}
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="2"
    stroke-linecap="round"
    stroke-linejoin="round"
    class={className}
    ><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" /><polyline
      points="17 8 12 3 7 8"
    /><line x1="12" y1="3" x2="12" y2="15" /></svg
  >
{:else if name === "trash2"}
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width={size}
    height={size}
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="2"
    stroke-linecap="round"
    stroke-linejoin="round"
    class={className}
    ><path d="M3 6h18" /><path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" /><path
      d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"
    /><line x1="10" y1="11" x2="10" y2="17" /><line
      x1="14"
      y1="11"
      x2="14"
      y2="17"
    /></svg
  >
{:else if name === "sun"}
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width={size}
    height={size}
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="2"
    stroke-linecap="round"
    stroke-linejoin="round"
    class={className}
    ><circle cx="12" cy="12" r="4" /><path d="M12 2v2" /><path
      d="M12 20v2"
    /><path d="m4.93 4.93 1.41 1.41" /><path d="m17.66 17.66 1.41 1.41" /><path
      d="M2 12h2"
    /><path d="M20 12h2" /><path d="m6.34 17.66-1.41 1.41" /><path
      d="m19.07 4.93-1.41 1.41"
    /></svg
  >
{:else if name === "moon"}
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width={size}
    height={size}
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="2"
    stroke-linecap="round"
    stroke-linejoin="round"
    class={className}><path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" /></svg
  >
{:else if name === "back"}
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width={size}
    height={size}
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="2"
    stroke-linecap="round"
    stroke-linejoin="round"
    class={className}><path d="m12 19-7-7 7-7" /><path d="M19 12H5" /></svg
  >
{:else if name === "bar-chart"}
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width={size}
    height={size}
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="2"
    stroke-linecap="round"
    stroke-linejoin="round"
    class={className}
    ><line x1="12" y1="20" x2="12" y2="10" /><line
      x1="18"
      y1="20"
      x2="18"
      y2="4"
    /><line x1="6" y1="20" x2="6" y2="16" /></svg
  >
{/if}
