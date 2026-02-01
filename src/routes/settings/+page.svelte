<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { save, open } from "@tauri-apps/plugin-dialog";
  import { writeTextFile } from "@tauri-apps/plugin-fs";
  import Icons from "$lib/components/Icons.svelte";
  import { theme } from "$lib/stores/theme";
  import { idleMinutes } from "$lib/stores/idle";

  let exportBusy = false;
  let importBusy = false;
  let clearBusy = false;
  let message = "";

  $: isMac = typeof navigator !== "undefined" && /Mac|iPod|iPhone|iPad/.test(navigator.platform);
  $: hotkeyLabel = isMac ? "⌘⇧P" : "Ctrl+Shift+P";

  async function exportCSV() {
    exportBusy = true;
    message = "";
    try {
      const csv = await invoke<string>("export_csv");
      const filePath = await save({
        filters: [{ name: "CSV", extensions: ["csv"] }],
      });
      if (filePath) {
        await writeTextFile(filePath, csv);
        message = "Export complete.";
      }
    } catch (error) {
      message = "Export failed: " + error;
    } finally {
      exportBusy = false;
    }
  }

  async function importCSV() {
    importBusy = true;
    message = "";
    try {
      const filePath = await open({
        multiple: false,
        filters: [{ name: "CSV", extensions: ["csv"] }],
      });
      if (filePath && typeof filePath === "string") {
        const count = await invoke<number>("import_csv", { path: filePath });
        message = `Imported ${count} session(s).`;
      }
    } catch (error) {
      message = "Import failed: " + error;
    } finally {
      importBusy = false;
    }
  }

  async function clearAllData() {
    const ok = confirm(
      "Clear all session data? This cannot be undone. Your timer will not be affected."
    );
    if (!ok) return;
    clearBusy = true;
    message = "";
    try {
      await invoke("reset_database");
      message = "All session data cleared.";
    } catch (error) {
      message = "Failed to clear data: " + error;
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
</script>

<main class="page">
  <div class="settings">
    <a href="/" class="back">
      <Icons name="back" size={18} className="back-icon" />
      Back to timer
    </a>
    <h1 class="title">Settings</h1>

    <section class="section">
      <h2 class="section-title">Appearance</h2>
      <div class="theme-toggle">
        <button
          class="btn theme-btn"
          class:active={$theme === "light"}
          on:click={() => setTheme("light")}
        >
          <Icons name="sun" size={18} />
          Light
        </button>
        <button
          class="btn theme-btn"
          class:active={$theme === "dark"}
          on:click={() => setTheme("dark")}
        >
          <Icons name="moon" size={18} />
          Dark
        </button>
      </div>
    </section>

    <section class="section">
      <h2 class="section-title">Shortcuts</h2>
      <p class="hint">Global hotkey (works when app is in background):</p>
      <p class="shortcut"><kbd>{hotkeyLabel}</kbd> — Toggle pause</p>
    </section>

    <section class="section">
      <h2 class="section-title">Idle detection</h2>
      <p class="hint">Pause timer after this many minutes with the window hidden (0 = off).</p>
      <div class="idle-row">
        <input
          type="number"
          min="0"
          max="60"
          value={$idleMinutes}
          on:input={(e) => setIdleMinutes(Number((e.currentTarget as HTMLInputElement).value))}
          class="idle-input"
        />
        <span class="idle-suffix">min</span>
      </div>
    </section>

    <section class="section">
      <h2 class="section-title">Data</h2>
      <div class="buttons">
        <button
          class="btn export"
          on:click={exportCSV}
          disabled={exportBusy}
        >
          <Icons name="download" size={18} className="btn-icon" />
          {exportBusy ? "…" : "Export CSV"}
        </button>
        <button
          class="btn import"
          on:click={importCSV}
          disabled={importBusy}
        >
          <Icons name="upload" size={18} className="btn-icon" />
          {importBusy ? "…" : "Import CSV"}
        </button>
      </div>
    </section>

    <section class="section danger">
      <h2 class="section-title">Danger zone</h2>
      <p class="hint">
        Reset database deletes all saved sessions. The current timer is not affected.
      </p>
      <button
        class="btn clear"
        on:click={clearAllData}
        disabled={clearBusy}
      >
        <Icons name="trash2" size={18} className="btn-icon" />
        {clearBusy ? "…" : "Clear all data"}
      </button>
    </section>

    <section class="section placeholder">
      <h2 class="section-title">Integrations</h2>
      <p class="hint">GitHub & Jira sync — coming soon.</p>
    </section>

    <section class="section placeholder">
      <h2 class="section-title">Sync</h2>
      <p class="hint">WebDAV backup — coming soon.</p>
    </section>

    {#if message}
      <p class="message">{message}</p>
    {/if}
  </div>
</main>

<style>
  .page {
    width: 100%;
    max-width: 360px;
    padding: 1rem;
  }

  :global(body) {
    margin: 0;
    padding: 0;
    min-height: 100vh;
    background: var(--bg-page);
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text);
  }

  .settings {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 12px;
    padding: 1.5rem 1.25rem;
    box-shadow: var(--shadow);
  }

  .back {
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;
    color: var(--text-muted);
    text-decoration: none;
    font-size: 0.8125rem;
    margin-bottom: 1rem;
    transition: color 0.15s;
  }

  .back:hover {
    color: var(--text);
  }

  .back :global(.back-icon),
  .back :global(svg) {
    flex-shrink: 0;
  }

  .title {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--text);
    margin: 0 0 1.25rem 0;
  }

  .section {
    margin-bottom: 1.25rem;
  }

  .section.danger {
    padding-top: 1rem;
    border-top: 1px solid var(--border);
  }

  .section.placeholder {
    opacity: 0.85;
  }

  .section-title {
    font-size: 0.6875rem;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.1em;
    margin: 0 0 0.5rem 0;
  }

  .hint {
    font-size: 0.8125rem;
    color: var(--text-muted);
    margin: 0 0 0.5rem 0;
    line-height: 1.4;
  }

  .theme-toggle {
    display: flex;
    gap: 0.5rem;
  }

  .theme-btn {
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;
    padding: 0.5rem 1rem;
    border: 1px solid var(--border);
    border-radius: 8px;
    font-size: 0.875rem;
    background: var(--bg-card);
    color: var(--text-muted);
    cursor: pointer;
    transition: background 0.15s, color 0.15s, border-color 0.15s;
  }

  .theme-btn:hover {
    color: var(--text);
    background: var(--btn-secondary-hover-bg);
  }

  .theme-btn.active {
    border-color: var(--accent-blue);
    color: var(--accent-blue);
    background: var(--accent-blue-hover);
  }

  .shortcut {
    font-size: 0.875rem;
    color: var(--text);
    margin: 0.25rem 0 0 0;
  }

  kbd {
    padding: 0.2rem 0.5rem;
    border-radius: 4px;
    border: 1px solid var(--border);
    background: var(--btn-secondary-hover-bg);
    font-family: ui-monospace, monospace;
    font-size: 0.8125rem;
  }

  .idle-row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-top: 0.25rem;
  }

  .idle-input {
    width: 4rem;
    padding: 0.4rem 0.5rem;
    border: 1px solid var(--input-border);
    border-radius: 8px;
    font-size: 0.875rem;
    background: var(--input-bg);
    color: var(--text);
  }

  .idle-input:focus {
    outline: none;
    border-color: var(--accent-blue);
  }

  .idle-suffix {
    font-size: 0.875rem;
    color: var(--text-muted);
  }

  .buttons {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .btn {
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;
    padding: 0.5rem 1rem;
    border: 1px solid var(--border);
    border-radius: 8px;
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.15s, border-color 0.15s, color 0.15s;
    background: var(--bg-card);
    color: var(--btn-secondary-text);
  }

  .btn :global(svg) {
    flex-shrink: 0;
  }

  .btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .btn.export {
    color: var(--accent-blue);
    border-color: var(--accent-blue-border);
  }

  .btn.export:hover:not(:disabled) {
    background: var(--accent-blue-hover);
  }

  .btn.import {
    color: var(--accent-green);
    border-color: var(--accent-green-border);
  }

  .btn.import:hover:not(:disabled) {
    background: var(--accent-green-hover);
  }

  .btn.clear {
    color: var(--accent-red);
    border-color: var(--accent-red-border);
  }

  .btn.clear:hover:not(:disabled) {
    background: var(--accent-red-hover);
  }

  .message {
    font-size: 0.8125rem;
    color: var(--text-muted);
    margin: 1rem 0 0 0;
  }
</style>
