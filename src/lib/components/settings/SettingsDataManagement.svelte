<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { save, open } from '@tauri-apps/plugin-dialog';
  import { writeTextFile } from '@tauri-apps/plugin-fs';
  import { createEventDispatcher } from 'svelte';
  import { Database, Download, Upload } from 'lucide-svelte';
  import { strings } from '$lib/i18n/store';

  const dispatch = createEventDispatcher<{ message: { text: string; type: 'success' | 'error' } }>();

  let exportBusy = false;
  let importBusy = false;

  async function exportCSV() {
    exportBusy = true;
    try {
      const csv = await invoke<string>('export_csv');
      const filePath = await save({ filters: [{ name: 'CSV', extensions: ['csv'] }] });
      if (filePath) {
        await writeTextFile(filePath, csv);
        dispatch('message', { text: 'Export complete', type: 'success' });
      }
    } catch (error) {
      dispatch('message', { text: 'Export failed: ' + error, type: 'error' });
    } finally {
      exportBusy = false;
    }
  }

  async function importCSV() {
    importBusy = true;
    try {
      const filePath = await open({ multiple: false, filters: [{ name: 'CSV', extensions: ['csv'] }] });
      if (filePath && typeof filePath === 'string') {
        const count = await invoke<number>('import_csv', { path: filePath });
        dispatch('message', { text: `Imported ${count} session(s)`, type: 'success' });
      }
    } catch (error) {
      dispatch('message', { text: 'Import failed: ' + error, type: 'error' });
    } finally {
      importBusy = false;
    }
  }
</script>

<div class="settings-card">
  <div class="card-header">
    <div class="header-icon management">
      <Database size={20} />
    </div>
    <div class="header-text">
      <h3>{$strings.dataManagement}</h3>
      <p>Import or export your time records</p>
    </div>
  </div>
  <div class="card-content">
    <div class="action-grid">
      <button class="premium-btn primary" on:click={exportCSV} disabled={exportBusy}>
        <Download size={18} />
        <span>{exportBusy ? 'Exporting…' : $strings.exportData}</span>
      </button>
      <button class="premium-btn secondary" on:click={importCSV} disabled={importBusy}>
        <Upload size={18} />
        <span>{importBusy ? 'Importing…' : $strings.importCsv}</span>
      </button>
    </div>
  </div>
</div>

<style>
  .settings-card {
    background: var(--bg-card); border: 1px solid var(--border);
    border-radius: 1.25rem; padding: 1.5rem;
    box-shadow: var(--shadow-lg); transition: box-shadow 0.2s;
  }
  .settings-card:hover { box-shadow: var(--shadow); }
  .card-header { display: flex; gap: 1rem; margin-bottom: 2rem; }
  .header-icon {
    width: 42px; height: 42px; border-radius: 12px;
    display: flex; align-items: center; justify-content: center; flex-shrink: 0;
  }
  .header-icon.management { background: rgba(245,158,11,0.1); color: #f59e0b; }
  .header-text h3 { margin: 0; font-size: 1.125rem; font-weight: 700; color: var(--text); }
  .header-text p  { margin: 0.25rem 0 0 0; font-size: 0.8125rem; color: var(--text-muted); }
  .card-content   { display: flex; flex-direction: column; gap: 1rem; }
  .action-grid    { display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; }
  .premium-btn {
    display: flex; align-items: center; justify-content: center; gap: 0.625rem;
    padding: 0.75rem; border-radius: 12px; font-size: 0.875rem; font-weight: 600;
    cursor: pointer; transition: all 0.2s; border: none; font-family: inherit;
  }
  .premium-btn.primary  { background: var(--accent-blue); color: white; }
  .premium-btn.primary:hover:not(:disabled) { filter: brightness(0.9); transform: translateY(-1px); }
  .premium-btn.secondary {
    background: var(--bg-page); border: 1px solid var(--border); color: var(--text);
  }
  .premium-btn.secondary:hover:not(:disabled) { background: var(--btn-secondary-hover-bg); }
  .premium-btn:disabled { opacity: 0.5; cursor: not-allowed; }
</style>
