<script lang="ts">
  import "../app.css";
  import { onMount, onDestroy } from "svelte";
  import { theme } from "$lib/stores/theme";
  import { invoke } from "@tauri-apps/api/core";
  import { register, unregister } from "@tauri-apps/plugin-global-shortcut";
  import { initTimerStore } from "$lib/stores/timer";
  import Sidebar from "$lib/components/Sidebar.svelte";

  const TOGGLE_PAUSE_SHORTCUT = "CommandOrControl+Shift+P";

  if (typeof document !== "undefined") {
    theme.init();
  }

  onMount(async () => {
    await initTimerStore();
    await register(TOGGLE_PAUSE_SHORTCUT, async (event) => {
      if (event.state === "Pressed") {
        // The backend tracks the active task name — pass empty string to toggle pause
        try {
          await invoke("pause_timer");
        } catch (_) {}
      }
    });
  });

  onDestroy(() => {
    unregister(TOGGLE_PAUSE_SHORTCUT);
  });
</script>

<div class="app-layout" data-theme={$theme}>
  <Sidebar />
  <div class="main-container">
    <slot />
  </div>
</div>

<style>
  .app-layout {
    display: flex;
    width: 100vw;
    height: 100vh;
    overflow: hidden;
    background: var(--bg-page);
    color: var(--text);
  }

  .main-container {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  :global(body) {
    margin: 0;
    padding: 0;
    font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  }
</style>
