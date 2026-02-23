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
  import Dropdown from '$lib/components/Dropdown.svelte';

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
    <div class="duration-wrap" style="min-width: 80px;">
      <Dropdown
        value={$timerDuration}
        options={DURATIONS.map(d => ({ value: d, label: d + 'm' }))}
        on:change={(e) => $timerDuration = e.detail}
      />
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
