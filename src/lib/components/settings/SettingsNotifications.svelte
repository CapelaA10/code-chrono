<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';
  import { strings } from '$lib/i18n/store';
  import { Bell } from 'lucide-svelte';

  let notificationsEnabled = true;
  let notifyOnStart       = true;
  let notifyOnEnd         = true;
  let notifyBreakRecommend = true;

  onMount(async () => {
    try {
      const load = async (key: string) => {
        const v = await invoke<string | null>('get_setting', { key });
        return v !== 'false';
      };
      notificationsEnabled  = await load('notifications_enabled');
      notifyOnStart         = await load('notify_on_timer_start');
      notifyOnEnd           = await load('notify_on_timer_end');
      notifyBreakRecommend  = await load('notify_break_recommend');
    } catch { /* keep defaults */ }
  });

  async function toggle(key: string, value: boolean) {
    await invoke('set_setting', { key, value: String(value) });
  }
</script>

<div class="settings-card">
  <div class="card-header">
    <div class="header-icon notif">
      <Bell size={20} />
    </div>
    <div class="header-text">
      <h3>{$strings.notifications}</h3>
      <p>{$strings.notificationsDesc}</p>
    </div>
  </div>

  <div class="card-content">

    <!-- Master toggle -->
    <div class="setting-row">
      <div class="row-info">
        <span class="row-label">{$strings.enableNotifications}</span>
      </div>
      <button
        class="toggle-switch" class:on={notificationsEnabled}
        on:click={async () => { notificationsEnabled = !notificationsEnabled; await toggle('notifications_enabled', notificationsEnabled); }}
        role="switch" aria-checked={notificationsEnabled}
        aria-label={$strings.enableNotifications}
      ><span class="toggle-knob"></span></button>
    </div>

    <div class="divider"></div>

    <!-- Per-event toggles (greyed out when master is off) -->
    <div class:dimmed={!notificationsEnabled}>

      <div class="setting-row">
        <div class="row-info">
          <span class="row-label">{$strings.notifyOnStart}</span>
        </div>
        <button
          class="toggle-switch" class:on={notifyOnStart}
          disabled={!notificationsEnabled}
          on:click={async () => { notifyOnStart = !notifyOnStart; await toggle('notify_on_timer_start', notifyOnStart); }}
          role="switch" aria-checked={notifyOnStart}
          aria-label={$strings.notifyOnStart}
        ><span class="toggle-knob"></span></button>
      </div>

      <div class="divider"></div>

      <div class="setting-row">
        <div class="row-info">
          <span class="row-label">{$strings.notifyOnEnd}</span>
        </div>
        <button
          class="toggle-switch" class:on={notifyOnEnd}
          disabled={!notificationsEnabled}
          on:click={async () => { notifyOnEnd = !notifyOnEnd; await toggle('notify_on_timer_end', notifyOnEnd); }}
          role="switch" aria-checked={notifyOnEnd}
          aria-label={$strings.notifyOnEnd}
        ><span class="toggle-knob"></span></button>
      </div>

      <div class="divider"></div>

      <div class="setting-row">
        <div class="row-info">
          <span class="row-label">{$strings.notifyBreakRecommend}</span>
          <span class="row-hint">{$strings.notifyBreakRecommendHint}</span>
        </div>
        <button
          class="toggle-switch" class:on={notifyBreakRecommend}
          disabled={!notificationsEnabled}
          on:click={async () => { notifyBreakRecommend = !notifyBreakRecommend; await toggle('notify_break_recommend', notifyBreakRecommend); }}
          role="switch" aria-checked={notifyBreakRecommend}
          aria-label={$strings.notifyBreakRecommend}
        ><span class="toggle-knob"></span></button>
      </div>

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
  .header-icon.notif {
    background: color-mix(in srgb, var(--accent-blue) 12%, transparent);
    color: var(--accent-blue);
  }
  .header-text h3 { margin: 0; font-size: 1.125rem; font-weight: 700; color: var(--text); }
  .header-text p  { margin: 0.25rem 0 0 0; font-size: 0.8125rem; color: var(--text-muted); }
  .card-content   { display: flex; flex-direction: column; gap: 1rem; }
  .setting-row    { display: flex; justify-content: space-between; align-items: center; }
  .row-info       { display: flex; flex-direction: column; gap: 0.2rem; }
  .row-label      { font-size: 0.9375rem; font-weight: 600; color: var(--text); }
  .row-hint       { font-size: 0.75rem; color: var(--text-muted); }
  .divider        { height: 1px; background: var(--border); margin: 0.5rem 0; }
  .dimmed         { opacity: 0.45; pointer-events: none; transition: opacity 0.2s; }

  .toggle-switch {
    width: 40px; height: 22px; border-radius: 99px;
    background: var(--border);
    border: none; cursor: pointer; padding: 2px; position: relative;
    transition: background 0.2s; flex-shrink: 0;
    display: flex; align-items: center;
  }
  .toggle-switch.on { background: var(--accent-blue); }
  .toggle-switch:disabled { cursor: default; }
  .toggle-knob {
    width: 18px; height: 18px; border-radius: 50%;
    background: white;
    box-shadow: 0 1px 3px rgba(0,0,0,0.3);
    transition: transform 0.2s;
    display: block;
  }
  .toggle-switch.on .toggle-knob { transform: translateX(18px); }
</style>
