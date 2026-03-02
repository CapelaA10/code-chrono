<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { strings } from '$lib/i18n/store';
  import { timerDuration } from '$lib/stores/timerSettings';
  import { X } from 'lucide-svelte';

  // { name, executable } payload from Tauri events
  interface ProgramEvent { name: string; executable: string }

  let visible = false;
  let programName = '';
  let currentExecutable = '';   // track which program the modal is showing
  let taskNames: string[] = [];
  let selectedTask = '';

  let unlistenOpened: (() => void) | null = null;
  let unlistenClosed:  (() => void) | null = null;

  onMount(async () => {
    unlistenOpened = await listen<ProgramEvent>('program-opened', async (event) => {
      programName       = event.payload.name;
      currentExecutable = event.payload.executable;
      taskNames         = await invoke<string[]>('get_unique_task_names', { limit: 20 });
      selectedTask      = taskNames[0] ?? '';
      visible           = true;
    });

    // Auto-dismiss if the program is closed while the modal is still open.
    unlistenClosed = await listen<ProgramEvent>('program-closed', (event) => {
      if (visible && event.payload.executable === currentExecutable) {
        visible = false;
      }
    });
  });

  onDestroy(() => {
    unlistenOpened?.();
    unlistenClosed?.();
  });

  async function startTracking() {
    if (!selectedTask) return;
    await invoke('start_pomodoro', {
      taskName: selectedTask,
      durationMinutes: $timerDuration,
      notifStarted:       $strings.notifTimerStart,
      notifComplete:      $strings.notifTimerEnd,
      notifBreakOver:     $strings.notifBreakOver,
      notifBreakTitle:    $strings.notifBreakTitle,
      notifBreakRecommend: $strings.notifBreakTime,
    });
    visible = false;
  }

  function dismiss() { visible = false; }

  // Close on backdrop click
  function onBackdrop(e: MouseEvent) {
    if ((e.target as HTMLElement).classList.contains('modal-backdrop')) dismiss();
  }
</script>

{#if visible}
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div class="modal-backdrop" on:click={onBackdrop}>
    <div class="modal" role="dialog" aria-modal="true">
      <div class="modal-header">
        <div class="icon-badge">💻</div>
        <div class="modal-title">
          <h3>{programName} {$strings.programOpened.replace('{name}', '') || $strings.programOpened}</h3>
          <p class="subtitle">{$strings.whichTask}</p>
        </div>
        <button class="close-btn" on:click={dismiss}><X size={16} /></button>
      </div>

      <div class="modal-body">
        {#if taskNames.length > 0}
          <select class="task-select" bind:value={selectedTask}>
            {#each taskNames as name}
              <option value={name}>{name}</option>
            {/each}
          </select>
        {:else}
          <input
            class="task-input"
            type="text"
            placeholder={$strings.selectTask}
            bind:value={selectedTask}
          />
        {/if}
      </div>

      <div class="modal-footer">
        <button class="btn-dismiss" on:click={dismiss}>{$strings.dismiss}</button>
        <button class="btn-start" on:click={startTracking} disabled={!selectedTask}>
          {$strings.startTracking}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-backdrop {
    position: fixed; inset: 0; z-index: 9999;
    background: rgba(0,0,0,0.45);
    display: flex; align-items: flex-end; justify-content: flex-end;
    padding: 1.5rem;
    animation: fade-in 0.15s ease;
  }

  @keyframes fade-in {
    from { opacity: 0; }
    to   { opacity: 1; }
  }

  .modal {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 1.25rem;
    box-shadow: var(--shadow-lg, 0 20px 60px rgba(0,0,0,0.25));
    padding: 1.5rem;
    width: 360px;
    max-width: 100%;
    animation: slide-up 0.2s cubic-bezier(0.22,1,0.36,1);
  }

  @keyframes slide-up {
    from { transform: translateY(16px); opacity: 0; }
    to   { transform: translateY(0);    opacity: 1; }
  }

  .modal-header {
    display: flex; align-items: flex-start;
    gap: 0.75rem; margin-bottom: 1.25rem;
  }

  .icon-badge {
    font-size: 1.5rem; flex-shrink: 0;
    width: 38px; height: 38px;
    background: color-mix(in srgb, var(--accent-blue) 10%, transparent);
    border-radius: 10px;
    display: flex; align-items: center; justify-content: center;
  }

  .modal-title { flex: 1; }
  .modal-title h3 { margin: 0; font-size: 1rem; font-weight: 700; color: var(--text); }
  .subtitle { margin: 0.25rem 0 0; font-size: 0.8125rem; color: var(--text-muted); }

  .close-btn {
    background: none; border: none; cursor: pointer; padding: 0.25rem;
    color: var(--text-muted); border-radius: 6px;
    display: flex; align-items: center; transition: var(--transition);
  }
  .close-btn:hover { background: var(--btn-secondary-hover-bg); color: var(--text); }

  .modal-body { margin-bottom: 1.25rem; }

  .task-select, .task-input {
    width: 100%; box-sizing: border-box;
    background: var(--bg-page); border: 1px solid var(--border);
    border-radius: 10px; padding: 0.625rem 0.875rem;
    color: var(--text); font-size: 0.9375rem; font-family: inherit;
    outline: none; transition: border-color 0.2s, box-shadow 0.2s;
    appearance: auto;
  }
  .task-select:focus, .task-input:focus {
    border-color: var(--accent-blue);
    box-shadow: 0 0 0 3px var(--focus-ring);
  }

  .modal-footer {
    display: flex; gap: 0.75rem; justify-content: flex-end;
  }

  .btn-dismiss {
    background: var(--bg-page); border: 1px solid var(--border);
    border-radius: 8px; padding: 0.5rem 1rem;
    font-size: 0.875rem; font-weight: 600; color: var(--text);
    cursor: pointer; transition: var(--transition); font-family: inherit;
  }
  .btn-dismiss:hover { filter: brightness(0.95); }

  .btn-start {
    background: var(--accent-blue); border: none; border-radius: 8px;
    padding: 0.5rem 1.125rem; font-size: 0.875rem; font-weight: 600;
    color: white; cursor: pointer; transition: var(--transition);
    font-family: inherit;
  }
  .btn-start:hover:not(:disabled) { filter: brightness(1.1); }
  .btn-start:disabled { opacity: 0.5; cursor: default; }
</style>
