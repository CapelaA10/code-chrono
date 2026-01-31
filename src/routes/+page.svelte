<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { save } from "@tauri-apps/plugin-dialog";
  import { writeTextFile } from "@tauri-apps/plugin-fs";
  import { onMount, onDestroy } from "svelte";

  console.log('Svelte component loaded');

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
    paused = false;
    phase = "Work";
    hasStarted = false;
    showTaskInput = true;
  }

  async function exportCSV() {
    console.log('Export CSV button clicked!');
    try {
      console.log('Starting CSV export...');
      const csv = await invoke<string>("export_csv");
      console.log('CSV data received:', csv.substring(0, 100) + '...');
      const filePath = await save({
        filters: [{
          name: 'CSV',
          extensions: ['csv']
        }]
      });
      console.log('File path selected:', filePath);
      if (filePath) {
        await writeTextFile(filePath, csv);
        console.log('File written successfully');
      } else {
        console.log('No file path selected');
      }
    } catch (error) {
      console.error('Export failed:', error);
      alert('Failed to export CSV: ' + error);
    }
  }
</script>

<main class="container">
  <h1>Code Chrono ⏱️</h1>

  <!-- NEW: Task Input Section -->
  {#if showTaskInput}
    <div class="task-section">
      <label for="task-input" class="task-label">Current Task:</label>
      <div class="task-input-wrapper">
        <input
          id="task-input"
          bind:value={taskName}
          type="text"
          placeholder="What are you working on?"
          class="task-input"
          class:task-input--disabled={isRunning}
          disabled={isRunning}
          maxlength="100"
        />
        {#if isRunning}
          <span class="task-lock-icon">🔒</span>
        {/if}
      </div>
    </div>
  {/if}

  <div class="timer-display">
    <div class="time">
      {Math.floor(remaining / 60)
        .toString()
        .padStart(2, "0")}:
      {(remaining % 60).toString().padStart(2, "0")}
    </div>
    <div class="phase">{phase}</div>
    {#if taskName}
      <div class="current-task">📝 {taskName}</div>
    {/if}
    <div class="status">{paused ? "⏸️ Paused" : "▶️ Running"}</div>
  </div>

  <div class="controls">
    {#if !hasStarted || remaining === 0}
      <button class="btn btn-start" on:click={startPomodoro}>
        Start Pomodoro (25min)
      </button>
    {/if}

    {#if hasStarted && remaining > 0}
      <button class="btn btn-pause" on:click={togglePause}>
        {paused ? "▶️ Resume" : "⏸️ Pause"}
      </button>
    {/if}

    <button class="btn btn-reset" on:click={resetTimer}>
      🔄 Reset
    </button>

    <button class="btn btn-export" on:click={exportCSV}>
      📊 Export CSV
    </button>
  </div>
</main>

<style>
  .container {
    max-width: 500px;
    margin: 0 auto;
    padding: 2rem;
    text-align: center;
    font-family: -apple-system, BlinkMacSystemFont, sans-serif;
    background: #ffffff;
    min-height: 100vh;
  }

  h1 {
    color: #1a1a1a;
    margin-bottom: 1rem;
    font-weight: 600;
  }

  /* Task Styles */
  .task-section {
    margin-bottom: 2rem;
    text-align: left;
  }

  .task-label {
    display: block;
    font-size: 0.9rem;
    font-weight: 500;
    color: #666;
    margin-bottom: 0.5rem;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .task-input-wrapper {
    position: relative;
    display: flex;
    align-items: center;
  }

  .task-input {
    flex: 1;
    padding: 0.75rem 1rem;
    border: 1px solid #d1d5db;
    border-radius: 8px;
    font-size: 1rem;
    transition: border-color 0.2s ease;
    background: white;
  }

  .task-input:focus {
    outline: none;
    border-color: #3b82f6;
    box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.1);
  }

  .task-input--disabled {
    background: #f9fafb;
    color: #6b7280;
    cursor: not-allowed;
  }

  .task-lock-icon {
    margin-left: 0.75rem;
    font-size: 1.2rem;
    opacity: 0.6;
  }

  .current-task {
    font-size: 1.1rem;
    color: #374151;
    font-weight: 500;
    margin: 0.5rem 0;
    padding: 0.25rem 0.75rem;
    background: #f3f4f6;
    border-radius: 6px;
    border-left: 3px solid #3b82f6;
  }

  .timer-display {
    margin: 2rem 0 3rem;
  }

  .time {
    font-size: 6rem;
    font-weight: 300;
    font-family: monospace;
    color: #1f2937;
    margin-bottom: 1rem;
  }

  .phase {
    font-size: 1.5rem;
    color: #6b7280;
    text-transform: uppercase;
    letter-spacing: 1px;
    margin-bottom: 0.5rem;
  }

  .status {
    font-size: 1.2rem;
    color: #9ca3af;
  }

  .controls {
    display: flex;
    gap: 0.75rem;
    justify-content: center;
    flex-wrap: wrap;
  }

  .btn {
    padding: 0.75rem 1.5rem;
    border: none;
    border-radius: 8px;
    font-size: 1rem;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
    min-width: 120px;
  }

  .btn-start {
    background: #3b82f6;
    color: white;
  }

  .btn-start:hover:not(:disabled) {
    background: #2563eb;
  }

  .btn-pause {
    background: #f3f4f6;
    color: #374151;
    border: 1px solid #d1d5db;
  }

  .btn-pause:hover:not(:disabled) {
    background: #e5e7eb;
  }

  .btn-reset {
    background: #ef4444;
    color: white;
  }

  .btn-reset:hover:not(:disabled) {
    background: #dc2626;
  }

  .btn-export {
    background: #10b981;
    color: white;
  }

  .btn-export:hover:not(:disabled) {
    background: #059669;
  }
</style>
