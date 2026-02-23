<!--
  CalendarCell.svelte
  ───────────────────
  Single day cell: day number + up to 3 task-title pills.
  Props:
    date:    Date
    tasks:   Task[]
    today:   boolean
-->
<script lang="ts">
  import type { Task } from '$lib/types';
  import { isOverdue } from '$lib/utils/format';

  export let date:  Date;
  export let tasks: Task[] = [];
  export let today  = false;
  export let faded  = false;

  const MAX_VISIBLE = 3;
  $: visible  = tasks.slice(0, MAX_VISIBLE);
  $: overflow = tasks.length - MAX_VISIBLE;
</script>

<div class="cell" class:today class:faded>
  <span class="day-number">{date.getDate()}</span>

  {#each visible as task (task.id)}
    <div
      class="task-pill"
      class:overdue={task.due_date != null && isOverdue(task.due_date)}
      title={task.title}
    >
      {task.title}
    </div>
  {/each}

  {#if overflow > 0}
    <div class="overflow-pill">+{overflow} more</div>
  {/if}
</div>

<style>
  .cell {
    min-height: 88px;
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 10px;
    padding: 0.5rem;
    display: flex;
    flex-direction: column;
    gap: 3px;
    transition: border-color 0.15s;
  }

  .cell:hover { border-color: var(--accent-blue); }
  .cell.today { border-color: var(--accent-blue); background: var(--accent-blue-hover); }
  .cell.faded { opacity: 0.35; }

  .day-number {
    font-size: 0.75rem;
    font-weight: 700;
    color: var(--text-muted);
    margin-bottom: 2px;
  }

  .cell.today .day-number {
    color: var(--accent-blue);
  }

  .task-pill {
    font-size: 0.65rem;
    font-weight: 500;
    background: color-mix(in srgb, var(--accent-blue) 12%, transparent);
    color: var(--accent-blue);
    border-radius: 4px;
    padding: 1px 5px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .task-pill.overdue {
    background: color-mix(in srgb, var(--error-red) 12%, transparent);
    color: var(--error-red);
  }

  .overflow-pill {
    font-size: 0.6rem;
    color: var(--text-muted);
    padding: 1px 5px;
  }
</style>
