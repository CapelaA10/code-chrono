<script lang="ts">
  export let totalSessions: number;
  export let totalHours: number;
  export let remainingMinutes: number;
</script>

<div class="summary-cards">
  <div class="card">
    <div class="card-label">Total Sessions</div>
    <div class="card-value">{totalSessions}</div>
  </div>
  <div class="card">
    <div class="card-label">Total Time</div>
    <div class="card-value">
      {#if totalHours > 0}
        {totalHours}h {remainingMinutes}m
      {:else}
        {remainingMinutes}m
      {/if}
    </div>
  </div>
</div>

<style>
  .summary-cards {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1.5rem;
    margin-bottom: 2rem;
  }

  .card {
    padding: 1.5rem;
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 12px;
    box-shadow: var(--shadow-sm);
  }

  .card-label {
    font-size: 0.75rem;
    font-weight: 700;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin-bottom: 0.5rem;
  }

  .card-value {
    font-size: 2rem;
    font-weight: 800;
    color: var(--text);
  }
</style>
