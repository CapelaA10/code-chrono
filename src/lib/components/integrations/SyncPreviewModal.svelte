<!-- SyncPreviewModal.svelte
     Orchestrator for the right-side import drawer.
     Owns all state and async logic; renders sub-components for each visual zone.

     Sub-components:
       ├── SyncDrawerHeader  — source badge, title, count badges, close
       ├── SyncFilterBar     — search, project/label selects, hide-imported toggle
       ├── SyncIssueList     — select-all bar + scrollable issue rows
       └── SyncDrawerFooter  — import-option checkboxes + action buttons

     Emits: close
-->
<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { createEventDispatcher, onMount } from 'svelte';
  import { refreshTasks, refreshTags, refreshProjects } from '$lib/stores/tasks';
  import { CheckCircle2, RefreshCw, AlertCircle } from 'lucide-svelte';
  import { strings } from '$lib/i18n/store';

  import SyncDrawerHeader  from './SyncDrawerHeader.svelte';
  import SyncFilterBar     from './SyncFilterBar.svelte';
  import SyncIssueList     from './SyncIssueList.svelte';
  import SyncDrawerFooter  from './SyncDrawerFooter.svelte';
  import type { ExternalTask } from './syncTypes';

  /** "GitHub" | "GitLab" | "Jira" */
  export let source: string;

  const dispatch = createEventDispatcher<{ close: void }>();

  // ── Data ──────────────────────────────────────────────────────────────────
  let issues: ExternalTask[] = [];

  // ── UI state ──────────────────────────────────────────────────────────────
  let loading      = true;
  let importBusy   = false;
  let error        = '';
  let done         = false;
  let importedCount = 0;

  // ── Filter state ──────────────────────────────────────────────────────────
  let search        = '';
  let filterProject = '';
  let filterLabel   = '';
  let hideImported  = true;

  // ── Import options ────────────────────────────────────────────────────────
  let importLabels   = true;
  let importProjects = true;   // default; overridden from DB setting on mount

  // ── Selection ─────────────────────────────────────────────────────────────
  let selected = new Set<string>();

  // ── Derived ───────────────────────────────────────────────────────────────
  $: allProjects = [...new Set(issues.map(i => i.project).filter(Boolean))] as string[];
  $: allLabels   = [...new Set(issues.flatMap(i => i.labels))];
  $: newCount    = issues.filter(i => !i.already_imported).length;
  $: importedTotal = issues.filter(i => i.already_imported).length;

  $: filtered = issues.filter(i => {
    if (hideImported && i.already_imported) return false;
    if (filterProject && i.project !== filterProject) return false;
    if (filterLabel   && !i.labels.includes(filterLabel)) return false;
    if (search        && !i.title.toLowerCase().includes(search.toLowerCase())) return false;
    return true;
  });

  // ── Lifecycle ─────────────────────────────────────────────────────────────
  onMount(async () => {
    // Read persisted default for auto-project import from settings
    try {
      const v = await invoke<string | null>('get_setting', { key: 'auto_import_projects' });
      importProjects = v !== 'false';
    } catch { /* keep default true */ }
    load();
  });

  // ── Async actions ─────────────────────────────────────────────────────────

  async function load() {
    loading = true; error = ''; selected = new Set();
    try {
      const cmd = source === 'GitHub' ? 'preview_sync_github'
                : source === 'GitLab' ? 'preview_sync_gitlab'
                : 'preview_sync_jira';
      issues = await invoke<ExternalTask[]>(cmd);
      // Auto-select all not-yet-imported issues
      issues.filter(i => !i.already_imported).forEach(i => selected.add(i.id));
      selected = new Set(selected);
    } catch (e: any) {
      error = cleanError(String(e));
    } finally {
      loading = false;
    }
  }

  async function doImport() {
    if (importBusy || selected.size === 0) return;
    importBusy = true;
    try {
      importedCount = await invoke<number>('import_selected', {
        source,
        selectedIds:  [...selected],
        importLabels,
        importProjects,
      });
      await refreshTasks();
      if (importLabels)   await refreshTags();
      if (importProjects) await refreshProjects();
      done = true;
    } catch (e: any) {
      error = cleanError(String(e));
    } finally {
      importBusy = false;
    }
  }

  // ── Helpers ───────────────────────────────────────────────────────────────

  function cleanError(raw: string): string {
    if (raw.includes('not configured')) return `${source} not configured — check Settings.`;
    if (raw.includes('401') || raw.includes('Unauthorized')) return 'Invalid token.';
    if (raw.includes('404')) return 'Not found — check your repo / domain.';
    return raw.replace('Error: ', '');
  }

  function toggle(id: string) {
    const issue = issues.find(i => i.id === id);
    if (!issue || issue.already_imported) return;
    selected.has(id) ? selected.delete(id) : selected.add(id);
    selected = new Set(selected);
  }

  function toggleAll() {
    const selectable = filtered.filter(i => !i.already_imported);
    const allSel     = selectable.every(i => selected.has(i.id));
    if (allSel) selectable.forEach(i => selected.delete(i.id));
    else        selectable.forEach(i => selected.add(i.id));
    selected = new Set(selected);
  }

  function close() { dispatch('close'); }
</script>

<!-- ── Backdrop ─────────────────────────────────────────────────────────── -->
<button class="backdrop" on:click={close} aria-label="Close"></button>

<!-- ── Drawer shell ─────────────────────────────────────────────────────── -->
<div class="drawer" role="dialog" aria-modal="true" aria-label="Import issues from {source}">

  <SyncDrawerHeader
    {source}
    {newCount}
    {importedTotal}
    showCounts={!loading && !error && !done}
    on:close={close}
  />

  <!-- ── States: done / loading / error / ready ──────────────────────── -->
  {#if done}
    <div class="center-state">
      <CheckCircle2 size={36} class="state-ok" />
      <p class="state-title">{importedCount} issue{importedCount !== 1 ? 's' : ''} imported</p>
      <button class="btn primary small" on:click={close}>{$strings.done}</button>
    </div>

  {:else if loading}
    <div class="center-state">
      <RefreshCw size={20} class="spin" />
      <p class="state-hint">{$strings.loadingIssues}</p>
    </div>

  {:else if error && issues.length === 0}
    <div class="center-state">
      <AlertCircle size={24} class="state-err" />
      <p class="state-hint">{error}</p>
      <button class="btn secondary small" on:click={load}>{$strings.retry}</button>
    </div>

  {:else}
    <SyncFilterBar
      bind:search
      bind:filterProject
      bind:filterLabel
      bind:hideImported
      {allProjects}
      {allLabels}
      on:refresh={load}
    />

    <SyncIssueList
      issues={filtered}
      {selected}
      on:toggle={e => toggle(e.detail)}
      on:toggleAll={toggleAll}
    />

    <SyncDrawerFooter
      selectedCount={selected.size}
      {importBusy}
      {error}
      bind:importProjects
      bind:importLabels
      projectCount={allProjects.length}
      labelCount={allLabels.length}
      on:cancel={close}
      on:import={doImport}
    />
  {/if}
</div>

<style>
  /* ── Backdrop ───────────────────────────────────────────────────────────── */
  .backdrop {
    position: fixed; inset: 0;
    background: rgba(0,0,0,0.35);
    backdrop-filter: blur(2px);
    z-index: 200; border: none; cursor: default;
  }

  /* ── Drawer shell ───────────────────────────────────────────────────────── */
  .drawer {
    position: fixed; top: 0; right: 0; bottom: 0;
    width: 360px; z-index: 201;
    background: var(--bg-card);
    border-left: 1px solid var(--border);
    display: flex; flex-direction: column;
    box-shadow: -8px 0 32px rgba(0,0,0,0.15);
    animation: slideIn 0.22s cubic-bezier(0.16,1,0.3,1);
    overflow: hidden;
  }

  @keyframes slideIn {
    from { transform: translateX(100%); opacity: 0; }
    to   { transform: translateX(0);    opacity: 1; }
  }

  /* ── Center states (loading / error / done) ─────────────────────────────── */
  .center-state {
    flex: 1; display: flex; flex-direction: column;
    align-items: center; justify-content: center;
    gap: 0.75rem; padding: 2rem; color: var(--text-muted); text-align: center;
  }

  .state-title { font-size: 1rem; font-weight: 700; color: var(--text); margin: 0; }
  .state-hint  { font-size: 0.8125rem; margin: 0; }

  :global(.state-ok)  { color: var(--accent-green, #10b981); }
  :global(.state-err) { color: var(--error-red, #ef4444); }

  /* ── Buttons (used inside center-states only) ───────────────────────────── */
  .btn {
    display: flex; align-items: center; gap: 0.4rem;
    padding: 0.5rem 1rem; border-radius: 8px;
    font-size: 0.8125rem; font-weight: 600; cursor: pointer;
    transition: all 0.15s; border: none; font-family: inherit;
  }
  .btn.small { padding: 0.4rem 0.875rem; }
  .btn.primary   { background: var(--accent-blue); color: white; }
  .btn.primary:hover { filter: brightness(0.9); }
  .btn.secondary { background: transparent; border: 1px solid var(--border); color: var(--text); }
  .btn.secondary:hover { background: var(--btn-secondary-hover-bg); }

  :global(.spin) { animation: spin 0.8s linear infinite; }
  @keyframes spin { to { transform: rotate(360deg); } }
</style>
