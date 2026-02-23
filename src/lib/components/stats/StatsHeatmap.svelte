<!--
  StatsHeatmap.svelte
  ───────────────────
  Weekly activity heatmap: 12 weeks × 7 days grid.
  Each cell is colored by total minutes tracked that day.
  Props:
    dailyStats: DailyStat[]
-->
<script lang="ts">
  import type { DailyStat } from '$lib/types';
  import { strings } from '$lib/i18n/store';

  export let dailyStats: DailyStat[];

  /** Build a lookup map: ISO date → total seconds */
  function buildDayMap(stats: DailyStat[]): Map<string, number> {
    const map = new Map<string, number>();
    for (const s of stats) {
      map.set(s.day, (map.get(s.day) ?? 0) + s.total_seconds);
    }
    return map;
  }

  /** Return the ISO date string (YYYY-MM-DD) for a given offset from today. */
  function dateOffset(daysBack: number): string {
    const d = new Date();
    d.setDate(d.getDate() - daysBack);
    return d.toISOString().split('T')[0];
  }

  const WEEKS = 12;
  const DAYS  = WEEKS * 7;
  const DAY_LABELS = ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun'];

  /** Build grid cells: oldest first, 84 entries. */
  $: dayMap  = buildDayMap(dailyStats);
  $: maxSecs = Math.max(...Array.from(dayMap.values()), 1);
  $: cells   = Array.from({ length: DAYS }, (_, i) => {
    const date  = dateOffset(DAYS - 1 - i);
    const secs  = dayMap.get(date) ?? 0;
    const level = secs === 0 ? 0 : Math.ceil((secs / maxSecs) * 4);
    return { date, secs, level };
  });

  function tooltip(date: string, secs: number): string {
    if (secs === 0) return date;
    const m = Math.floor(secs / 60);
    const h = Math.floor(m / 60);
    const label = h > 0 ? `${h}h ${m % 60}m` : `${m}m`;
    return `${date} · ${label}`;
  }
</script>

<section class="heatmap-section">
  <h2 class="section-title">{$strings.heatmap}</h2>

  <div class="heatmap-wrap">
    <!-- Row labels Mon–Sun -->
    <div class="day-labels">
      {#each DAY_LABELS as d}
        <span>{d}</span>
      {/each}
    </div>

    <!-- Grid: 7 rows × 12 columns -->
    <div class="grid">
      {#each cells as cell (cell.date)}
        <div
          class="cell level-{cell.level}"
          title={tooltip(cell.date, cell.secs)}
          aria-label={tooltip(cell.date, cell.secs)}
        ></div>
      {/each}
    </div>
  </div>

  <div class="legend">
    <span class="legend-label">Less</span>
    {#each [0,1,2,3,4] as l}
      <div class="cell level-{l}"></div>
    {/each}
    <span class="legend-label">More</span>
  </div>
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

  .heatmap-wrap {
    display: flex;
    gap: 0.375rem;
    align-items: flex-start;
  }

  .day-labels {
    display: flex;
    flex-direction: column;
    gap: 3px;
    padding-top: 2px;
  }

  .day-labels span {
    font-size: 0.625rem;
    color: var(--text-muted);
    line-height: 11px;
    height: 11px;
    display: flex;
    align-items: center;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(12, 11px);
    grid-template-rows: repeat(7, 11px);
    gap: 3px;
    grid-auto-flow: column;
  }

  .cell {
    width: 11px;
    height: 11px;
    border-radius: 2px;
    cursor: default;
  }

  /* Level 0 = no activity */
  .level-0 { background: var(--bg-card); border: 1px solid var(--border); }
  .level-1 { background: color-mix(in srgb, var(--accent-blue) 20%, transparent); }
  .level-2 { background: color-mix(in srgb, var(--accent-blue) 45%, transparent); }
  .level-3 { background: color-mix(in srgb, var(--accent-blue) 70%, transparent); }
  .level-4 { background: var(--accent-blue); }

  .legend {
    display: flex;
    align-items: center;
    gap: 3px;
    margin-top: 0.5rem;
  }

  .legend-label {
    font-size: 0.625rem;
    color: var(--text-muted);
    margin: 0 4px;
  }
</style>
