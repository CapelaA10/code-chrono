<!--
  SettingsAppearance.svelte
  ─────────────────────────
  Compact card for theme (light/dark) and language selection.
-->
<script lang="ts">
  import { theme } from '$lib/stores/theme';
  import { locale, LOCALE_OPTIONS } from '$lib/i18n/store';
  import { Sun, Moon } from 'lucide-svelte';
  import { strings } from '$lib/i18n/store';
  import Dropdown from '$lib/components/Dropdown.svelte';

  function setTheme(t: 'light' | 'dark') { theme.set(t); }
</script>

<div class="settings-card">
  <div class="card-header">
    <div>
      <h3>{$strings.appearance}</h3>
      <p>Personalize how the application looks</p>
    </div>
    <!-- Compact inline theme pill -->
    <div class="theme-pill">
      <button
        class="theme-btn"
        class:active={$theme === 'light'}
        on:click={() => setTheme('light')}
        title="Light mode"
      >
        <Sun size={14} /><span>{$strings.light}</span>
      </button>
      <button
        class="theme-btn"
        class:active={$theme === 'dark'}
        on:click={() => setTheme('dark')}
        title="Dark mode"
      >
        <Moon size={14} /><span>{$strings.dark}</span>
      </button>
    </div>
  </div>

  <div class="divider"></div>

  <!-- Language selector -->
  <div class="lang-row">
    <div class="row-info">
      <span class="row-label">{$strings.language}</span>
      <span class="row-hint">Interface display language</span>
    </div>
    <div style="min-width: 140px;">
      <Dropdown
        value={$locale}
        options={LOCALE_OPTIONS}
        on:change={(e) => $locale = e.detail}
      />
    </div>
  </div>
</div>

<style>
  .settings-card {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 1.25rem;
    padding: 1.25rem 1.5rem;
    box-shadow: 0 4px 20px rgba(0,0,0,0.03);
    transition: box-shadow 0.2s;
  }
  .settings-card:hover { box-shadow: 0 8px 30px rgba(0,0,0,0.05); }

  .card-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
  }

  .card-header h3 { margin: 0; font-size: 1rem; font-weight: 700; color: var(--text); }
  .card-header p  { margin: 0.2rem 0 0; font-size: 0.75rem; color: var(--text-muted); }

  .divider { height: 1px; background: var(--border); margin: 1rem 0; }

  .lang-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .row-info { display: flex; flex-direction: column; gap: 0.2rem; }
  .row-label { font-size: 0.9375rem; font-weight: 600; color: var(--text); }
  .row-hint  { font-size: 0.75rem; color: var(--text-muted); }

  /* Compact pill toggle */
  .theme-pill {
    display: flex;
    padding: 3px;
    background: var(--bg-page);
    border: 1px solid var(--border);
    border-radius: 99px;
    gap: 2px;
    flex-shrink: 0;
  }

  .theme-btn {
    display: flex; align-items: center; gap: 0.3rem;
    padding: 0.3rem 0.7rem;
    border: none; background: transparent;
    color: var(--text-muted);
    font-size: 0.75rem; font-weight: 600;
    border-radius: 99px;
    cursor: pointer;
    transition: all 0.18s;
    font-family: inherit;
  }

  .theme-btn.active {
    background: var(--bg-card);
    color: var(--text);
    box-shadow: 0 1px 6px rgba(0,0,0,0.1);
  }

  .theme-btn:not(.active):hover { color: var(--text); }
</style>
