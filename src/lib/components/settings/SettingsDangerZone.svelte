<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { ask } from '@tauri-apps/plugin-dialog';
  import { createEventDispatcher } from 'svelte';
  import { Trash2 } from 'lucide-svelte';

  const dispatch = createEventDispatcher<{ message: { text: string; type: 'success' | 'error' } }>();

  let clearBusy = false;
  let isConfirming = false;

  async function clearAllData() {
    if (clearBusy || isConfirming) return;
    isConfirming = true;
    const ok = await ask(
      'Clear all session data? This cannot be undone. Your timer will not be affected.',
      { title: 'Confirm deletion', kind: 'warning' }
    );
    isConfirming = false;
    if (!ok) return;

    clearBusy = true;
    try {
      await invoke('reset_database');
      dispatch('message', { text: 'All session data cleared', type: 'success' });
    } catch (error) {
      dispatch('message', { text: 'Failed to clear data: ' + error, type: 'error' });
    } finally {
      clearBusy = false;
    }
  }
</script>

<div class="settings-card danger-card">
  <div class="card-header">
    <div class="header-icon danger">
      <Trash2 size={20} />
    </div>
    <div class="header-text">
      <h3>Danger Zone</h3>
      <p>Actions that cannot be undone</p>
    </div>
  </div>
  <div class="card-content">
    <p class="danger-text">Wipe all session history. This will not affect your current timer status or settings.</p>
    <button class="premium-btn danger-btn" on:click={clearAllData} disabled={clearBusy || isConfirming}>
      <Trash2 size={18} />
      <span>{clearBusy ? 'Clearing…' : 'Reset Database'}</span>
    </button>
  </div>
</div>

<style>
  .settings-card {
    background: var(--bg-card); border: 1px solid var(--border);
    border-radius: 1.25rem; padding: 1.5rem;
    box-shadow: 0 4px 20px rgba(0,0,0,0.03); transition: box-shadow 0.2s;
  }
  .settings-card:hover { box-shadow: 0 8px 30px rgba(0,0,0,0.05); }
  .danger-card   { border-color: rgba(239,68,68,0.2); }
  .card-header   { display: flex; gap: 1rem; margin-bottom: 2rem; }
  .header-icon {
    width: 42px; height: 42px; border-radius: 12px;
    display: flex; align-items: center; justify-content: center; flex-shrink: 0;
  }
  .header-icon.danger { background: rgba(239,68,68,0.1); color: #ef4444; }
  .header-text h3 { margin: 0; font-size: 1.125rem; font-weight: 700; color: var(--text); }
  .header-text p  { margin: 0.25rem 0 0 0; font-size: 0.8125rem; color: var(--text-muted); }
  .card-content   { display: flex; flex-direction: column; gap: 1rem; }
  .danger-text    { font-size: 0.8125rem; color: var(--text-muted); line-height: 1.5; margin: 0; }
  .premium-btn {
    display: flex; align-items: center; gap: 0.625rem;
    padding: 0.75rem; border-radius: 12px; font-size: 0.875rem; font-weight: 600;
    cursor: pointer; transition: all 0.2s; border: none; font-family: inherit;
  }
  .danger-btn {
    background: rgba(239,68,68,0.1); color: #ef4444;
    border: 1px solid rgba(239,68,68,0.2); margin-top: 0.5rem;
  }
  .danger-btn:hover:not(:disabled) { background: #ef4444; color: white; }
  .premium-btn:disabled { opacity: 0.5; cursor: not-allowed; }
</style>
