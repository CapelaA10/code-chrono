<!--
  StatsTimeByTask.svelte
  ──────────────────────
  Horizontal bar chart showing time tracked per task for the selected period.
-->
<script lang="ts">
  import type { TaskStat } from '$lib/types';
  import { formatDuration } from '$lib/utils/format';

  export let stats: TaskStat[];

  /**
   * Returns the bar width percentage for a task relative to the busiest task.
   * Avoids division-by-zero when all values are 0.
   */
  function barWidth(seconds: number): number {
    const max = Math.max(...stats.map(s => s.total_seconds), 1);
    return (seconds / max) * 100;
  }
</script>

<section>
  <h2 class="section-title">Time by Task</h2>

  {#if stats.length === 0}
    <p class="empty">No sessions recorded for this period.</p>
  {:else}
    <ul class="task-list">
      {#each stats as stat (stat.task_name)}
        <li class="task-entry">
          <div class="row">
            <span class="task-name">{stat.task_name}</span>
            <span class="task-duration">{formatDuration(stat.total_seconds)}</span>
          </div>
          <div class="bar-track">
            <div class="bar-fill" style="width: {barWidth(stat.total_seconds)}%"></div>
          </div>
          <span class="sessions">
            {stat.sessions} session{stat.sessions !== 1 ? 's' : ''}
          </span>
        </li>
      {/each}
    </ul>
  {/if}
</section>

<style>
  .section-title {
    font-size: 0.75rem;
    font-weight: 700;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.1em;
    margin: 0 0 1rem;
  }

  .task-list {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
  }

  .task-entry {
    background: var(--bg-card);
    padding: 1rem;
    border-radius: 12px;
    border: 1px solid var(--border);
  }

  .row {
    display: flex;
    justify-content: space-between;
    margin-bottom: 0.75rem;
  }

  .task-name   { font-weight: 600; color: var(--text); }
  .task-duration { font-weight: 700; color: var(--accent-blue); }

  .bar-track {
    height: 8px;
    background: var(--bg-page);
    border-radius: 4px;
    overflow: hidden;
    margin-bottom: 0.5rem;
  }

  .bar-fill {
    height: 100%;
    background: var(--accent-blue);
    border-radius: 4px;
    transition: width 0.4s ease;
  }

  .sessions {
    font-size: 0.75rem;
    color: var(--text-muted);
  }

  .empty {
    color: var(--text-muted);
    text-align: center;
    padding: 2rem;
    margin: 0;
  }
</style>
