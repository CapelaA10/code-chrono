<!-- SyncDrawerFooter.svelte
     Import-options checkboxes (projects / labels) + error message + Cancel/Import buttons.
     Emits: cancel, import -->
<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { Download, RefreshCw } from 'lucide-svelte';

  export let selectedCount: number = 0;
  export let importBusy: boolean = false;
  export let error: string = '';
  export let importProjects: boolean = true;
  export let importLabels: boolean = true;
  export let projectCount: number = 0;
  export let labelCount: number = 0;

  const dispatch = createEventDispatcher<{ cancel: void; import: void }>();
</script>

<div class="drawer-footer">
  {#if projectCount > 0}
    <label class="mini-toggle footer-toggle">
      <input type="checkbox" bind:checked={importProjects} />
      Import projects ({projectCount})
    </label>
  {/if}

  {#if labelCount > 0}
    <label class="mini-toggle footer-toggle">
      <input type="checkbox" bind:checked={importLabels} />
      Import labels as tags ({labelCount})
    </label>
  {/if}

  {#if error}
    <p class="footer-err">{error}</p>
  {/if}

  <div class="footer-actions">
    <button class="btn secondary" on:click={() => dispatch('cancel')}>Cancel</button>
    <button
      class="btn primary"
      on:click={() => dispatch('import')}
      disabled={selectedCount === 0 || importBusy}
    >
      {#if importBusy}
        <RefreshCw size={13} class="spin" />
      {:else}
        <Download size={13} />
      {/if}
      {importBusy ? 'Importing…' : `Import ${selectedCount}`}
    </button>
  </div>
</div>

<style>
  .drawer-footer {
    padding: 0.75rem 0.875rem;
    border-top: 1px solid var(--border);
    flex-shrink: 0; background: var(--bg-page);
    display: flex; flex-direction: column; gap: 0.5rem;
  }

  .mini-toggle {
    display: flex; align-items: center; gap: 0.3rem;
    font-size: 0.75rem; color: var(--text-muted); cursor: pointer;
    white-space: nowrap;
  }
  .footer-toggle { font-size: 0.8rem; }
  .footer-err    { font-size: 0.75rem; color: var(--error-red, #ef4444); margin: 0; }
  .footer-actions { display: flex; gap: 0.5rem; justify-content: flex-end; }

  .btn {
    display: flex; align-items: center; gap: 0.4rem;
    padding: 0.5rem 1rem; border-radius: 8px;
    font-size: 0.8125rem; font-weight: 600; cursor: pointer;
    transition: all 0.15s; border: none; font-family: inherit;
  }
  .btn.primary { background: var(--accent-blue); color: white; }
  .btn.primary:hover:not(:disabled) { background: #2563eb; }
  .btn.primary:disabled { opacity: 0.5; cursor: not-allowed; }
  .btn.secondary { background: transparent; border: 1px solid var(--border); color: var(--text); }
  .btn.secondary:hover { background: var(--btn-secondary-hover-bg); }

  :global(.spin) { animation: spin 0.8s linear infinite; }
  @keyframes spin { to { transform: rotate(360deg); } }
</style>
