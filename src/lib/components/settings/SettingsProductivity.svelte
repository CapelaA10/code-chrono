<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import { idleMinutes } from '$lib/stores/idle';
  import { timerDuration } from '$lib/stores/timerSettings';
  import { strings } from '$lib/i18n/store';
  import { Keyboard } from 'lucide-svelte';
  import Dropdown from '$lib/components/Dropdown.svelte';

  const DURATIONS = [5, 10, 15, 20, 25, 30, 45, 60, 90, 120];

  $: isMac = typeof navigator !== 'undefined' && /Mac|iPod|iPhone|iPad/.test(navigator.platform);
  $: hotkeyLabel = isMac ? '⌘⇧P' : 'Ctrl+Shift+P';

  let autoImportProjects = true;

  onMount(async () => {
    try {
      const v = await invoke<string | null>('get_setting', { key: 'auto_import_projects' });
      autoImportProjects = v !== 'false';
    } catch { /* keep default */ }
  });

  async function toggleAutoImportProjects() {
    autoImportProjects = !autoImportProjects;
    await invoke('set_setting', { key: 'auto_import_projects', value: String(autoImportProjects) });
  }

  function setIdleMinutes(v: number) {
    const n = Math.max(0, Math.min(60, Math.round(v)));
    idleMinutes.set(n);
  }
</script>

<div class="settings-card">
  <div class="card-header">
    <div class="header-icon productivity">
      <Keyboard size={20} />
    </div>
    <div class="header-text">
      <h3>{$strings.productivity}</h3>
      <p>{$strings.productivityDesc}</p>
    </div>
  </div>
  <div class="card-content">

    <div class="setting-row">
      <div class="row-info">
        <span class="row-label">{$strings.globalToggle}</span>
        <span class="row-hint">{$strings.pauseResumeHint}</span>
      </div>
      <kbd class="shortcut-tag">{hotkeyLabel}</kbd>
    </div>

    <div class="divider"></div>

    <div class="setting-row">
      <div class="row-info">
        <span class="row-label">{$strings.autoPause}</span>
        <span class="row-hint">{$strings.autoPauseHint}</span>
      </div>
      <div class="number-input-group">
        <input
          type="number" min="0" max="60"
          value={$idleMinutes}
          on:input={(e) => setIdleMinutes(Number((e.currentTarget as HTMLInputElement).value))}
          class="premium-input number"
        />
        <span class="unit-label">{$strings.mins}</span>
      </div>
    </div>

    <div class="divider"></div>

    <div class="setting-row">
      <div class="row-info">
        <span class="row-label">{$strings.defaultDuration}</span>
        <span class="row-hint">{$strings.defaultDurationHint}</span>
      </div>
      <div style="min-width: 140px;">
        <Dropdown
          value={$timerDuration}
          options={DURATIONS.map(d => ({ value: d, label: d + ' ' + $strings.mins }))}
          on:change={(e) => $timerDuration = e.detail}
        />
      </div>
    </div>
    <div class="divider"></div>

    <div class="setting-row">
      <div class="row-info">
        <span class="row-label">{$strings.autoImportProjects}</span>
        <span class="row-hint">{$strings.autoImportHint}</span>
      </div>
      <button
        class="toggle-switch"
        class:on={autoImportProjects}
        on:click={toggleAutoImportProjects}
        role="switch"
        aria-checked={autoImportProjects}
        title={autoImportProjects ? 'Enabled — click to disable' : 'Disabled — click to enable'}
      >
        <span class="toggle-knob"></span>
      </button>
    </div>

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
  .header-icon.productivity { background: var(--accent-green-hover, rgba(16,185,129,0.1)); color: var(--accent-green, #10b981); }
  .header-text h3 { margin: 0; font-size: 1.125rem; font-weight: 700; color: var(--text); }
  .header-text p  { margin: 0.25rem 0 0 0; font-size: 0.8125rem; color: var(--text-muted); }
  .card-content   { display: flex; flex-direction: column; gap: 1rem; }
  .setting-row    { display: flex; justify-content: space-between; align-items: center; }
  .row-info       { display: flex; flex-direction: column; gap: 0.2rem; }
  .row-label      { font-size: 0.9375rem; font-weight: 600; color: var(--text); }
  .row-hint       { font-size: 0.75rem; color: var(--text-muted); }
  .shortcut-tag {
    background: var(--bg-page); border: 1px solid var(--border);
    padding: 0.4rem 0.75rem; border-radius: 8px; font-family: inherit;
    font-size: 0.8125rem; font-weight: 600; color: var(--text);
    box-shadow: inset 0 1px 0 rgba(255,255,255,0.1);
  }
  .divider { height: 1px; background: var(--border); margin: 0.5rem 0; }
  .number-input-group { display: flex; align-items: center; gap: 0.75rem; }
  .premium-input {
    background: var(--bg-page); border: 1px solid var(--border); border-radius: 10px;
    padding: 0.625rem; color: var(--text); font-size: 0.9375rem;
    outline: none; transition: border-color 0.2s, box-shadow 0.2s; font-family: inherit;
  }
  .premium-input:focus { border-color: var(--accent-blue); box-shadow: 0 0 0 3px var(--focus-ring); }
  .premium-input.number { width: 4rem; text-align: center; }
  .unit-label { font-size: 0.875rem; color: var(--text-muted); }

  /* ── Toggle switch ────────────────────────────────────────────────────── */
  .toggle-switch {
    width: 40px; height: 22px; border-radius: 99px;
    background: var(--border);
    border: none; cursor: pointer; padding: 2px; position: relative;
    transition: background 0.2s; flex-shrink: 0;
    display: flex; align-items: center;
  }
  .toggle-switch.on { background: var(--accent-blue); }
  .toggle-knob {
    width: 18px; height: 18px; border-radius: 50%;
    background: white;
    box-shadow: 0 1px 3px rgba(0,0,0,0.3);
    transition: transform 0.2s;
    display: block;
  }
  .toggle-switch.on .toggle-knob { transform: translateX(18px); }
</style>
