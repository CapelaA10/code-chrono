<script lang="ts">
  import { get } from "svelte/store";
  import { invoke } from "@tauri-apps/api/core";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { onMount, onDestroy } from "svelte";
  import TimerCard from "$lib/components/TimerCard.svelte";
  import { idleMinutes } from "$lib/stores/idle";

  type TimerState = {
    remaining: number;
    paused: boolean;
    phase: number;
  };

  let remaining = 1500;
  let paused = false;
  let phase = "Work";
  let hasStarted = false;
  let isRunning = false;
  let taskName = "";
  let recentTasks: string[] = [];
  let idleTimeoutId: ReturnType<typeof setTimeout> | null = null;

  $: isRunning = !paused && remaining > 0;

  let unlisten: UnlistenFn | null = null;

  function syncFromBackend(state: TimerState) {
    remaining = state.remaining;
    paused = state.paused;
    phase = state.phase === 0 ? "Work" : "Break";
    if (state.remaining > 0 && !hasStarted) hasStarted = true;
  }

  function clearIdleTimeout() {
    if (idleTimeoutId != null) {
      clearTimeout(idleTimeoutId);
      idleTimeoutId = null;
    }
  }

  function scheduleIdlePause() {
    clearIdleTimeout();
    const mins = get(idleMinutes);
    if (mins <= 0 || !isRunning) return;
    idleTimeoutId = setTimeout(async () => {
      try {
        await invoke("pause_timer", { taskName });
        const updated = await invoke<TimerState>("get_timer");
        syncFromBackend(updated);
      } catch (_) {}
      idleTimeoutId = null;
    }, mins * 60 * 1000);
  }

  onMount(async () => {
    unlisten = await listen<TimerState>("timer-tick", (event) => {
      syncFromBackend(event.payload);
      scheduleIdlePause();
    });

    const initial = await invoke<TimerState>("get_timer");
    syncFromBackend(initial);

    window.addEventListener("shortcut-pause", onShortcutPause);
    document.addEventListener("visibilitychange", onVisibilityChange);

    try {
      recentTasks = await invoke<string[]>("get_unique_task_names", { limit: 10 });
    } catch (_) {
      recentTasks = [];
    }
  });

  async function onShortcutPause() {
    const updated = await invoke<TimerState>("get_timer");
    syncFromBackend(updated);
  }

  function onVisibilityChange() {
    if (document.visibilityState === "visible") {
      clearIdleTimeout();
      if (isRunning) scheduleIdlePause();
    } else {
      scheduleIdlePause();
    }
  }

  onDestroy(() => {
    unlisten?.();
    window.removeEventListener("shortcut-pause", onShortcutPause);
    document.removeEventListener("visibilitychange", onVisibilityChange);
    clearIdleTimeout();
  });

  async function startPomodoro() {
    await invoke("start_pomodoro", { taskName });
    hasStarted = true;
    scheduleIdlePause();
  }

  async function togglePause() {
    await invoke("pause_timer", { taskName });
    const updated = await invoke<TimerState>("get_timer");
    syncFromBackend(updated);
    clearIdleTimeout();
    if (updated.paused === false) scheduleIdlePause();
  }

  async function resetTimer() {
    await invoke("reset_timer");
    remaining = 1500;
    paused = true;
    phase = "Work";
    hasStarted = false;
    taskName = "";
    clearIdleTimeout();
  }
</script>

<main class="page">
  <TimerCard
    bind:taskName
    {remaining}
    {paused}
    {phase}
    {hasStarted}
    {isRunning}
    {recentTasks}
    onStart={startPomodoro}
    onPause={togglePause}
    onReset={resetTimer}
  />
</main>

<style>
  .page {
    width: 100%;
    max-width: 320px;
    padding: 1rem;
  }

  :global(body) {
    margin: 0;
    padding: 0;
    min-height: 100vh;
    background: var(--bg-page);
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text);
  }
</style>
