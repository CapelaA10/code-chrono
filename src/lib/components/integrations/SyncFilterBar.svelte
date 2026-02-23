<!-- SyncFilterBar.svelte
     Search input + project/label selects + "hide imported" toggle + refresh button.
     All filter values are two-way bound via props.
     Emits: refresh -->
<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { Search, RefreshCw } from 'lucide-svelte';
  import Dropdown from '$lib/components/Dropdown.svelte';

  export let search: string = '';
  export let filterProject: string = '';
  export let filterLabel: string = '';
  export let hideImported: boolean = true;

  /** Unique project names across all fetched issues. */
  export let allProjects: string[] = [];
  /** Unique label names across all fetched issues. */
  export let allLabels: string[] = [];
  import { strings } from '$lib/i18n/store';

  const dispatch = createEventDispatcher<{ refresh: void }>();
</script>

<div class="filters">
  <!-- Search -->
  <div class="search-row">
    <Search size={13} />
    <input class="search" placeholder="Search…" bind:value={search} />
    <button class="icon-btn sm" on:click={() => dispatch('refresh')} title="Refresh">
      <RefreshCw size={13} />
    </button>
  </div>

  <!-- Dropdowns + toggle -->
  <div class="select-row">
    {#if allProjects.length > 0}
      <div style="width: 130px;">
        <Dropdown
          value={filterProject}
          placeholder={$strings.allProjects}
          options={[{ value: '', label: $strings.allProjects }, ...allProjects.map(p => ({ value: p, label: p }))]}
          on:change={(e) => filterProject = e.detail}
        />
      </div>
    {/if}

    {#if allLabels.length > 0}
      <div style="width: 130px;">
        <Dropdown
          value={filterLabel}
          placeholder={$strings.allLabels}
          options={[{ value: '', label: $strings.allLabels }, ...allLabels.map(l => ({ value: l, label: l }))]}
          on:change={(e) => filterLabel = e.detail}
        />
      </div>
    {/if}

    <label class="mini-toggle">
      <input type="checkbox" bind:checked={hideImported} />
      Hide imported
    </label>
  </div>
</div>

<style>
  .filters {
    padding: 0.625rem 0.875rem;
    border-bottom: 1px solid var(--border);
    display: flex; flex-direction: column; gap: 0.4rem;
    flex-shrink: 0; background: var(--bg-page);
  }

  .search-row {
    display: flex; align-items: center; gap: 0.4rem;
    background: var(--bg-card); border: 1px solid var(--border);
    border-radius: 7px; padding: 0.35rem 0.6rem;
    color: var(--text-muted);
  }

  .search {
    flex: 1; border: none; background: transparent; outline: none;
    font-size: 0.8125rem; color: var(--text); font-family: inherit;
  }

  .select-row {
    display: flex; align-items: center; gap: 0.4rem; flex-wrap: wrap;
  }

  .mini-toggle {
    display: flex; align-items: center; gap: 0.3rem;
    font-size: 0.75rem; color: var(--text-muted); cursor: pointer;
    white-space: nowrap;
  }

  .icon-btn {
    width: 28px; height: 28px; display: flex; align-items: center; justify-content: center;
    border: none; border-radius: 6px; background: none;
    color: var(--text-muted); cursor: pointer; transition: all 0.12s;
  }
  .icon-btn.sm { width: 22px; height: 22px; }
  .icon-btn:hover { background: var(--btn-secondary-hover-bg); color: var(--text); }
</style>
