<script lang="ts">
  import "../app.css";
  import { onMount, onDestroy } from "svelte";
  import { theme } from "$lib/stores/theme";
  import { invoke } from "@tauri-apps/api/core";
  import { register, unregister } from "@tauri-apps/plugin-global-shortcut";
  import { initTimerStore } from "$lib/stores/timer";
  import Sidebar from "$lib/components/Sidebar.svelte";
  import UpdateChecker from "$lib/components/UpdateChecker.svelte";
  import BreakBanner from "$lib/components/timer/BreakBanner.svelte";
  import ProgramNotificationModal from "$lib/components/ProgramNotificationModal.svelte";
  import { getCurrentWindow } from "@tauri-apps/api/window";

  const TOGGLE_PAUSE_SHORTCUT = "CommandOrControl+Shift+P";

  if (typeof document !== "undefined") {
    theme.init();
  }

  let platformClass = '';

  onMount(async () => {
    // Detect platform for CSS targeting
    const platform = typeof navigator !== 'undefined' ? navigator.platform.toLowerCase() : '';
    if (platform.includes('mac')) platformClass = 'platform-macos';
    else if (platform.includes('win')) platformClass = 'platform-windows';
    else platformClass = 'platform-linux';

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

  // Sync native window background color with the theme
  $: if (typeof window !== 'undefined' && $theme) {
    const isDark = $theme === 'dark';
    const color = isDark ? '#15181e' : '#ffffff';
    getCurrentWindow().setBackgroundColor(color).catch(() => {});
  }

  onDestroy(() => {
    unregister(TOGGLE_PAUSE_SHORTCUT);
  });
</script>

<div class="app-layout {platformClass}" data-theme={$theme}>
  <Sidebar />
  <div class="main-container">
    <div class="banner-wrapper">
      <BreakBanner />
    </div>
    <slot />
  </div>
</div>

<UpdateChecker />
<ProgramNotificationModal />

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
    position: relative;
  }

  .banner-wrapper {
    position: absolute;
    top: 1.5rem;
    left: 50%;
    transform: translateX(-50%);
    z-index: 50;
  }

  :global(body) {
    margin: 0;
    padding: 0;
    background: var(--bg-page);
    color: var(--text);
    font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  }
</style>
