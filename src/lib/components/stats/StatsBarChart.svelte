<!--
  StatsBarChart.svelte
  ────────────────────
  Inline SVG bar chart showing daily total tracked time.
  Props:
    dailyStats: DailyStat[]
-->
<script lang="ts">
  import type { DailyStat } from '$lib/types';
  import { formatDuration } from '$lib/utils/format';

  export let dailyStats: DailyStat[];

  const CHART_H = 80;
  const BAR_W   = 28;
  const GAP     = 6;

  /** Aggregate seconds per day (multiple tasks per day). */
  $: dayTotals = (() => {
    const map = new Map<string, number>();
    for (const s of dailyStats) {
      map.set(s.day, (map.get(s.day) ?? 0) + s.total_seconds);
    }
    return Array.from(map.entries())
      .sort(([a], [b]) => a.localeCompare(b))
      .map(([day, secs]) => ({ day, secs }));
  })();

  $: maxSecs = Math.max(...dayTotals.map(d => d.secs), 1);
  $: svgW    = dayTotals.length * (BAR_W + GAP);

  function barH(secs: number): number {
    return Math.max(4, (secs / maxSecs) * CHART_H);
  }

  function shortLabel(dateStr: string): string {
    return new Date(`${dateStr}T00:00:00`).toLocaleDateString(undefined, { weekday: 'short' });
  }

  function xPos(i: number): number {
    return i * (BAR_W + GAP);
  }
</script>

{#if dayTotals.length > 0}
  <section>
    <h2 class="section-title">Daily Totals</h2>

    <div class="chart-wrap">
      <svg
        width={svgW}
        height={CHART_H + 28}
        viewBox="0 0 {svgW} {CHART_H + 28}"
        aria-label="Daily totals bar chart"
        role="img"
      >
        {#each dayTotals as { day, secs }, i}
          {@const bh = barH(secs)}
          {@const x  = xPos(i)}
          {@const y  = CHART_H - bh}

          <g>
            <!-- Bar -->
            <rect
              x={x} y={y}
              width={BAR_W} height={bh}
              rx="4"
              class="bar"
            >
              <title>{day}: {formatDuration(secs)}</title>
            </rect>

            <!-- Day label -->
            <text
              x={x + BAR_W / 2}
              y={CHART_H + 14}
              text-anchor="middle"
              class="axis-label"
            >
              {shortLabel(day)}
            </text>
          </g>
        {/each}
      </svg>
    </div>
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

  .chart-wrap {
    overflow-x: auto;
    padding-bottom: 0.25rem;
  }

  .bar {
    fill: var(--accent-blue);
    opacity: 0.85;
    transition: opacity 0.2s;
  }

  .bar:hover { opacity: 1; }

  .axis-label {
    fill: var(--text-muted);
    font-size: 9px;
    font-family: inherit;
  }
</style>
