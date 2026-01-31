<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { save } from "@tauri-apps/plugin-dialog";
  import { writeTextFile } from "@tauri-apps/plugin-fs";
  import { onMount, onDestroy } from "svelte";

  console.log("Svelte component loaded");

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
  let showTaskInput = true;

  $: minutes = Math.floor(remaining / 60)
    .toString()
    .padStart(2, "0");
  $: seconds = (remaining % 60).toString().padStart(2, "0");
  $: isRunning = !paused && remaining > 0;

  let unlisten: UnlistenFn | null = null;

  onMount(async () => {
    unlisten = await listen<TimerState>("timer-tick", (event) => {
      const payload = event.payload;
      remaining = payload.remaining;
      paused = payload.paused;
      phase = payload.phase === 0 ? "Work" : "Break";
      if (!hasStarted && payload.remaining > 0) {
        hasStarted = true;
      }
    });

    const initial = await invoke<TimerState>("get_timer");
    remaining = initial.remaining;
    paused = initial.paused;
    phase = initial.phase === 0 ? "Work" : "Break";
    if (initial.remaining > 0) {
      hasStarted = true;
    }
  });

  onDestroy(() => {
    unlisten?.();
  });

  async function startPomodoro() {
    await invoke("start_pomodoro", { taskName: taskName });
    hasStarted = true;
  }

  async function togglePause() {
    await invoke("pause_timer", { taskName: taskName });
    const updated = await invoke<TimerState>("get_timer");
    remaining = updated.remaining;
    paused = updated.paused;
    phase = updated.phase === 0 ? "Work" : "Break";
  }

  async function resetTimer() {
    await invoke("reset_timer");
    remaining = 1500;
    paused = true;
    phase = "Work";
    hasStarted = false;
    taskName = "";
    showTaskInput = true;
  }

  async function exportCSV() {
    console.log("Export CSV button clicked!");
    try {
      console.log("Starting CSV export...");
      const csv = await invoke<string>("export_csv");
      console.log("CSV data received:", csv.substring(0, 100) + "...");
      const filePath = await save({
        filters: [
          {
            name: "CSV",
            extensions: ["csv"],
          },
        ],
      });
      console.log("File path selected:", filePath);
      if (filePath) {
        await writeTextFile(filePath, csv);
        console.log("File written successfully");
      } else {
        console.log("No file path selected");
      }
    } catch (error) {
      console.error("Export failed:", error);
      alert("Failed to export CSV: " + error);
    }
  }
</script>

<main>
  <div class="timer">
    <div class="display">
      <div class="time">
        {minutes}:{seconds}
      </div>
      <div class="phase">{phase}</div>
    </div>

    <div class="task">
      <input
        bind:value={taskName}
        type="text"
        placeholder="Task name"
        class:running={isRunning}
        maxlength="50"
        disabled={isRunning}
      />
    </div>

    <div class="status">
      {paused ? "⏸️ Paused" : isRunning ? "▶️ Running" : "⏹️ Stopped"}
    </div>

    <div class="buttons">
      {#if !hasStarted || remaining === 0}
        <button class="start" on:click={startPomodoro}>▶️</button>
      {:else if hasStarted && remaining > 0}
        <div class="control-buttons">
          <button class="pause" on:click={togglePause}>
            {paused ? "▶️" : "⏸️"}
          </button>
          <button class="reset" on:click={resetTimer}>🔄</button>
        </div>
      {/if}

      <button class="export" on:click={exportCSV}>📊</button>
    </div>
  </div>
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    height: 100vh;
    background: radial-gradient(
      circle at 20% 80%,
      #1e3a8a 0%,
      #1e1b4b 50%,
      #0f0f23 100%
    );
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  main {
    width: 100%;
    max-width: 340px;
    padding: 1rem;
  }

  .timer {
    background: rgba(15, 23, 42, 0.85);
    backdrop-filter: blur(40px);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 28px;
    padding: 1.75rem 1.25rem;
    box-shadow:
      0 25px 50px -12px rgba(0, 0, 0, 0.5),
      0 0 0 1px rgba(255, 255, 255, 0.05);
    text-align: center;
    overflow: hidden;
  }

  .display {
    margin-bottom: 1.25rem;
  }

  .time {
    font-size: 3.75rem;
    font-weight: 200;
    font-family: "SF Mono", Monaco, "Roboto Mono", monospace;
    background: linear-gradient(135deg, #e2e8f0 0%, #f1f5f9 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    margin-bottom: 0.125rem;
    letter-spacing: 0;
    line-height: 1;
    font-variant-numeric: tabular-nums;
  }

  .phase {
    font-size: 0.75rem;
    font-weight: 600;
    color: rgba(156, 163, 175, 0.8);
    text-transform: uppercase;
    letter-spacing: 1.5px;
    line-height: 1;
  }

  .task {
    margin: 1.25rem 0 1rem 0;
    padding: 0 0.125rem;
  }

  .task input {
    width: 100%;
    padding: 0.6875rem 0.75rem;
    border: 1px solid rgba(71, 85, 105, 0.5);
    border-radius: 12px;
    font-size: 0.9375rem;
    background: rgba(30, 41, 59, 0.6);
    color: #f8fafc;
    text-align: center;
    backdrop-filter: blur(20px);
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    box-sizing: border-box;
  }

  .task input::placeholder {
    color: rgba(148, 163, 184, 0.6);
  }

  .task input:focus {
    outline: none;
    border-color: #60a5fa;
    background: rgba(30, 41, 59, 0.8);
    box-shadow: 0 0 0 3px rgba(99, 102, 241, 0.15);
  }

  .task input.running {
    background: rgba(71, 85, 105, 0.4);
    color: rgba(156, 163, 175, 0.7);
    border-color: rgba(71, 85, 105, 0.3);
  }

  .status {
    font-size: 0.75rem;
    color: rgba(148, 163, 175, 0.8);
    margin-bottom: 1.5rem;
    font-weight: 500;
    text-transform: uppercase;
    letter-spacing: 0.75px;
  }

  .buttons {
    display: flex;
    gap: 0.625rem;
    justify-content: center;
    padding-top: 0.25rem;
  }

  button {
    width: 52px;
    height: 52px;
    border: none;
    border-radius: 14px;
    font-size: 1.125rem;
    cursor: pointer;
    transition: all 0.15s cubic-bezier(0.4, 0, 0.2, 1);
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 600;
    backdrop-filter: blur(20px);
  }

  .start {
    width: 52px;
    height: 52px;
    font-size: 1.125rem;
    background: linear-gradient(135deg, #10b981 0%, #047857 100%);
    color: white;
    box-shadow: 0 4px 20px rgba(16, 185, 129, 0.35);
  }

  .start:hover:not(:active) {
    transform: translateY(-2px) scale(1.03);
    box-shadow: 0 8px 28px rgba(16, 185, 129, 0.45);
  }

  .start:active {
    transform: scale(0.97);
  }

  .pause {
    background: rgba(59, 130, 246, 0.2);
    color: #60a5fa;
    border: 1px solid rgba(59, 130, 246, 0.4);
  }

  .pause:hover:not(:active) {
    background: rgba(59, 130, 246, 0.3);
    transform: translateY(-1px);
  }

  .reset {
    background: rgba(239, 68, 68, 0.2);
    color: #fca5a5;
    border: 1px solid rgba(239, 68, 68, 0.4);
  }

  .reset:hover:not(:active) {
    background: rgba(239, 68, 68, 0.3);
    transform: translateY(-1px);
  }

  .export {
    background: rgba(251, 191, 36, 0.2);
    color: #fbbf24;
    border: 1px solid rgba(251, 191, 36, 0.4);
  }

  .export:hover:not(:active) {
    background: rgba(251, 191, 36, 0.3);
    transform: translateY(-1px);
  }

  @media (max-width: 420px) {
    main {
      padding: 0.75rem;
    }

    .timer {
      padding: 1.5rem 1rem;
    }

    .time {
      font-size: 3.25rem;
    }

    button {
      width: 48px;
      height: 48px;
    }

    .start {
      width: 48px;
      height: 48px;
    }
  }

  .control-buttons {
    display: flex;
    gap: 0.625rem;
  }
</style>
