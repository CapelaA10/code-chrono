<script lang="ts">
  import { filterProject, filterTag, filterStatus, projects, tags, refreshTasks } from '$lib/stores/tasks';
  import { X, Filter } from 'lucide-svelte';
  import { strings } from '$lib/i18n/store';
  import Dropdown from '$lib/components/Dropdown.svelte';

  const STATUSES = [
    { value: 'todo',  label: 'To Do' },
    { value: 'doing', label: 'In Progress' },
    { value: 'done',  label: 'Done' },
  ];

  // Local reactive copies for the selects
  let selectedProject: number | null = null;
  let selectedTag: number | null = null;
  let selectedStatus: string | null = null;

  // Sync local → store → reload
  function applyProject(val: number | null) {
    selectedProject = val;
    filterProject.set(val);
    refreshTasks();
  }

  function applyTag(val: number | null) {
    selectedTag = val;
    filterTag.set(val);
    refreshTasks();
  }

  function applyStatus(val: string | null) {
    selectedStatus = val;
    filterStatus.set(val);
    refreshTasks();
  }

  function clearAll() {
    selectedProject = null;
    selectedTag = null;
    selectedStatus = null;
    filterProject.set(null);
    filterTag.set(null);
    filterStatus.set(null);
    refreshTasks();
  }

  $: hasFilter = selectedProject !== null || selectedTag !== null || selectedStatus !== null;

  // Active filter pills
  $: activeProjectName = selectedProject !== null
    ? $projects.find(p => p.id === selectedProject)?.name
    : null;

  $: activeTagName = selectedTag !== null
    ? $tags.find(t => t.id === selectedTag)?.name
    : null;

  $: activeStatusLabel = selectedStatus !== null
    ? STATUSES.find(s => s.value === selectedStatus)?.label
    : null;
</script>

<div class="filter-bar">
  <div class="filter-icon-wrap">
    <Filter size={14} />
    <span class="filter-label">{$strings.filter}</span>
  </div>

  <!-- Project select -->
  {#if $projects.length > 0}
    <div class="dropdown-container">
      <Dropdown
        value={selectedProject ?? ''}
        placeholder={$strings.allProjects}
        options={[
          { value: '', label: $strings.allProjects },
          ...$projects.map(p => ({ value: p.id, label: p.name }))
        ]}
        on:change={(e) => applyProject(e.detail === '' ? null : Number(e.detail))}
      />
    </div>
  {/if}

  <!-- Tag select -->
  {#if $tags.length > 0}
    <div class="dropdown-container">
      <Dropdown
        value={selectedTag ?? ''}
        placeholder={$strings.allTags}
        options={[
          { value: '', label: $strings.allTags },
          ...$tags.map(t => ({ value: t.id, label: '#' + t.name }))
        ]}
        on:change={(e) => applyTag(e.detail === '' ? null : Number(e.detail))}
      />
    </div>
  {/if}

  <!-- Status select -->
  <div class="dropdown-container status">
    <Dropdown
      value={selectedStatus ?? ''}
      placeholder={$strings.allStatuses}
      options={[
        { value: '', label: $strings.allStatuses },
        ...STATUSES.map(s => ({ value: s.value, label: s.label }))
      ]}
      on:change={(e) => applyStatus(e.detail === '' ? null : e.detail)}
    />
  </div>

  <!-- Active filter pills -->
  {#if hasFilter}
    <div class="active-pills">
      {#if activeProjectName}
        <button class="pill project-pill" on:click={() => applyProject(null)}>
          {activeProjectName} <X size={11} />
        </button>
      {/if}
      {#if activeTagName}
        <button class="pill tag-pill" on:click={() => applyTag(null)}>
          #{activeTagName} <X size={11} />
        </button>
      {/if}
      {#if activeStatusLabel}
        <button class="pill status-pill" on:click={() => applyStatus(null)}>
          {activeStatusLabel} <X size={11} />
        </button>
      {/if}
    </div>

    <button class="clear-btn" on:click={clearAll} title="Clear all filters">
      <X size={13} />
      {$strings.clear}
    </button>
  {/if}
</div>

<style>
  .filter-bar {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 0.5rem;
    margin-bottom: 1.25rem;
    padding: 0.625rem 0.875rem;
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 12px;
  }

  .filter-icon-wrap {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    color: var(--text-muted);
    flex-shrink: 0;
  }

  .filter-label {
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.06em;
  }

  .dropdown-container {
    min-width: 140px;
    flex-shrink: 0;
  }

  .dropdown-container.status {
    min-width: 150px;
  }

  .active-pills {
    display: flex;
    gap: 0.35rem;
    flex-wrap: wrap;
    align-items: center;
    flex: 1;
  }

  .pill {
    display: flex;
    align-items: center;
    gap: 0.3rem;
    font-size: 0.7rem;
    font-weight: 600;
    padding: 0.2rem 0.55rem;
    border-radius: 99px;
    border: none;
    cursor: pointer;
    transition: all 0.15s;
  }

  .project-pill { background: var(--accent-blue-hover, rgba(59,130,246,0.12)); color: var(--accent-blue, #6366f1); }
  .project-pill:hover { background: var(--accent-blue, rgba(59,130,246,0.22)); color: white; }

  .tag-pill { background: var(--accent-green-hover, rgba(16,185,129,0.12)); color: var(--accent-green, #10b981); }
  .tag-pill:hover { background: var(--accent-green, rgba(16,185,129,0.22)); color: white; }

  .status-pill { background: var(--bg-card); border: 1px solid var(--border); color: var(--text); }
  .status-pill:hover { border-color: var(--accent-blue); }

  .clear-btn {
    display: flex;
    align-items: center;
    gap: 0.3rem;
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--text-muted);
    background: none;
    border: none;
    cursor: pointer;
    padding: 0.25rem 0.5rem;
    border-radius: 6px;
    transition: all 0.15s;
    margin-left: auto;
    font-family: inherit;
  }

  .clear-btn:hover { color: var(--error-red, #ef4444); }
</style>
