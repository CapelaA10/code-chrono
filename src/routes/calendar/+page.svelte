<script lang="ts">
  import { onMount } from 'svelte';
  import { ArrowLeft } from 'lucide-svelte';
  import type { Task } from '$lib/types';
  import { tasks, refreshTasks } from '$lib/stores/tasks';
  import CalendarHeader from '$lib/components/calendar/CalendarHeader.svelte';
  import CalendarGrid   from '$lib/components/calendar/CalendarGrid.svelte';
  import { strings } from '$lib/i18n/store';

  const now   = new Date();
  let year    = now.getFullYear();
  let month   = now.getMonth();

  let tasksWithDue: Task[] = [];

  $: tasksWithDue = $tasks.filter(t => t.due_date != null);

  function prevMonth() {
    if (month === 0) { month = 11; year -= 1; }
    else month -= 1;
  }

  function nextMonth() {
    if (month === 11) { month = 0; year += 1; }
    else month += 1;
  }

  onMount(refreshTasks);
</script>

<main class="cal-page">
  <div class="cal-container">
    <a href="/" class="back-link">
      <ArrowLeft size={18} />
      <span>{$strings.back}</span>
    </a>

    <h1 class="page-title">{$strings.calendarTitle}</h1>

    <CalendarHeader
      {year}
      {month}
      on:prev={prevMonth}
      on:next={nextMonth}
    />

    <CalendarGrid
      {year}
      {month}
      tasks={tasksWithDue}
    />
  </div>
</main>

<style>
  .cal-page {
    flex: 1;
    overflow-y: auto;
    background: var(--bg-page);
    padding: 2rem;
  }

  .cal-container { max-width: 1000px; margin: 0 auto; }

  .back-link {
    display: inline-flex;
    align-items: center;
    gap: 0.4rem;
    color: var(--text-muted);
    text-decoration: none;
    font-size: 0.875rem;
    font-weight: 500;
    margin-bottom: 1rem;
    transition: color 0.15s;
  }
  .back-link:hover { color: var(--text); }

  .page-title {
    font-size: 2rem;
    font-weight: 800;
    margin: 0 0 1.5rem;
  }
</style>
