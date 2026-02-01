<script lang="ts">
  import "../app.css";
  import { onMount, onDestroy } from "svelte";
  import { theme } from "$lib/stores/theme";
  import { get } from "svelte/store";
  import { currentTaskName } from "$lib/stores/currentTask";
  import { invoke } from "@tauri-apps/api/core";
  import { register, unregister } from "@tauri-apps/plugin-global-shortcut";

  const TOGGLE_PAUSE_SHORTCUT = "CommandOrControl+Shift+P";

  if (typeof document !== "undefined") {
    theme.init();
  }

  onMount(async () => {
    await register(TOGGLE_PAUSE_SHORTCUT, async (event) => {
      if (event.state === "Pressed") {
        const taskName = get(currentTaskName);
        try {
          await invoke("pause_timer", { taskName });
          window.dispatchEvent(new CustomEvent("shortcut-pause"));
        } catch (_) {}
      }
    });
  });

  onDestroy(() => {
    unregister(TOGGLE_PAUSE_SHORTCUT);
  });
</script>

<slot />
