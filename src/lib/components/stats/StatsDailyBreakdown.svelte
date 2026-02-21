<!--
  StatsDailyBreakdown.svelte
  ──────────────────────────
  Shows each day's task sessions grouped by date.
-->
<script lang="ts">
  import type { DailyStat } from '$lib/types';
  import { formatDuration, formatDayLabel } from '$lib/utils/format';

  /** Pre-grouped by the parent page: Record<"YYYY-MM-DD", DailyStat[]> */
  export let dailyGroups: Record<string, DailyStat[]>;

  $: days = Object.entries(dailyGroups);
</script>

{#if days.length > 0}
  <section>
    <h2 class="section-title">Daily Breakdown</h2>

    <ul class="day-list">
      {#each days as [day, dayStats] (day)}
        <li class="day-card">
          <div class="day-header">{formatDayLabel(day)}</div>
          <ul class="task-list">
            {#each dayStats as stat (stat.task_name)}
              <li class="task-row">
                <span class="task-name">{stat.task_name}</span>
                <span class="task-time">{formatDuration(stat.total_seconds)}</span>
              </li>
            {/each}
          </ul>
        </li>
      {/each}
    </ul>
  </section>
{/if}

<style>
  .section-title {
    font-size: 0.75rem;
    font-weight: 700;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.1em;
    margin: 0 0 1rem;
  }

  .day-list, .task-list {
    list-style: none;
    margin: 0;
    padding: 0;
  }

  .day-list { display: flex; flex-direction: column; gap: 1rem; }

  .day-card {
    background: var(--bg-card);
    padding: 1.25rem;
    border-radius: 12px;
    border: 1px solid var(--border);
  }

  .day-header {
    font-size: 0.75rem;
    font-weight: 700;
    color: var(--text-muted);
    text-transform: uppercase;
    margin-bottom: 0.75rem;
  }

  .task-list { display: flex; flex-direction: column; gap: 0.5rem; }

  .task-row {
    display: flex;
    justify-content: space-between;
    font-size: 0.875rem;
  }

  .task-name  { color: var(--text); }
  .task-time  { color: var(--text-muted); font-weight: 500; }
</style>
