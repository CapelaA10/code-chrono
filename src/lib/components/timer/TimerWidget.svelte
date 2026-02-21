<!--
  TimerWidget.svelte
  ──────────────────
  Self-contained Pomodoro timer widget.
  Subscribes to the backend "timer-tick" event and exposes
  Play/Pause/Reset controls and a duration selector.
-->
<script lang="ts">
  import { Play, Pause, Square } from 'lucide-svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { timerDuration } from '$lib/stores/timerSettings';
  import { activeTimer } from '$lib/stores/timer';
  import { formatTime } from '$lib/utils/format';

  // ── Derived State ──────────────────────────────────────────────────────────

  // Fallback state if store is not populated yet
  $: timer = $activeTimer || {
    remaining:        $timerDuration * 60,
    paused:           true,
    phase:            0,
    task_active:      false,
    active_task_name: null,
    session_duration: $timerDuration * 60,
    last_activity:    0
  };

  /** Show the selected duration when idle, otherwise the live remaining time */
  $: displaySeconds = (!timer.task_active && timer.remaining === timer.session_duration)
    ? $timerDuration * 60
    : timer.remaining;

  $: phaseLabel = timer.phase === 0 ? 'Work' : 'Break';
  $: isIdle     = !timer.task_active || timer.remaining === 0;

  // ── Actions ────────────────────────────────────────────────────────────────

  async function toggle() {
    if (isIdle) {
      await invoke('start_pomodoro', { taskName: 'General', durationMinutes: $timerDuration });
    } else {
      await invoke('pause_timer');
    }
  }

  async function reset() {
    await invoke('reset_timer');
  }

  // Available durations in minutes
  const DURATIONS = [5, 10, 15, 20, 25, 30, 45, 60, 90, 120];
</script>

<div class="widget" class:running={!timer.paused}>

  <!-- Phase + countdown -->
  <div class="info">
    <span class="phase">{phaseLabel}</span>
    <span class="time">{formatTime(displaySeconds)}</span>
  </div>

  {#if !isIdle && timer.active_task_name}
    <div class="task-name-wrap" title={timer.active_task_name}>
      {timer.active_task_name}
    </div>
  {/if}

  <!-- Duration picker (only while idle) -->
  {#if isIdle}
    <div class="duration-wrap">
      <select bind:value={$timerDuration} class="duration-select" aria-label="Timer duration">
        {#each DURATIONS as d}
          <option value={d}>{d}m</option>
        {/each}
      </select>
    </div>
  {/if}

  <!-- Play/Pause + Stop -->
  <div class="controls">
    <button on:click={toggle} title={timer.paused ? 'Start' : 'Pause'}>
      {#if timer.paused}
        <Play  size={16} fill="currentColor" />
      {:else}
        <Pause size={16} fill="currentColor" />
      {/if}
    </button>
    <button on:click={reset} title="Reset">
      <Square size={14} fill="currentColor" />
    </button>
  </div>

</div>

<style>
  .widget {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.5rem 1rem;
    background: var(--bg-page);
    border: 1px solid var(--border);
    border-radius: 12px;
    transition: all 0.3s;
    flex-shrink: 0;
  }

  /* Highlight border + background while running */
  .widget.running {
    border-color: var(--accent-blue);
    background: var(--accent-blue-hover);
  }

  /* ── Phase + time ── */
  .info {
    display: flex;
    flex-direction: column;
    align-items: center;
    min-width: 58px;
  }

  .phase {
    font-size: 0.6rem;
    text-transform: uppercase;
    font-weight: 700;
    color: var(--text-muted);
    letter-spacing: 0.08em;
  }

  .time {
    font-family: 'SF Mono', 'Roboto Mono', monospace;
    font-size: 1.125rem;
    font-weight: 600;
    color: var(--text);
    line-height: 1.1;
  }

  .widget.running .time { color: var(--accent-blue); }

  /* ── Task Name ── */
  .task-name-wrap {
    border-left: 1px solid var(--border);
    padding: 0 0.75rem;
    font-size: 0.8125rem;
    font-weight: 500;
    color: var(--text);
    max-width: 150px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .widget.running .task-name-wrap {
    color: var(--accent-blue);
    border-color: rgba(59, 130, 246, 0.2); /* var(--accent-blue) with opacity */
  }

  /* ── Duration picker ── */
  .duration-wrap {
    border-left: 1px solid var(--border);
    padding-left: 0.75rem;
  }

  .duration-select {
    background: var(--bg-card);
    color: var(--text);
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 4px 20px 4px 8px;
    font-size: 0.75rem;
    font-weight: 600;
    outline: none;
    cursor: pointer;
    appearance: none;
    /* Custom caret arrow */
    background-image: url("data:image/svg+xml;charset=US-ASCII,%3Csvg xmlns%3D'http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg' width%3D'292' height%3D'292'%3E%3Cpath fill%3D'%239CA3AF' d%3D'M287 69a18 18 0 0 0-13-5H18a18 18 0 0 0-13 18c0 5 2 9 5 13l128 128c4 4 8 5 13 5s9-1 13-5l128-128c4-3 5-8 5-13a18 18 0 0 0-5-13z'%2F%3E%3C%2Fsvg%3E");
    background-repeat: no-repeat;
    background-position: right 6px center;
    background-size: 8px auto;
    transition: border-color 0.2s;
  }

  .duration-select:hover,
  .duration-select:focus { border-color: var(--accent-blue); }

  /* ── Controls ── */
  .controls { display: flex; gap: 0.25rem; }

  .controls button {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 6px;
    border-radius: 6px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s;
  }

  .controls button:hover {
    background: var(--btn-secondary-hover-bg);
    color: var(--text);
  }
</style>
