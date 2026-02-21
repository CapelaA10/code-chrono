<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import { ArrowLeft, Download } from 'lucide-svelte';
  import type { TaskStat, DailyStat } from '$lib/types';
  import StatsSummary       from '$lib/components/stats/StatsSummary.svelte';
  import StatsTimeByTask    from '$lib/components/stats/StatsTimeByTask.svelte';
  import StatsDailyBreakdown from '$lib/components/stats/StatsDailyBreakdown.svelte';

  // ── Types ─────────────────────────────────────────────────────────────────

  type FilterPeriod = 'today' | 'week' | 'month' | 'custom';

  // ── State ─────────────────────────────────────────────────────────────────

  let stats:      TaskStat[]  = [];
  let dailyStats: DailyStat[] = [];
  let loading = false;
  let selectedPeriod: FilterPeriod = 'today';
  let customStartDate = '';
  let customEndDate   = '';

  // ── Derived ───────────────────────────────────────────────────────────────

  $: totalSeconds  = stats.reduce((sum, s) => sum + s.total_seconds, 0);
  $: totalMinutes  = Math.floor(totalSeconds / 60);
  $: totalHours    = Math.floor(totalMinutes / 60);
  $: remainingMins = totalMinutes % 60;
  $: totalSessions = stats.reduce((sum, s) => sum + s.sessions, 0);

  $: dailyGroups = dailyStats.reduce<Record<string, DailyStat[]>>((acc, stat) => {
    (acc[stat.day] ??= []).push(stat);
    return acc;
  }, {});

  // ── Helpers ───────────────────────────────────────────────────────────────

  /** Returns [startUnix, endUnix] for the selected period. */
  function getTimestampRange(): [number, number] {
    const now = new Date();
    const endOfToday = new Date(now.getFullYear(), now.getMonth(), now.getDate(), 23, 59, 59);
    const end = Math.floor(endOfToday.getTime() / 1000);

    switch (selectedPeriod) {
      case 'today': {
        const start = new Date(now.getFullYear(), now.getMonth(), now.getDate());
        return [Math.floor(start.getTime() / 1000), end];
      }
      case 'week': {
        const start = new Date(now);
        start.setDate(now.getDate() - now.getDay());
        start.setHours(0, 0, 0, 0);
        return [Math.floor(start.getTime() / 1000), end];
      }
      case 'month': {
        const start = new Date(now.getFullYear(), now.getMonth(), 1);
        return [Math.floor(start.getTime() / 1000), end];
      }
      case 'custom': {
        if (!customStartDate || !customEndDate) {
          return [Math.floor(new Date(now.getFullYear(), now.getMonth(), now.getDate()).getTime() / 1000), end];
        }
        // Use local midnight by appending T00:00:00 to avoid UTC parse
        const s = new Date(`${customStartDate}T00:00:00`);
        const e = new Date(`${customEndDate}T23:59:59`);
        return [Math.floor(s.getTime() / 1000), Math.floor(e.getTime() / 1000)];
      }
    }
  }

  // ── Data loading ──────────────────────────────────────────────────────────

  async function loadStats() {
    loading = true;
    try {
      const [startTimestamp, endTimestamp] = getTimestampRange();
      [stats, dailyStats] = await Promise.all([
        invoke<TaskStat[]>('get_task_stats',      { startTimestamp, endTimestamp }),
        invoke<DailyStat[]>('get_daily_breakdown', { startTimestamp, endTimestamp })
      ]);
    } catch (err) {
      console.error('Failed to load stats:', err);
      stats = [];
      dailyStats = [];
    } finally {
      loading = false;
    }
  }

  function setPeriod(p: FilterPeriod) {
    selectedPeriod = p;
    loadStats();
  }

  async function exportCsv() {
    try {
      const csv  = await invoke<string>('export_csv');
      const blob = new Blob([csv], { type: 'text/csv' });
      const url  = URL.createObjectURL(blob);
      const a    = Object.assign(document.createElement('a'), {
        href:     url,
        download: `chrono-stats-${new Date().toISOString().split('T')[0]}.csv`
      });
      a.click();
      URL.revokeObjectURL(url); // clean up the blob URL
    } catch (err) {
      console.error('CSV export failed:', err);
    }
  }

  onMount(loadStats);
</script>

<main class="stats-page">
  <div class="stats-container">
    <div class="stats-header">
      <div class="header-left">
        <a href="/" class="back-link">
          <ArrowLeft size={20} />
          <span>Back</span>
        </a>
        <h1 class="title">Statistics</h1>
      </div>
      <button class="export-btn" on:click={exportCsv}>
        <Download size={18} />
        <span>Export CSV</span>
      </button>
    </div>

    <section class="filter-section">
      <div class="filter-group">
        {#each ['today', 'week', 'month', 'custom'] as period}
          <button 
            class="filter-tab" 
            class:active={selectedPeriod === period}
            on:click={() => setPeriod(period as FilterPeriod)}
          >
            {period.charAt(0).toUpperCase() + period.slice(1)}
          </button>
        {/each}
      </div>

      {#if selectedPeriod === "custom"}
        <div class="custom-range">
          <input type="date" bind:value={customStartDate} class="date-input" />
          <span class="separator">to</span>
          <input type="date" bind:value={customEndDate} class="date-input" />
          <button class="apply-btn" on:click={loadStats}>Apply</button>
        </div>
      {/if}
    </section>

    {#if loading}
      <div class="loading-state">Loading records...</div>
    {:else}
      <StatsSummary {totalSessions} {totalHours} remainingMinutes={remainingMins} />
      <div class="stats-grid">
        <StatsTimeByTask {stats} />
        <StatsDailyBreakdown {dailyGroups} />
      </div>
    {/if}
  </div>
</main>

<style>
  .stats-page {
    flex: 1;
    overflow-y: auto;
    background: var(--bg-page);
    padding: 2rem;
  }

  .stats-container {
    max-width: 1000px;
    margin: 0 auto;
  }

  .stats-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2.5rem;
  }

  .header-left {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .back-link {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: var(--text-muted);
    text-decoration: none;
    font-size: 0.875rem;
    font-weight: 500;
    transition: color 0.15s;
  }

  .back-link:hover {
    color: var(--text);
  }

  .title {
    font-size: 2rem;
    font-weight: 800;
    margin: 0;
  }

  .export-btn {
    display: flex;
    align-items: center;
    gap: 0.625rem;
    padding: 0.625rem 1.25rem;
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 10px;
    color: var(--text);
    font-size: 0.875rem;
    font-weight: 600;
    cursor: pointer;
    transition: var(--transition);
  }

  .export-btn:hover {
    border-color: var(--accent-blue);
    background: var(--accent-blue-hover);
    color: var(--accent-blue);
  }

  .filter-section {
    margin-bottom: 2rem;
  }

  .filter-group {
    display: flex;
    gap: 0.5rem;
    background: var(--bg-card);
    padding: 0.35rem;
    border-radius: 12px;
    border: 1px solid var(--border);
    width: fit-content;
  }

  .filter-tab {
    padding: 0.5rem 1.25rem;
    border: none;
    background: none;
    border-radius: 8px;
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--text-muted);
    cursor: pointer;
    transition: all 0.2s;
  }

  .filter-tab:hover {
    color: var(--text);
  }

  .filter-tab.active {
    background: var(--accent-blue);
    color: white;
  }

  .custom-range {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin-top: 1rem;
  }

  .date-input {
    background: var(--bg-card);
    border: 1px solid var(--border);
    color: var(--text);
    padding: 0.5rem 0.75rem;
    border-radius: 8px;
    font-family: inherit;
  }

  .apply-btn {
    background: var(--accent-blue);
    color: white;
    border: none;
    padding: 0.5rem 1.25rem;
    border-radius: 8px;
    font-weight: 600;
    cursor: pointer;
  }

  .loading-state {
    text-align: center;
    padding: 4rem;
    color: var(--text-muted);
    font-weight: 500;
  }

  .stats-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 2rem;
  }

  @media (max-width: 900px) {
    .stats-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
