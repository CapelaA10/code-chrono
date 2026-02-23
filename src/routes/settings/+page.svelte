<script lang="ts">
  import { ArrowLeft, Check, Info } from 'lucide-svelte';
  import SettingsAppearance    from '$lib/components/settings/SettingsAppearance.svelte';
  import SettingsProductivity  from '$lib/components/settings/SettingsProductivity.svelte';
  import SettingsIntegrations  from '$lib/components/settings/SettingsIntegrations.svelte';
  import SettingsDataManagement from '$lib/components/settings/SettingsDataManagement.svelte';
  import SettingsDangerZone    from '$lib/components/settings/SettingsDangerZone.svelte';
  import { strings } from '$lib/i18n/store';

  // ── Toast state ──────────────────────────────────────────────────────────
  let message = '';
  let messageType: 'info' | 'success' | 'error' = 'info';
  let toastTimer: ReturnType<typeof setTimeout> | null = null;

  function showMessage(text: string, type: 'info' | 'success' | 'error' = 'success') {
    message = text;
    messageType = type;
    if (toastTimer) clearTimeout(toastTimer);
    toastTimer = setTimeout(() => { message = ''; }, 3000);
  }

  function handleMessage(e: CustomEvent<{ text: string; type: 'success' | 'error' }>) {
    showMessage(e.detail.text, e.detail.type);
  }
</script>

<main class="settings-container">
  <div class="settings-wrapper">
    <div class="settings-header">
      <a href="/" class="back-link">
        <ArrowLeft size={20} />
        <span>{$strings.dashboard}</span>
      </a>
      <h1 class="settings-title">{$strings.preferences}</h1>
      <p class="settings-subtitle">{$strings.preferencesDesc}</p>
    </div>

    <div class="settings-grid">
      <SettingsAppearance />
      <SettingsProductivity />
      <SettingsIntegrations on:message={handleMessage} />
      <SettingsDataManagement on:message={handleMessage} />
      <SettingsDangerZone on:message={handleMessage} />
    </div>
  </div>
</main>

{#if message}
  <button class="toast {messageType}" on:click={() => message = ''} aria-label="Dismiss">
    <div class="toast-content">
      {#if messageType === 'success'}
        <Check size={18} />
      {:else}
        <Info size={18} />
      {/if}
      <span>{message}</span>
    </div>
  </button>
{/if}

<style>
  :global(body) {
    background: var(--bg-page);
    color: var(--text);
    font-family: 'Inter', system-ui, -apple-system, sans-serif;
    margin: 0; padding: 0;
  }

  .settings-container {
    flex: 1; overflow-y: auto; width: 100%;
    padding: 3rem 2rem; box-sizing: border-box; height: 100vh;
  }

  .settings-container::-webkit-scrollbar { width: 8px; }
  .settings-container::-webkit-scrollbar-track { background: transparent; }
  .settings-container::-webkit-scrollbar-thumb { background: var(--border); border-radius: 4px; }
  .settings-container::-webkit-scrollbar-thumb:hover { background: var(--text-muted); }

  .settings-wrapper { max-width: 900px; margin: 0 auto; }

  .settings-header { margin-bottom: 3rem; }

  .back-link {
    display: flex; align-items: center; gap: 0.5rem;
    color: var(--text-muted); text-decoration: none;
    font-size: 0.875rem; font-weight: 500; margin-bottom: 1.5rem; transition: color 0.2s;
  }
  .back-link:hover { color: var(--accent-blue); }

  .settings-title {
    font-size: 2.5rem; font-weight: 800; margin: 0;
    letter-spacing: -0.02em; color: var(--text);
  }

  .settings-subtitle {
    color: var(--text-muted); font-size: 1.125rem; margin: 0.5rem 0 0 0;
  }

  .settings-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 1.5rem;
  }

  /* SettingsIntegrations uses full-width = grid-column: span 2 internally */

  @media (max-width: 800px) {
    .settings-grid { grid-template-columns: 1fr; }
  }

  /* ── Toast ────────────────────────────────────────────────────────────── */
  .toast {
    position: fixed; bottom: 2rem; left: 50%; transform: translateX(-50%);
    background: var(--btn-primary-bg); color: var(--btn-primary-text); padding: 0.75rem 1.5rem; border-radius: 99px;
    border: none; box-shadow: var(--shadow-lg); z-index: 1000;
    cursor: pointer; animation: slideUp 0.4s cubic-bezier(0.16, 1, 0.3, 1);
  }
  .toast.success { background: var(--accent-green); }
  .toast.error   { background: var(--error-red); }
  .toast-content { display: flex; align-items: center; gap: 0.75rem; font-size: 0.875rem; font-weight: 600; }

  @keyframes slideUp {
    from { transform: translate(-50%, 2rem); opacity: 0; }
    to   { transform: translate(-50%, 0);    opacity: 1; }
  }
</style>
