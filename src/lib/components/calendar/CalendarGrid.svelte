<!--
  CalendarGrid.svelte
  ───────────────────
  7-column CSS grid for one month.
  Props:
    year:  number
    month: number (0-indexed)
    tasks: Task[]
-->
<script lang="ts">
  import type { Task } from '$lib/types';
  import CalendarCell from './CalendarCell.svelte';
  import { locale } from '$lib/i18n/store';

  export let year:  number;
  export let month: number;
  export let tasks: Task[] = [];

  $: WEEKDAYS = Array.from({ length: 7 }, (_, i) => {
    const d = new Date(2024, 0, i + 1); // 2024-01-01 is Monday
    return d.toLocaleDateString($locale, { weekday: 'short' });
  });

  /** Build the grid: leading padding cells + actual month days + trailing padding. */
  $: cells = (() => {
    const firstDay = new Date(year, month, 1);
    // Mon-based: Sunday becomes 6, Monday becomes 0
    const startPad = (firstDay.getDay() + 6) % 7;
    const daysInMonth = new Date(year, month + 1, 0).getDate();
    const total = startPad + daysInMonth;
    const endPad = (7 - (total % 7)) % 7;

    const result: { date: Date; faded: boolean }[] = [];

    for (let i = startPad - 1; i >= 0; i--) {
      result.push({ date: new Date(year, month, -i), faded: true });
    }
    for (let d = 1; d <= daysInMonth; d++) {
      result.push({ date: new Date(year, month, d), faded: false });
    }
    for (let d = 1; d <= endPad; d++) {
      result.push({ date: new Date(year, month + 1, d), faded: true });
    }
    return result;
  })();

  const todayStr = new Date().toDateString();

  /** Tasks that have a due_date matching the given date. */
  function tasksForDay(date: Date): Task[] {
    return tasks.filter(t => {
      if (!t.due_date) return false;
      return new Date(t.due_date * 1000).toDateString() === date.toDateString();
    });
  }
</script>

<div class="grid-wrapper">
  <!-- Weekday headers -->
  <div class="day-headers">
    {#each WEEKDAYS as wd}
      <div class="weekday">{wd}</div>
    {/each}
  </div>

  <!-- Day cells -->
  <div class="grid">
    {#each cells as { date, faded } (date.toISOString())}
      <CalendarCell
        {date}
        {faded}
        today={date.toDateString() === todayStr}
        tasks={tasksForDay(date)}
      />
    {/each}
  </div>
</div>

<style>
  .day-headers {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    gap: 0.5rem;
    margin-bottom: 0.5rem;
  }

  .weekday {
    text-align: center;
    font-size: 0.6875rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.07em;
    color: var(--text-muted);
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(7, 1fr);
    gap: 0.5rem;
  }
</style>
