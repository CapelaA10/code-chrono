<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { save, open, ask } from "@tauri-apps/plugin-dialog";
  import { writeTextFile } from "@tauri-apps/plugin-fs";
  import { onMount } from "svelte";
  import { theme } from "$lib/stores/theme";
  import { idleMinutes } from "$lib/stores/idle";
  import { timerDuration } from "$lib/stores/timerSettings";
  import { 
    Sun, 
    Moon, 
    Keyboard, 
    Clock, 
    Database, 
    Github, 
    Gitlab, 
    CloudIcon, 
    ArrowLeft,
    Info,
    Trash2,
    Download,
    Upload,
    Check
  } from "lucide-svelte";

  let exportBusy = false;
  let importBusy = false;
  let clearBusy = false;
  let message = "";
  let messageType: 'info' | 'success' | 'error' = 'info';

  let githubToken = "";
  let githubRepo = ""; // Optional: "owner/repo" for specific repo
  let gitlabToken = "";
  let gitlabHost = "";
  let jiraDomain = "";
  let jiraEmail = "";
  let jiraToken = "";

  $: isMac =
    typeof navigator !== "undefined" &&
    /Mac|iPod|iPhone|iPad/.test(navigator.platform);
  $: hotkeyLabel = isMac ? "⌘⇧P" : "Ctrl+Shift+P";

  onMount(async () => {
    message = "";
    githubToken = (await invoke<string | null>("get_setting", { key: "github_token" })) || "";
    githubRepo = (await invoke<string | null>("get_setting", { key: "github_repo" })) || "";
    gitlabToken = (await invoke<string | null>("get_setting", { key: "gitlab_token" })) || "";
    gitlabHost = (await invoke<string | null>("get_setting", { key: "gitlab_host" })) || "";
    jiraDomain = (await invoke<string | null>("get_setting", { key: "jira_domain" })) || "";
    jiraEmail = (await invoke<string | null>("get_setting", { key: "jira_email" })) || "";
    jiraToken = (await invoke<string | null>("get_setting", { key: "jira_token" })) || "";
  });

  function showMessage(msg: string, type: 'info' | 'success' | 'error' = 'success') {
    message = msg;
    messageType = type;
    setTimeout(() => { if (message === msg) message = ""; }, 3000);
  }

  async function saveSetting(key: string, value: string) {
    try {
      await invoke("set_setting", { key, value });
      showMessage("Setting saved successfully");
    } catch (e) {
      showMessage("Error: " + e, 'error');
    }
  }

  async function saveJiraSettings() {
    try {
      await invoke("set_setting", { key: "jira_domain", value: jiraDomain });
      await invoke("set_setting", { key: "jira_email", value: jiraEmail });
      await invoke("set_setting", { key: "jira_token", value: jiraToken });
      showMessage("Jira settings updated");
    } catch (e) {
      showMessage("Error: " + e, 'error');
    }
  }

  async function exportCSV() {
    exportBusy = true;
    try {
      const csv = await invoke<string>("export_csv");
      const filePath = await save({
        filters: [{ name: "CSV", extensions: ["csv"] }],
      });
      if (filePath) {
        await writeTextFile(filePath, csv);
        showMessage("Export complete", 'success');
      }
    } catch (error) {
      showMessage("Export failed: " + error, 'error');
    } finally {
      exportBusy = false;
    }
  }

  async function importCSV() {
    importBusy = true;
    try {
      const filePath = await open({
        multiple: false,
        filters: [{ name: "CSV", extensions: ["csv"] }],
      });
      if (filePath && typeof filePath === "string") {
        const count = await invoke<number>("import_csv", { path: filePath });
        showMessage(`Imported ${count} session(s)`, 'success');
      }
    } catch (error) {
      showMessage("Import failed: " + error, 'error');
    } finally {
      importBusy = false;
    }
  }

  let isConfirming = false;

  async function clearAllData() {
    if (clearBusy || isConfirming) return;
    isConfirming = true;
    const ok = await ask(
      "Clear all session data? This cannot be undone. Your timer will not be affected.",
      { title: "Confirm deletion", kind: "warning" }
    );
    isConfirming = false;
    if (!ok) return;

    clearBusy = true;
    try {
      await invoke("reset_database");
      showMessage("All session data cleared", 'success');
    } catch (error) {
      showMessage("Failed to clear data: " + error, 'error');
    } finally {
      clearBusy = false;
    }
  }

  function setTheme(t: "light" | "dark") {
    theme.set(t);
  }

  function setIdleMinutes(v: number) {
    const n = Math.max(0, Math.min(60, Math.round(v)));
    idleMinutes.set(n);
  }

  const DURATIONS = [5, 10, 15, 20, 25, 30, 45, 60, 90, 120];
</script>

<main class="settings-container">
  <div class="settings-wrapper">
    <div class="settings-header">
      <a href="/" class="back-link">
        <ArrowLeft size={20} />
        <span>Dashboard</span>
      </a>
      <h1 class="settings-title">Preferences</h1>
      <p class="settings-subtitle">Customize your experience and manage integrations</p>
    </div>

    <div class="settings-grid">
      <!-- Appearance Section -->
      <div class="settings-card">
        <div class="card-header">
          <div class="header-icon appearance">
            <Sun size={20} />
          </div>
          <div class="header-text">
            <h3>Appearance</h3>
            <p>Personalize how the application looks</p>
          </div>
        </div>
        <div class="card-content">
          <span class="input-label">Interface Theme</span>
          <div class="theme-switcher">
            <button
              class="theme-opt"
              class:active={$theme === "light"}
              on:click={() => setTheme("light")}
            >
              <Sun size={18} />
              <span>Light</span>
            </button>
            <button
              class="theme-opt"
              class:active={$theme === "dark"}
              on:click={() => setTheme("dark")}
            >
              <Moon size={18} />
              <span>Dark</span>
            </button>
          </div>
        </div>
      </div>

      <!-- Productivity Section -->
      <div class="settings-card">
        <div class="card-header">
          <div class="header-icon productivity">
            <Keyboard size={20} />
          </div>
          <div class="header-text">
            <h3>Productivity</h3>
            <p>Shortcuts and automated behaviors</p>
          </div>
        </div>
        <div class="card-content">
          <div class="setting-row">
            <div class="row-info">
              <span class="row-label">Global Toggle</span>
              <span class="row-hint">Pause/Resume from anywhere</span>
            </div>
            <kbd class="shortcut-tag">{hotkeyLabel}</kbd>
          </div>
          
          <div class="divider"></div>

          <div class="setting-row">
            <div class="row-info">
              <span class="row-label">Auto-Pause (Idle)</span>
              <span class="row-hint">Time until auto-pause when inactive</span>
            </div>
            <div class="number-input-group">
              <input
                type="number"
                min="0"
                max="60"
                value={$idleMinutes}
                on:input={(e) => setIdleMinutes(Number((e.currentTarget as HTMLInputElement).value))}
                class="premium-input number"
              />
              <span class="unit-label">min</span>
            </div>
          </div>

          <div class="divider"></div>

          <div class="setting-row">
            <div class="row-info">
              <span class="row-label">Default Timer Duration</span>
              <span class="row-hint">Standard length of a Pomodoro session</span>
            </div>
            <select bind:value={$timerDuration} class="premium-input select">
              {#each DURATIONS as d}
                <option value={d}>{d} minutes</option>
              {/each}
            </select>
          </div>
        </div>
      </div>

      <!-- Integrations Section -->
      <div class="settings-card full-width">
        <div class="card-header">
          <div class="header-icon integration">
            <CloudIcon size={20} />
          </div>
          <div class="header-text">
            <h3>Integrations</h3>
            <p>Connect with external platforms to sync your tasks</p>
          </div>
        </div>
        <div class="card-content">
          <div class="integrations-grid">
            <!-- GitHub Card -->
            <div class="integration-subcard">
              <div class="subcard-title">
                <Github size={18} />
                <span>GitHub</span>
              </div>
              <div class="field">
                <label for="gh-token">Personal Access Token</label>
                <p class="field-hint">Needs <code>repo</code> scope to read issues</p>
                <input id="gh-token" type="password" bind:value={githubToken} placeholder="ghp_..." class="premium-input-field mb-2" />
                
                <label for="gh-repo">Repository (optional)</label>
                <p class="field-hint">Format: <code>owner/repo</code> — leave empty for all assigned issues</p>
                <div class="input-with-action">
                  <input id="gh-repo" type="text" bind:value={githubRepo} placeholder="owner/repo" class="premium-input-field" />
                  <button class="save-icon-btn" on:click={async () => {
                    await saveSetting('github_token', githubToken);
                    await saveSetting('github_repo', githubRepo);
                  }} title="Save GitHub Settings">
                    <Check size={18} />
                  </button>
                </div>
              </div>
            </div>

            <!-- GitLab Card -->
            <div class="integration-subcard">
              <div class="subcard-title">
                <Gitlab size={18} />
                <span>GitLab</span>
              </div>
              <div class="field">
                <label for="gl-host">GitLab Host</label>
                <input id="gl-host" type="text" bind:value={gitlabHost} placeholder="https://gitlab.com" class="premium-input-field mb-2" />
                
                <label for="gl-token">Personal Access Token</label>
                <div class="input-with-action">
                  <input id="gl-token" type="password" bind:value={gitlabToken} placeholder="glpat-..." class="premium-input-field" />
                  <button class="save-icon-btn" on:click={async () => {
                    await saveSetting('gitlab_host', gitlabHost);
                    await saveSetting('gitlab_token', gitlabToken);
                  }} title="Save GitLab Settings">
                    <Check size={18} />
                  </button>
                </div>
              </div>
            </div>

            <!-- Jira Card -->
            <div class="integration-subcard">
              <div class="subcard-title">
                <span>Jira</span>
              </div>
              <div class="field">
                <label for="jr-domain">Domain</label>
                <input id="jr-domain" type="text" bind:value={jiraDomain} placeholder="company.atlassian.net" class="premium-input-field mb-2" />
                
                <label for="jr-email">Email</label>
                <input id="jr-email" type="text" bind:value={jiraEmail} placeholder="name@company.com" class="premium-input-field mb-2" />
                
                <label for="jr-token">API Token</label>
                <div class="input-with-action">
                  <input id="jr-token" type="password" bind:value={jiraToken} placeholder="ATATT..." class="premium-input-field" />
                  <button class="save-icon-btn" on:click={saveJiraSettings} title="Save Jira Settings">
                    <Check size={18} />
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Data Management Section -->
      <div class="settings-card">
        <div class="card-header">
          <div class="header-icon management">
            <Database size={20} />
          </div>
          <div class="header-text">
            <h3>Data Management</h3>
            <p>Import or export your time records</p>
          </div>
        </div>
        <div class="card-content">
          <div class="action-grid">
            <button class="premium-btn primary" on:click={exportCSV} disabled={exportBusy}>
              <Download size={18} />
              <span>{exportBusy ? "Exporting..." : "Export CSV"}</span>
            </button>
            <button class="premium-btn secondary" on:click={importCSV} disabled={importBusy}>
              <Upload size={18} />
              <span>{importBusy ? "Importing..." : "Import CSV"}</span>
            </button>
          </div>
        </div>
      </div>

      <!-- Danger Zone Section -->
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
            <span>{clearBusy ? "Clearing..." : "Reset Database"}</span>
          </button>
        </div>
      </div>
    </div>
  </div>
</main>

  {#if message}
    <button class="toast {messageType}" on:click={() => message = ""} aria-label="Dismiss">
      <div class="toast-content">
        {#if messageType === 'success'}
          <Check size={18} />
        {:else if messageType === 'error'}
          <Info size={18} />
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
    margin: 0;
    padding: 0;
  }

  .settings-container {
    flex: 1;
    overflow-y: auto;
    width: 100%;
    padding: 3rem 2rem;
    box-sizing: border-box;
    height: 100vh;
  }

  .settings-wrapper {
    max-width: 900px;
    margin: 0 auto;
  }

  .settings-header {
    margin-bottom: 3rem;
  }

  .back-link {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: var(--text-muted);
    text-decoration: none;
    font-size: 0.875rem;
    font-weight: 500;
    margin-bottom: 1.5rem;
    transition: color 0.2s;
  }

  .back-link:hover {
    color: var(--accent-blue);
  }

  .settings-title {
    font-size: 2.5rem;
    font-weight: 800;
    margin: 0;
    letter-spacing: -0.02em;
    color: var(--text);
  }

  .settings-subtitle {
    color: var(--text-muted);
    font-size: 1.125rem;
    margin: 0.5rem 0 0 0;
  }

  .settings-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 1.5rem;
  }

  .settings-card {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 1.25rem;
    padding: 1.5rem;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.03);
    transition: transform 0.2s, box-shadow 0.2s;
  }

  .settings-card:hover {
    box-shadow: 0 8px 30px rgba(0, 0, 0, 0.05);
  }

  .full-width {
    grid-column: span 2;
  }

  .card-header {
    display: flex;
    gap: 1rem;
    margin-bottom: 2rem;
  }

  .header-icon {
    width: 42px;
    height: 42px;
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
  }

  .header-icon.appearance { background: rgba(59, 130, 246, 0.1); color: #3b82f6; }
  .header-icon.productivity { background: rgba(16, 185, 129, 0.1); color: #10b981; }
  .header-icon.integration { background: rgba(139, 92, 246, 0.1); color: #8b5cf6; }
  .header-icon.management { background: rgba(245, 158, 11, 0.1); color: #f59e0b; }
  .header-icon.danger { background: rgba(239, 68, 68, 0.1); color: #ef4444; }

  .header-text h3 {
    margin: 0;
    font-size: 1.125rem;
    font-weight: 700;
    color: var(--text);
  }

  .header-text p {
    margin: 0.25rem 0 0 0;
    font-size: 0.8125rem;
    color: var(--text-muted);
  }

  .card-content {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  /* Theme Switcher */
  .theme-switcher {
    display: flex;
    padding: 4px;
    background: var(--bg-page);
    border: 1px solid var(--border);
    border-radius: 12px;
    gap: 4px;
  }

  .theme-opt {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 0.625rem;
    border: none;
    background: transparent;
    color: var(--text-muted);
    font-size: 0.875rem;
    font-weight: 500;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s;
  }

  .theme-opt.active {
    background: var(--bg-card);
    color: var(--text);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }

  .input-label {
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-muted);
    margin-bottom: 0.25rem;
  }

  /* Setting Rows */
  .setting-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .row-label {
    display: block;
    font-size: 0.9375rem;
    font-weight: 600;
    color: var(--text);
  }

  .row-hint {
    display: block;
    font-size: 0.75rem;
    color: var(--text-muted);
  }

  .shortcut-tag {
    background: var(--bg-page);
    border: 1px solid var(--border);
    padding: 0.4rem 0.75rem;
    border-radius: 8px;
    font-family: inherit;
    font-size: 0.8125rem;
    font-weight: 600;
    color: var(--text);
    box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.1);
  }

  .divider {
    height: 1px;
    background: var(--border);
    margin: 0.5rem 0;
  }

  /* Inputs */
  .number-input-group {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .premium-input {
    background: var(--bg-page);
    border: 1px solid var(--border);
    border-radius: 10px;
    padding: 0.625rem;
    color: var(--text);
    font-size: 0.9375rem;
    outline: none;
    transition: border-color 0.2s, box-shadow 0.2s;
  }

  .premium-input:focus {
    border-color: var(--accent-blue);
    box-shadow: 0 0 0 3px var(--focus-ring);
  }

  .premium-input.number { width: 4rem; text-align: center; }
  .premium-input.select {
    cursor: pointer;
    appearance: none;
    padding-right: 2rem;
    background-image: url("data:image/svg+xml;charset=US-ASCII,%3Csvg xmlns%3D'http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg' width%3D'292' height%3D'292'%3E%3Cpath fill%3D'%239CA3AF' d%3D'M287 69a18 18 0 0 0-13-5H18a18 18 0 0 0-13 18c0 5 2 9 5 13l128 128c4 4 8 5 13 5s9-1 13-5l128-128c4-3 5-8 5-13a18 18 0 0 0-5-13z'%2F%3E%3C%2Fsvg%3E");
    background-repeat: no-repeat;
    background-position: right 0.75rem center;
    background-size: 8px auto;
  }
  .unit-label { font-size: 0.875rem; color: var(--text-muted); }

  /* Integration Section */
  .integrations-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 1.25rem;
  }

  .integration-subcard {
    background: var(--bg-page);
    border: 1px solid var(--border);
    border-radius: 14px;
    padding: 1.25rem;
  }

  .subcard-title {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    font-weight: 700;
    margin-bottom: 1.25rem;
    color: var(--text);
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
  }

  .field label {
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--text-muted);
  }

  .field-hint {
    font-size: 0.7rem;
    color: var(--text-muted);
    margin: 0;
    line-height: 1.4;
    opacity: 0.8;
  }

  .field-hint code {
    font-family: 'SF Mono', 'Roboto Mono', monospace;
    background: var(--btn-secondary-hover-bg);
    padding: 0.1em 0.3em;
    border-radius: 3px;
    font-size: 0.85em;
  }

  .premium-input-field {
    width: 100%;
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 10px;
    padding: 0.625rem;
    color: var(--text);
    font-size: 0.875rem;
    outline: none;
    transition: all 0.2s;
    box-sizing: border-box;
  }

  .premium-input-field:focus {
    border-color: var(--accent-blue);
    background: var(--bg-page);
  }

  .input-with-action {
    display: flex;
    gap: 0.5rem;
  }

  .save-icon-btn {
    width: 38px;
    height: 38px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 10px;
    border: 1px solid var(--accent-blue-border, #3b82f633);
    background: var(--accent-blue-hover, #3b82f611);
    color: var(--accent-blue);
    cursor: pointer;
    transition: all 0.2s;
    flex-shrink: 0;
  }

  .save-icon-btn:hover {
    background: var(--accent-blue);
    color: white;
  }

  .mb-2 { margin-bottom: 1rem; }

  /* Premium Buttons */
  .action-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
  }

  .premium-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.625rem;
    padding: 0.75rem;
    border-radius: 12px;
    font-size: 0.875rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
    border: none;
  }

  .premium-btn.primary {
    background: var(--accent-blue);
    color: white;
  }

  .premium-btn.primary:hover:not(:disabled) {
    background: #2563eb;
    transform: translateY(-1px);
  }

  .premium-btn.secondary {
    background: var(--bg-page);
    border: 1px solid var(--border);
    color: var(--text);
  }

  .premium-btn.secondary:hover:not(:disabled) {
    background: var(--btn-secondary-hover-bg);
  }

  .premium-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  /* Danger Card */
  .danger-card {
    border-color: rgba(239, 68, 68, 0.2);
  }

  .danger-text {
    font-size: 0.8125rem;
    color: var(--text-muted);
    line-height: 1.5;
    margin: 0;
  }

  .danger-btn {
    background: rgba(239, 68, 68, 0.1);
    color: #ef4444;
    border: 1px solid rgba(239, 68, 68, 0.2);
    margin-top: 0.5rem;
  }

  .danger-btn:hover:not(:disabled) {
    background: #ef4444;
    color: white;
  }

  /* Toast */
  .toast {
    position: fixed;
    bottom: 2rem;
    left: 50%;
    transform: translateX(-50%);
    background: #1f2937;
    color: white;
    padding: 0.75rem 1.5rem;
    border-radius: 99px;
    border: none;
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
    z-index: 1000;
    cursor: pointer;
    animation: slideUp 0.4s cubic-bezier(0.16, 1, 0.3, 1);
  }

  .toast.success { background: #10b981; }
  .toast.error { background: #ef4444; }

  .toast-content {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    font-size: 0.875rem;
    font-weight: 600;
  }

  @keyframes slideUp {
    from { transform: translate(-50%, 2rem); opacity: 0; }
    to { transform: translate(-50%, 0); opacity: 1; }
  }

  @media (max-width: 800px) {
    .settings-grid {
      grid-template-columns: 1fr;
    }
    .full-width {
      grid-column: span 1;
    }
    .integrations-grid {
      grid-template-columns: 1fr;
    }
  }

  .settings-container::-webkit-scrollbar {
    width: 8px;
  }

  .settings-container::-webkit-scrollbar-track {
    background: transparent;
  }

  .settings-container::-webkit-scrollbar-thumb {
    background: var(--border);
    border-radius: 4px;
  }

  .settings-container::-webkit-scrollbar-thumb:hover {
    background: var(--text-muted);
  }
</style>
