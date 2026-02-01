<script lang="ts">
  import Icons from "./Icons.svelte";
  import { currentTaskName } from "$lib/stores/currentTask";

  export let remaining = 1500;
  export let paused = false;
  export let phase = "Work";
  export let hasStarted = false;
  export let isRunning = false;
  export let taskName = "";
  export let recentTasks: string[] = [];
  export let onStart: (duration?: number) => void = () => {};
  export let onPause: () => void = () => {};
  export let onReset: () => void = () => {};

  let selectedDuration: "25" | "45" | "60" | "custom" = "25";
  let customDuration = "";

  $: minutes = Math.floor(remaining / 60)
    .toString()
    .padStart(2, "0");
  $: seconds = (remaining % 60).toString().padStart(2, "0");

  $: currentTaskName.set(taskName);

  function pickTask(name: string) {
    taskName = name;
  }

  function handleStart() {
    let durationMinutes: number | undefined;
    
    if (selectedDuration === "custom") {
      const parsed = parseInt(customDuration);
      if (!isNaN(parsed) && parsed > 0) {
        durationMinutes = parsed;
      } else {
        durationMinutes = 25; // fallback
      }
    } else {
      durationMinutes = parseInt(selectedDuration);
    }
    
    onStart(durationMinutes);
  }
</script>

<div class="timer">
  <div class="top-nav">
    <a href="/stats" class="nav-link" title="Statistics">
      <Icons name="bar-chart" size={20} />
    </a>
    <a href="/settings" class="nav-link" title="Settings">
      <Icons name="settings" size={20} />
    </a>
  </div>
  <div class="display">
    <div class="time">{minutes}:{seconds}</div>
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
    {#if recentTasks.length > 0 && !isRunning}
      <div class="recent-tasks">
        <span class="recent-label">Recent:</span>
        {#each recentTasks as name (name)}
          <button
            type="button"
            class="task-chip"
            on:click={() => pickTask(name)}
          >
            {name}
          </button>
        {/each}
      </div>
    {/if}
  </div>

  <div class="status">
    {#if !hasStarted}
      Ready
    {:else if paused}
      Paused
    {:else if isRunning}
      Running
    {:else}
      Stopped
    {/if}
  </div>

  {#if !hasStarted || remaining === 0}
    <div class="duration-selector">
      <label class="duration-label">Session duration</label>
      <div class="duration-options">
        <button
          type="button"
          class="duration-btn"
          class:active={selectedDuration === "25"}
          on:click={() => selectedDuration = "25"}
        >
          25 min
        </button>
        <button
          type="button"
          class="duration-btn"
          class:active={selectedDuration === "45"}
          on:click={() => selectedDuration = "45"}
        >
          45 min
        </button>
        <button
          type="button"
          class="duration-btn"
          class:active={selectedDuration === "60"}
          on:click={() => selectedDuration = "60"}
        >
          60 min
        </button>
        <button
          type="button"
          class="duration-btn"
          class:active={selectedDuration === "custom"}
          on:click={() => selectedDuration = "custom"}
        >
          Custom
        </button>
      </div>
      {#if selectedDuration === "custom"}
        <div class="custom-duration">
          <input
            type="number"
            bind:value={customDuration}
            placeholder="Minutes"
            min="1"
            max="480"
            class="custom-input"
          />
        </div>
      {/if}
    </div>
  {/if}

  <div class="buttons">
    {#if !hasStarted || remaining === 0}
      <button class="btn start" on:click={handleStart}>
        <Icons name="play" size={18} className="btn-icon" />
        Start
      </button>
    {:else if hasStarted && remaining > 0}
      <div class="control-buttons">
        <button class="btn pause" on:click={onPause}>
          <Icons name={paused ? "play" : "pause"} size={18} className="btn-icon" />
          {paused ? "Resume" : "Pause"}
        </button>
        <button class="btn reset" on:click={onReset} title="Reset timer (clear current task only)">
          <Icons name="reset" size={18} className="btn-icon" />
          Reset
        </button>
      </div>
    {/if}
  </div>
</div>

<style>
  .timer {
    position: relative;
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 12px;
    padding: 1.5rem 1.25rem;
    box-shadow: var(--shadow);
    text-align: center;
    overflow: hidden;
  }

  .top-nav {
    position: absolute;
    top: 0.75rem;
    right: 0.75rem;
    display: flex;
    gap: 0.5rem;
  }

  .nav-link {
    color: var(--text-muted);
    transition: color 0.15s;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .nav-link:hover {
    color: var(--text);
  }

  .display {
    margin-bottom: 1rem;
  }

  .time {
    font-size: 3rem;
    font-weight: 400;
    font-family: "SF Mono", "Roboto Mono", ui-monospace, monospace;
    color: var(--text);
    margin-bottom: 0.25rem;
    letter-spacing: 0.02em;
    line-height: 1;
    font-variant-numeric: tabular-nums;
  }

  .phase {
    font-size: 0.6875rem;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.1em;
    line-height: 1;
  }

  .task {
    margin: 1rem 0 0.75rem 0;
    padding: 0 0.125rem;
  }

  .task input {
    width: 100%;
    padding: 0.5rem 0.625rem;
    border: 1px solid var(--input-border);
    border-radius: 8px;
    font-size: 0.875rem;
    background: var(--input-bg);
    color: var(--text);
    text-align: center;
    transition: border-color 0.15s, box-shadow 0.15s;
    box-sizing: border-box;
  }

  .task input::placeholder {
    color: var(--input-placeholder);
  }

  .task input:focus {
    outline: none;
    border-color: var(--accent-blue);
    box-shadow: 0 0 0 2px var(--focus-ring);
  }

  .task input.running {
    background: var(--btn-secondary-hover-bg);
    color: var(--text-muted);
    border-color: var(--border);
  }

  .recent-tasks {
    margin-top: 0.5rem;
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.35rem;
    justify-content: center;
  }

  .recent-label {
    font-size: 0.6875rem;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .task-chip {
    padding: 0.25rem 0.5rem;
    border-radius: 6px;
    border: 1px solid var(--border);
    background: var(--bg-card);
    color: var(--text-muted);
    font-size: 0.75rem;
    cursor: pointer;
    transition: background 0.15s, color 0.15s;
  }

  .task-chip:hover {
    background: var(--btn-secondary-hover-bg);
    color: var(--text);
  }

  .status {
    font-size: 0.6875rem;
    color: var(--text-muted);
    margin-bottom: 1rem;
    font-weight: 500;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .duration-selector {
    margin-bottom: 1rem;
  }

  .duration-label {
    display: block;
    font-size: 0.6875rem;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin-bottom: 0.5rem;
  }

  .duration-options {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 0.35rem;
  }

  .duration-btn {
    padding: 0.5rem 0.35rem;
    border: 1px solid var(--border);
    border-radius: 6px;
    font-size: 0.75rem;
    background: var(--bg-card);
    color: var(--text-muted);
    cursor: pointer;
    transition: background 0.15s, color 0.15s, border-color 0.15s;
  }

  .duration-btn:hover {
    background: var(--btn-secondary-hover-bg);
    color: var(--text);
  }

  .duration-btn.active {
    border-color: var(--accent-blue);
    color: var(--accent-blue);
    background: var(--accent-blue-hover);
  }

  .custom-duration {
    margin-top: 0.5rem;
  }

  .custom-input {
    width: 100%;
    padding: 0.5rem 0.625rem;
    border: 1px solid var(--input-border);
    border-radius: 8px;
    font-size: 0.875rem;
    background: var(--input-bg);
    color: var(--text);
    text-align: center;
    box-sizing: border-box;
  }

  .custom-input:focus {
    outline: none;
    border-color: var(--accent-blue);
    box-shadow: 0 0 0 2px var(--focus-ring);
  }

  .custom-input::placeholder {
    color: var(--input-placeholder);
  }

  .buttons {
    display: flex;
    gap: 0.5rem;
    justify-content: center;
  }

  .btn {
    padding: 0.5rem 1rem;
    min-width: 72px;
    border: 1px solid var(--btn-secondary-border);
    border-radius: 8px;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.15s, border-color 0.15s, color 0.15s;
    background: var(--bg-card);
    color: var(--btn-secondary-text);
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.35rem;
  }

  .btn :global(svg) {
    flex-shrink: 0;
  }

  .btn.start {
    background: var(--btn-primary-bg);
    color: var(--btn-primary-text);
    border-color: var(--btn-primary-bg);
  }

  .btn.start:hover {
    background: var(--btn-primary-hover);
    border-color: var(--btn-primary-hover);
  }

  .btn.pause {
    color: var(--accent-blue);
    border-color: var(--accent-blue-border);
  }

  .btn.pause:hover {
    background: var(--accent-blue-hover);
  }

  .btn.reset {
    color: var(--text-muted);
    border-color: var(--border);
  }

  .btn.reset:hover {
    background: var(--btn-secondary-hover-bg);
    color: var(--text);
  }

  .control-buttons {
    display: flex;
    gap: 0.5rem;
  }
</style>