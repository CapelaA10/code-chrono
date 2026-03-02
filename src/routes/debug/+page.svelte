<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { strings } from "$lib/i18n/store";
  import { dev } from "$app/environment";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { Bell } from "lucide-svelte";

  let permissionStatus = "unknown";
  let dbSetting = "unknown";
  let backendError = "";

  async function checkStatus() {
    backendError = "";
    try {
      // Direct command call results in true if OS says granted
      const granted = await invoke<boolean>("request_notification_permission");
      permissionStatus = granted ? "granted" : "denied";
      
      const setting = await invoke<string | null>("get_setting", { key: "notifications_enabled" });
      dbSetting = (setting === null) ? "default (ON)" : (setting === 'true' ? "ON" : "OFF");
    } catch (e: any) {
      console.error("Failed to check status:", e);
      backendError = e.message || String(e);
      permissionStatus = "error";
    }
  }

  onMount(() => {
    if (!dev) {
      goto("/");
      return;
    }
    checkStatus();
  });

  async function requestPermission() {
    backendError = "";
    try {
      const granted = await invoke<boolean>("request_notification_permission");
      permissionStatus = granted ? "granted" : "denied";
    } catch (e: any) {
      console.error("Failed to request permission:", e);
      backendError = e.message || String(e);
      permissionStatus = "error";
    }
  }

  async function testNotification() {
    backendError = "";
    try {
      await invoke("show_notification", {
        title: $strings.testNotificationTitle,
        body: $strings.testNotificationBody,
      });
    } catch (e: any) {
      console.error("Failed to send notification:", e);
      backendError = e.message || String(e);
    }
  }
</script>

<div class="debug-page">
  <header class="debug-header">
    <h1>{$strings.debug}</h1>
  </header>

  <div class="debug-content">
    <section class="debug-section">
      <h2>{$strings.testNotifications}</h2>
      <p class="section-desc">Troubleshoot native OS connectivity.</p>
      
      <div class="status-grid">
        <div class="status-item">
          <span class="label">OS Permission Status:</span>
          <span class="value" class:granted={permissionStatus === 'granted'} class:error={permissionStatus === 'error'}>
            {permissionStatus === 'error' ? 'Backend Error' : permissionStatus}
          </span>
        </div>
        <div class="status-item">
          <span class="label">App Settings Master Toggle:</span>
          <span class="value" class:off={dbSetting === 'OFF'}>{dbSetting}</span>
        </div>
      </div>

      {#if backendError}
        <div class="error-log">
          <span class="label">Backend Output:</span>
          <code>{backendError}</code>
        </div>
      {/if}

      <div class="actions">
        <button class="debug-btn secondary" on:click={requestPermission}>
          <span>Re-request Permission Popup</span>
        </button>
        
        <button class="debug-btn" on:click={testNotification}>
          <Bell size={18} />
          <span>{$strings.sendTestNotification}</span>
        </button>
      </div>

      <div class="troubleshoot-hint">
        <b>Tip:</b> If permission is "denied", go to <i>System Settings > Notifications > Code Chrono</i> and enable it manually.
      </div>

      <div class="troubleshoot-hint unsigned">
        <b>macOS Unsigned App Fix:</b> Because the app is not signed, the OS might ignore notification requests. Try changing <code>"identifier"</code> in <code>tauri.conf.json</code> briefly (e.g. to <code>com.codechrono.app.dev</code>) and restart to force the OS to see it as a new candidate for notifications.
      </div>
    </section>
  </div>
</div>

<style>
  .debug-page {
    flex: 1;
    display: flex;
    flex-direction: column;
    padding: 2rem;
    overflow-y: auto;
    background: var(--bg-page);
  }

  .debug-header {
    margin-bottom: 2rem;
  }

  .debug-header h1 {
    font-size: 1.5rem;
    font-weight: 700;
    margin: 0;
    color: var(--text);
  }

  .debug-content {
    display: flex;
    flex-direction: column;
    gap: 2rem;
    max-width: 800px;
  }

  .debug-section {
    background: var(--bg-card);
    padding: 1.5rem;
    border-radius: 12px;
    border: 1px solid var(--border);
    box-shadow: var(--shadow-sm);
  }

  .debug-section h2 {
    font-size: 1.125rem;
    font-weight: 600;
    margin: 0 0 0.5rem;
    color: var(--text);
  }

  .section-desc {
    color: var(--text-muted);
    font-size: 0.875rem;
    margin-bottom: 1.5rem;
  }

  .status-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1.5rem;
    margin-bottom: 2rem;
    padding: 1rem;
    background: var(--bg-page);
    border-radius: 8px;
    border: 1px solid var(--border);
  }

  .status-item {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .status-item .label {
    font-size: 0.75rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-muted);
  }

  .status-item .value {
    font-size: 1rem;
    font-weight: 600;
    color: var(--text);
  }

  .status-item .value.granted { color: #10b981; }
  .status-item .value.error { color: #ef4444; }
  .status-item .value.off { color: #ef4444; }

  .error-log {
    margin-bottom: 2rem;
    padding: 1rem;
    background: #fef2f2;
    border: 1px solid #fecaca;
    border-radius: 8px;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .error-log .label {
    font-size: 0.75rem;
    font-weight: 700;
    color: #991b1b;
    text-transform: uppercase;
  }

  .error-log code {
    font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    font-size: 0.8125rem;
    color: #b91c1c;
    word-break: break-all;
  }

  .actions {
    display: flex;
    gap: 1rem;
    margin-bottom: 1.5rem;
  }

  .debug-btn {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem 1.25rem;
    background: var(--btn-primary-bg);
    color: var(--btn-primary-text);
    border: none;
    border-radius: 8px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s;
  }

  .debug-btn.secondary {
    background: var(--bg-page);
    color: var(--text);
    border: 1px solid var(--border);
  }

  .debug-btn:hover {
    background: var(--btn-primary-hover);
    transform: translateY(-1px);
    box-shadow: var(--shadow);
  }

  .debug-btn.secondary:hover {
    background: var(--btn-secondary-hover-bg);
    color: var(--text);
  }

  .troubleshoot-hint {
    margin: 1.5rem 0 0;
    font-size: 0.8125rem;
    color: var(--text-muted);
    padding: 0.75rem;
    border-left: 3px solid var(--accent-blue);
    background: color-mix(in srgb, var(--accent-blue) 5%, transparent);
  }

  .debug-btn:active {
    transform: translateY(0);
  }
</style>
