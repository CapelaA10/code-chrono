<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { refreshTasks } from '$lib/stores/tasks';
  import { RefreshCw, CheckCircle, AlertCircle } from 'lucide-svelte';

  type SyncResult = { count: number; error?: string } | null;

  let syncState: Record<string, 'idle' | 'syncing' | 'success' | 'error'> = {
    GitHub: 'idle',
    GitLab: 'idle',
    Jira: 'idle'
  };
  let syncMessages: Record<string, string> = {};

  async function syncIntegration(name: string) {
    if (syncState[name] === 'syncing') return;
    syncState[name] = 'syncing';
    syncMessages[name] = '';

    try {
      let count = 0;
      if (name === 'GitHub') {
        count = await invoke<number>('sync_github');
      } else if (name === 'Jira') {
        count = await invoke<number>('sync_jira');
      } else if (name === 'GitLab') {
        count = await invoke<number>('sync_gitlab');
      }

      syncState[name] = 'success';
      syncMessages[name] = count > 0 ? `${count} task${count !== 1 ? 's' : ''} synced` : 'Up to date';
      refreshTasks();
    } catch (e: any) {
      syncState[name] = 'error';
      // Extract a cleaner error message
      const raw = String(e);
      if (raw.includes('token not found') || raw.includes('not found')) {
        syncMessages[name] = 'Not configured — check Settings';
      } else if (raw.includes('401') || raw.includes('Unauthorized')) {
        syncMessages[name] = 'Invalid token';
      } else if (raw.includes('404')) {
        syncMessages[name] = 'Resource not found';
      } else {
        syncMessages[name] = 'Sync failed';
      }
    } finally {
      // Reset to idle after 3 seconds
      setTimeout(() => {
        syncState[name] = 'idle';
        syncMessages[name] = '';
        syncState = { ...syncState };
      }, 3000);
      syncState = { ...syncState };
    }
  }
</script>

<div class="nav-section">
  <div class="section-header">
    <h3>Integrations</h3>
  </div>
  <div class="section-list">
    <!-- GitHub -->
    <div class="sync-item">
      <button
        class="nav-item"
        class:syncing={syncState.GitHub === 'syncing'}
        on:click={() => syncIntegration('GitHub')}
        disabled={syncState.GitHub === 'syncing'}
        title="Sync GitHub issues"
      >
        <svg viewBox="0 0 24 24" width="18" height="18" fill="currentColor" style="flex-shrink:0">
          <path d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.43.372.823 1.102.823 2.222 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12"/>
        </svg>
        <span class="label">GitHub</span>
        {#if syncState.GitHub === 'syncing'}
          <RefreshCw size={14} class="spin" />
        {:else if syncState.GitHub === 'success'}
          <CheckCircle size={14} class="success-icon" />
        {:else if syncState.GitHub === 'error'}
          <AlertCircle size={14} class="error-icon" />
        {/if}
      </button>
      {#if syncMessages.GitHub}
        <div class="sync-msg" class:is-error={syncState.GitHub === 'error'}>
          {syncMessages.GitHub}
        </div>
      {/if}
    </div>

    <!-- GitLab -->
    <div class="sync-item">
      <button
        class="nav-item"
        class:syncing={syncState.GitLab === 'syncing'}
        on:click={() => syncIntegration('GitLab')}
        disabled={syncState.GitLab === 'syncing'}
        title="Sync GitLab issues"
      >
        <svg viewBox="0 0 24 24" width="18" height="18" fill="currentColor">
          <path d="M23.955 13.587l-1.342-4.135-2.664-8.189c-.135-.417-.724-.417-.859 0L16.425 9.452H7.575L4.91 1.263c-.135-.417-.724-.417-.859 0L1.387 9.452.045 13.587c-.114.353.011.737.311.955L12 23.054l11.644-8.512c.3-.218.425-.602.311-.955z"/>
        </svg>
        <span class="label">GitLab</span>
        {#if syncState.GitLab === 'syncing'}
          <RefreshCw size={14} class="spin" />
        {:else if syncState.GitLab === 'success'}
          <CheckCircle size={14} class="success-icon" />
        {:else if syncState.GitLab === 'error'}
          <AlertCircle size={14} class="error-icon" />
        {/if}
      </button>
      {#if syncMessages.GitLab}
        <div class="sync-msg" class:is-error={syncState.GitLab === 'error'}>
          {syncMessages.GitLab}
        </div>
      {/if}
    </div>

    <!-- Jira -->
    <div class="sync-item">
      <button
        class="nav-item"
        class:syncing={syncState.Jira === 'syncing'}
        on:click={() => syncIntegration('Jira')}
        disabled={syncState.Jira === 'syncing'}
        title="Sync Jira issues"
      >
        <svg viewBox="0 0 24 24" width="18" height="18" fill="#0052CC">
          <path d="M11.75 0C5.26 0 0 5.26 0 11.75s5.26 11.75 11.75 11.75S23.5 18.24 23.5 11.75 18.24 0 11.75 0zm5.02 12.56l-5.5 5.5a.88.88 0 01-1.24 0l-5.5-5.5a.88.88 0 010-1.24l5.5-5.5a.88.88 0 011.24 0l5.5 5.5a.88.88 0 010 1.24z"/>
        </svg>
        <span class="label">Jira</span>
        {#if syncState.Jira === 'syncing'}
          <RefreshCw size={14} class="spin" />
        {:else if syncState.Jira === 'success'}
          <CheckCircle size={14} class="success-icon" />
        {:else if syncState.Jira === 'error'}
          <AlertCircle size={14} class="error-icon" />
        {/if}
      </button>
      {#if syncMessages.Jira}
        <div class="sync-msg" class:is-error={syncState.Jira === 'error'}>
          {syncMessages.Jira}
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .nav-section {
    padding: 0.5rem 0.75rem;
  }

  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
    padding: 0 0.5rem;
  }

  .section-header h3 {
    font-size: 0.6875rem;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: var(--text-muted);
    font-weight: 600;
    margin: 0;
  }

  .section-list {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .sync-item {
    display: flex;
    flex-direction: column;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.625rem 0.75rem;
    border: none;
    background: none;
    color: var(--text);
    font-size: 0.875rem;
    cursor: pointer;
    text-align: left;
    border-radius: 8px;
    width: 100%;
    transition: background 0.15s;
    font-family: inherit;
  }

  .nav-item:hover:not(:disabled) {
    background: var(--btn-secondary-hover-bg);
  }

  .nav-item:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }

  .label {
    flex: 1;
  }

  :global(.spin) {
    animation: spin 0.8s linear infinite;
    color: var(--accent-blue);
  }

  :global(.success-icon) {
    color: var(--accent-green);
  }

  :global(.error-icon) {
    color: var(--error-red);
  }

  .sync-msg {
    font-size: 0.7rem;
    color: var(--accent-green);
    padding: 0.15rem 0.75rem 0.35rem 2.5rem;
    line-height: 1;
  }

  .sync-msg.is-error {
    color: var(--error-red);
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }
</style>
