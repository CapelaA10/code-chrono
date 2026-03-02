<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { open } from '@tauri-apps/plugin-dialog';
  import { onMount } from 'svelte';
  import { strings } from '$lib/i18n/store';
  import { Cpu, Plus, Trash2, Search } from 'lucide-svelte';

  interface TrackedProgram {
    id: number | null;
    name: string;
    executable: string;
    enabled: boolean;
    is_custom: boolean;
  }

  let programs: TrackedProgram[] = [];
  let notifyOnOpen = true;
  let scanning = false;

  onMount(async () => {
    await loadPrograms();
    const v = await invoke<string | null>('get_setting', { key: 'notify_on_program_open' });
    notifyOnOpen = v !== 'false';
  });

  async function loadPrograms() {
    programs = await invoke<TrackedProgram[]>('get_tracked_programs');
  }

  async function scan() {
    scanning = true;
    try {
      const detected = await invoke<{ name: string; executable: string }[]>('scan_installed_programs');
      for (const d of detected) {
        const exists = programs.some(p => p.executable === d.executable);
        if (!exists) {
          await invoke('save_tracked_program', {
            program: { id: null, name: d.name, executable: d.executable, enabled: true, is_custom: false }
          });
        }
      }
      await loadPrograms();
    } finally {
      scanning = false;
    }
  }

  async function toggleProgram(p: TrackedProgram) {
    p.enabled = !p.enabled;
    programs = [...programs];
    await invoke('toggle_tracked_program', { id: p.id, enabled: p.enabled });
  }

  async function removeProgram(p: TrackedProgram) {
    await invoke('remove_tracked_program', { id: p.id });
    await loadPrograms();
  }

  async function addCustom() {
    const selected = await open({ multiple: false, title: $strings.addCustomProgram });
    if (!selected || typeof selected !== 'string') return;
    const parts = selected.split('/');
    const fileName = parts[parts.length - 1];
    const name = fileName.replace(/\.(app|exe)$/i, '');
    await invoke('save_tracked_program', {
      program: { id: null, name, executable: fileName.toLowerCase(), is_custom: true, enabled: true }
    });
    await loadPrograms();
  }

  async function toggleNotifyOnOpen() {
    notifyOnOpen = !notifyOnOpen;
    await invoke('set_setting', { key: 'notify_on_program_open', value: String(notifyOnOpen) });
  }
</script>

<div class="settings-card">
  <div class="card-header">
    <div class="header-icon programs">
      <Cpu size={20} />
    </div>
    <div class="header-text">
      <h3>{$strings.programs}</h3>
      <p>{$strings.programsDesc}</p>
    </div>
  </div>

  <div class="card-content">

    <!-- Notify toggle -->
    <div class="setting-row">
      <div class="row-info">
        <span class="row-label">{$strings.notifyOnProgramOpen}</span>
        <span class="row-hint">{$strings.notifyOnProgramOpenHint}</span>
      </div>
      <button
        class="toggle-switch" class:on={notifyOnOpen}
        on:click={toggleNotifyOnOpen}
        role="switch" aria-checked={notifyOnOpen}
        aria-label={$strings.notifyOnProgramOpen}
      ><span class="toggle-knob"></span></button>
    </div>

    <div class="divider"></div>

    <!-- Action buttons -->
    <div class="action-row">
      <button class="btn-secondary scan-btn" on:click={scan} disabled={scanning}>
        <Search size={14} />
        {scanning ? '…' : $strings.scanPrograms}
      </button>
      <button class="btn-secondary" on:click={addCustom}>
        <Plus size={14} />
        {$strings.addCustomProgram}
      </button>
    </div>

    <!-- Program list -->
    {#if programs.length === 0}
      <p class="empty-hint">{$strings.noTrackedPrograms}</p>
    {:else}
      <div class="program-list">
        {#each programs as p (p.id)}
          <div class="program-row">
            <div class="program-info">
              <span class="program-name">{p.name}</span>
              <span class="program-exe">{p.executable}</span>
            </div>
            <div class="program-controls">
              <button
                class="toggle-switch sm" class:on={p.enabled}
                on:click={() => toggleProgram(p)}
                role="switch" aria-checked={p.enabled}
                aria-label="{$strings.enableFor} {p.name}"
              ><span class="toggle-knob"></span></button>
              <button class="icon-btn danger" on:click={() => removeProgram(p)} title="Remove">
                <Trash2 size={14} />
              </button>
            </div>
          </div>
        {/each}
      </div>
    {/if}

  </div>
</div>

<style>
  .settings-card {
    background: var(--bg-card); border: 1px solid var(--border);
    border-radius: 1.25rem; padding: 1.5rem;
    box-shadow: 0 4px 20px rgba(0,0,0,0.03); transition: box-shadow 0.2s;
  }
  .settings-card:hover { box-shadow: 0 8px 30px rgba(0,0,0,0.05); }
  .card-header { display: flex; gap: 1rem; margin-bottom: 2rem; }
  .header-icon {
    width: 42px; height: 42px; border-radius: 12px;
    display: flex; align-items: center; justify-content: center; flex-shrink: 0;
  }
  .header-icon.programs {
    background: color-mix(in srgb, var(--accent-green, #10b981) 12%, transparent);
    color: var(--accent-green, #10b981);
  }
  .header-text h3 { margin: 0; font-size: 1.125rem; font-weight: 700; color: var(--text); }
  .header-text p  { margin: 0.25rem 0 0 0; font-size: 0.8125rem; color: var(--text-muted); }
  .card-content { display: flex; flex-direction: column; gap: 1rem; }
  .setting-row { display: flex; justify-content: space-between; align-items: center; }
  .row-info    { display: flex; flex-direction: column; gap: 0.2rem; }
  .row-label   { font-size: 0.9375rem; font-weight: 600; color: var(--text); }
  .row-hint    { font-size: 0.75rem; color: var(--text-muted); }
  .divider     { height: 1px; background: var(--border); margin: 0.25rem 0; }
  .empty-hint  { font-size: 0.8125rem; color: var(--text-muted); margin: 0; }

  .action-row {
    display: flex; gap: 0.75rem; flex-wrap: wrap;
  }
  .btn-secondary {
    display: inline-flex; align-items: center; gap: 0.4rem;
    background: var(--bg-page); border: 1px solid var(--border);
    border-radius: 8px; padding: 0.5rem 0.875rem;
    font-size: 0.8125rem; font-weight: 600; color: var(--text);
    cursor: pointer; transition: var(--transition);
    font-family: inherit;
  }
  .btn-secondary:hover:not(:disabled) { filter: brightness(0.95); }
  .btn-secondary:disabled { opacity: 0.5; cursor: default; }

  .program-list { display: flex; flex-direction: column; gap: 0.5rem; }
  .program-row {
    display: flex; justify-content: space-between; align-items: center;
    padding: 0.625rem 0.75rem; border-radius: 10px;
    background: var(--bg-page); border: 1px solid var(--border);
    gap: 1rem;
  }
  .program-info { display: flex; flex-direction: column; gap: 0.15rem; min-width: 0; }
  .program-name { font-size: 0.875rem; font-weight: 600; color: var(--text); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .program-exe  { font-size: 0.7rem; color: var(--text-muted); font-family: monospace; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .program-controls { display: flex; align-items: center; gap: 0.5rem; flex-shrink: 0; }

  .icon-btn {
    background: none; border: none; cursor: pointer; padding: 0.25rem;
    border-radius: 6px; display: flex; align-items: center; transition: var(--transition);
    color: var(--text-muted);
  }
  .icon-btn:hover { background: var(--btn-secondary-hover-bg); }
  .icon-btn.danger:hover { color: var(--accent-red, #ef4444); }

  /* Toggle switch */
  .toggle-switch {
    width: 40px; height: 22px; border-radius: 99px;
    background: var(--border);
    border: none; cursor: pointer; padding: 2px;
    transition: background 0.2s; flex-shrink: 0;
    display: flex; align-items: center;
  }
  .toggle-switch.sm { width: 34px; height: 18px; }
  .toggle-switch.on { background: var(--accent-blue); }
  .toggle-knob {
    width: 18px; height: 18px; border-radius: 50%;
    background: white; box-shadow: 0 1px 3px rgba(0,0,0,0.3);
    transition: transform 0.2s; display: block;
  }
  .toggle-switch.sm .toggle-knob { width: 14px; height: 14px; }
  .toggle-switch.on .toggle-knob { transform: translateX(18px); }
  .toggle-switch.sm.on .toggle-knob { transform: translateX(16px); }
</style>
