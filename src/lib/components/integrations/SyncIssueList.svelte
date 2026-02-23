<!-- SyncIssueList.svelte
     Select-all bar + scrollable issue rows.
     Emits: toggle(id) — parent manages the selected Set. -->
<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { CheckSquare, Square, CheckCircle2 } from 'lucide-svelte';
  import type { ExternalTask } from './syncTypes';

  /** Filtered subset to display (already processed by parent). */
  export let issues: ExternalTask[] = [];
  /** IDs currently selected for import. */
  export let selected: Set<string> = new Set();

  $: selectable   = issues.filter(i => !i.already_imported);
  $: allSelected  = selectable.length > 0 && selectable.every(i => selected.has(i.id));

  const dispatch = createEventDispatcher<{ toggle: string; toggleAll: void }>();
</script>

<!-- Select-all bar -->
<div class="select-bar">
  <button class="text-btn" on:click={() => dispatch('toggleAll')}>
    {#if allSelected}<CheckSquare size={14} />{:else}<Square size={14} />{/if}
    <span>{allSelected ? 'Deselect all' : 'Select all'}</span>
  </button>
  <span class="sel-count">{selected.size} selected</span>
</div>

<!-- Scrollable list -->
<div class="list">
  {#if issues.length === 0}
    <div class="empty">No issues match the current filters.</div>
  {/if}

  {#each issues as issue (issue.id)}
    <button
      class="row"
      class:is-imported={issue.already_imported}
      class:is-selected={selected.has(issue.id)}
      on:click={() => dispatch('toggle', issue.id)}
      disabled={issue.already_imported}
    >
      <!-- Check state icon -->
      <span class="row-check">
        {#if issue.already_imported}
          <CheckCircle2 size={14} class="ok-icon" />
        {:else if selected.has(issue.id)}
          <CheckSquare size={14} class="sel-icon" />
        {:else}
          <Square size={14} class="idle-icon" />
        {/if}
      </span>

      <!-- Title + chips -->
      <span class="row-body">
        <span class="row-title">{issue.title}</span>
        <span class="row-chips">
          {#if issue.project}
            <span class="chip proj">{issue.project.split('/').pop()}</span>
          {/if}
          {#each issue.labels.slice(0, 2) as l}
            <span class="chip lbl">{l}</span>
          {/each}
        </span>
      </span>

      <!-- Hover-only external link -->
      {#if issue.url}
        <a
          href={issue.url}
          class="ext-link"
          target="_blank"
          rel="noopener"
          on:click|stopPropagation
          title="Open in browser"
        >↗</a>
      {/if}
    </button>
  {/each}
</div>

<style>
  /* ── Select-all bar ─────────────────────────────────────────────────────── */
  .select-bar {
    display: flex; align-items: center; justify-content: space-between;
    padding: 0.4rem 0.875rem;
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
  }

  .text-btn {
    display: flex; align-items: center; gap: 0.35rem;
    border: none; background: none; cursor: pointer; padding: 0;
    font-size: 0.75rem; font-weight: 600; color: var(--accent-blue);
    font-family: inherit;
  }

  .sel-count { font-size: 0.75rem; color: var(--text-muted); }

  /* ── Scrollable list ────────────────────────────────────────────────────── */
  .list { flex: 1; overflow-y: auto; padding: 0.25rem 0; }

  .list::-webkit-scrollbar { width: 5px; }
  .list::-webkit-scrollbar-track { background: transparent; }
  .list::-webkit-scrollbar-thumb { background: var(--border); border-radius: 3px; }

  /* ── Issue row ──────────────────────────────────────────────────────────── */
  .row {
    display: flex; align-items: center; gap: 0.6rem;
    width: 100%; padding: 0.5rem 0.875rem;
    border: none; background: none; text-align: left;
    cursor: pointer; transition: background 0.1s;
    font-family: inherit; color: var(--text);
  }

  .row:hover:not(:disabled) { background: var(--btn-secondary-hover-bg); }
  .row:disabled  { cursor: default; }
  .row.is-imported { opacity: 0.4; }
  .row.is-selected { background: color-mix(in srgb, var(--accent-blue) 6%, transparent); }

  .row-check { flex-shrink: 0; display: flex; color: var(--text-muted); }

  :global(.ok-icon)   { color: var(--accent-green, #10b981); }
  :global(.sel-icon)  { color: var(--accent-blue); }
  :global(.idle-icon) { color: var(--text-muted); }

  .row-body { flex: 1; min-width: 0; }

  .row-title {
    display: block; font-size: 0.8125rem; font-weight: 500; color: var(--text);
    white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
  }

  .row-chips { display: flex; gap: 0.25rem; margin-top: 0.2rem; flex-wrap: wrap; }

  .chip {
    font-size: 0.62rem; font-weight: 600; padding: 0.1rem 0.4rem;
    border-radius: 99px; letter-spacing: 0.02em;
  }
  .chip.proj { background: color-mix(in srgb, var(--accent-blue) 12%, transparent); color: var(--accent-blue); }
  .chip.lbl  { background: color-mix(in srgb, var(--accent-green) 12%, transparent); color: var(--accent-green); }

  .ext-link {
    flex-shrink: 0; font-size: 0.8rem; color: var(--text-muted);
    text-decoration: none; padding: 0.1rem 0.25rem; border-radius: 4px;
    transition: color 0.12s; opacity: 0;
  }
  .row:hover .ext-link { opacity: 1; }
  .ext-link:hover { color: var(--accent-blue); }

  .empty {
    text-align: center; padding: 2rem 1rem;
    color: var(--text-muted); font-size: 0.8125rem;
  }
</style>
