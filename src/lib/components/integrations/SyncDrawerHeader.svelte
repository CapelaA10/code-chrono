<!-- SyncDrawerHeader.svelte
     Displays the source badge, title, new/imported count badges, and close button.
     Emits: close -->
<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { X } from 'lucide-svelte';
  import { strings } from '$lib/i18n/store';

  /** "GitHub" | "GitLab" | "Jira" */
  export let source: string;
  export let newCount: number = 0;
  export let importedTotal: number = 0;
  export let showCounts: boolean = false;   // only after a successful fetch

  const dispatch = createEventDispatcher<{ close: void }>();
</script>

<div class="drawer-header">
  <div class="header-left">
    <span class="source-tag">{source}</span>
    <span class="header-title">{$strings.importIssues}</span>
  </div>
  <div class="header-right">
    {#if showCounts}
      <span class="count-badge">{newCount} new</span>
      {#if importedTotal > 0}
        <span class="already-badge">{importedTotal} done</span>
      {/if}
    {/if}
    <button class="icon-btn" on:click={() => dispatch('close')} title="Close">
      <X size={16} />
    </button>
  </div>
</div>

<style>
  .drawer-header {
    display: flex; align-items: center; justify-content: space-between;
    padding: 0.875rem 1rem;
    border-bottom: 1px solid var(--border);
    flex-shrink: 0; gap: 0.5rem;
  }

  .header-left  { display: flex; align-items: center; gap: 0.5rem; min-width: 0; }
  .header-right { display: flex; align-items: center; gap: 0.4rem; flex-shrink: 0; }

  .source-tag {
    font-size: 0.65rem; font-weight: 700; text-transform: uppercase; letter-spacing: 0.08em;
    padding: 0.2rem 0.55rem; border-radius: 99px; flex-shrink: 0;
    background: var(--accent-blue-hover, #3b82f615);
    color: var(--accent-blue);
    border: 1px solid var(--accent-blue-border, #3b82f630);
  }

  .header-title { font-size: 0.9rem; font-weight: 700; color: var(--text); white-space: nowrap; }

  .count-badge, .already-badge {
    font-size: 0.68rem; font-weight: 600; padding: 0.15rem 0.45rem; border-radius: 99px;
  }
  .count-badge   { background: rgba(59,130,246,0.1); color: var(--accent-blue); }
  .already-badge { background: rgba(107,114,128,0.1); color: var(--text-muted); }

  .icon-btn {
    width: 28px; height: 28px; display: flex; align-items: center; justify-content: center;
    border: none; border-radius: 6px; background: none;
    color: var(--text-muted); cursor: pointer; transition: all 0.12s;
  }
  .icon-btn:hover { background: var(--btn-secondary-hover-bg); color: var(--text); }
</style>
