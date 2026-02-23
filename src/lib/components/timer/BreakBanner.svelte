<!--
  BreakBanner.svelte
  ──────────────────
  Dismissible pill that appears after a work session finishes,
  offering to start a short or long break.
  Props: none — reads from the activeTimer store.
-->
<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { activeTimer } from '$lib/stores/timer';
  import { strings } from '$lib/i18n/store';

  let dismissed = false;

  /** Show banner when work phase just completed (task_active = false, phase = 0). */
  $: show = !dismissed
    && $activeTimer !== null
    && !$activeTimer.task_active
    && $activeTimer.phase === 0
    && $activeTimer.remaining === 0;

  $: if ($activeTimer?.task_active) dismissed = false;

  async function startBreak(minutes: number, phase: number) {
    dismissed = true;
    await invoke('start_break', { durationMinutes: minutes, phase });
  }
</script>

{#if show}
  <div class="banner">
    <span class="emoji">🎉</span>
    <span class="label">Session done!</span>
    <button class="break-btn short" on:click={() => startBreak(5, 1)}>
      {$strings.startShortBreak}
    </button>
    <button class="break-btn long" on:click={() => startBreak(15, 2)}>
      {$strings.startLongBreak}
    </button>
    <button class="skip-btn" on:click={() => dismissed = true}>
      {$strings.skipBreak}
    </button>
  </div>
{/if}

<style>
  .banner {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 0.875rem;
    background: color-mix(in srgb, var(--accent-blue) 10%, var(--bg-card));
    border: 1px solid var(--accent-blue);
    border-radius: 99px;
    font-size: 0.8125rem;
    animation: slideDown 0.3s cubic-bezier(0.16, 1, 0.3, 1);
    flex-wrap: wrap;
  }

  .emoji { font-size: 1rem; }

  .label {
    font-weight: 600;
    color: var(--text);
    margin-right: 0.25rem;
  }

  .break-btn {
    padding: 0.25rem 0.75rem;
    border: none;
    border-radius: 99px;
    font-size: 0.75rem;
    font-weight: 600;
    cursor: pointer;
    font-family: inherit;
    transition: opacity 0.15s;
  }

  .break-btn:hover { opacity: 0.85; }

  .break-btn.short {
    background: var(--accent-blue);
    color: white;
  }

  .break-btn.long {
    background: color-mix(in srgb, var(--accent-blue) 20%, transparent);
    color: var(--accent-blue);
  }

  .skip-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    font-size: 0.75rem;
    cursor: pointer;
    font-family: inherit;
    padding: 0.25rem;
    transition: color 0.15s;
  }

  .skip-btn:hover { color: var(--text); }

  @keyframes slideDown {
    from { transform: translateY(-8px); opacity: 0; }
    to   { transform: translateY(0);    opacity: 1; }
  }
</style>
