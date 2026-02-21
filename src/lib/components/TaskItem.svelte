<!--
  TaskItem.svelte
  ───────────────
  Renders a single task row. Handles user actions (toggle, start timer, edit,
  delete) and delegates heavy UI to child components.
-->
<script lang="ts">
  import type { Task } from '$lib/types';
  import { Play, Pause, Pencil, Trash2 } from 'lucide-svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { refreshTasks, projects, tags } from '$lib/stores/tasks';
  import { timerDuration } from '$lib/stores/timerSettings';
  import { activeTimer } from '$lib/stores/timer';
  import TaskCheckbox  from './task/TaskCheckbox.svelte';
  import TaskMeta      from './task/TaskMeta.svelte';
  import TaskEditModal from './task/TaskEditModal.svelte';

  export let task: Task;

  let showEditModal = false;
  let deleting = false;

  $: project = $projects.find(p => p.id === task.project_id);
  $: isThisTaskActive = $activeTimer?.task_active && $activeTimer.active_task_name === task.title;
  $: isPlaying = isThisTaskActive && !$activeTimer?.paused;

  // ── Actions ───────────────────────────────────────────────────────────────

  async function toggleComplete() {
    const status = task.status === 'done' ? 'todo' : 'done';
    await invoke('update_task', { task: { ...task, status, tags: task.tags } });
    refreshTasks();
  }

  async function toggleTimer() {
    if (isThisTaskActive) {
      await invoke('pause_timer');
    } else {
      if (task.status === 'todo') {
        await invoke('update_task', { task: { ...task, status: 'doing', tags: task.tags } });
        refreshTasks();
      }
      await invoke('start_pomodoro', {
        taskName: task.title,
        durationMinutes: $timerDuration
      });
    }
  }

  async function handleSave(e: CustomEvent<{ task: Task }>) {
    await invoke('update_task', { task: e.detail.task });
    showEditModal = false;
    refreshTasks();
  }

  async function deleteTask() {
    if (deleting) return;
    deleting = true;
    await invoke('delete_task', { id: task.id });
    refreshTasks();
  }
</script>

<!-- Edit modal (conditionally rendered) -->
{#if showEditModal}
  <TaskEditModal
    {task}
    tags={$tags}
    projects={$projects}
    on:save={handleSave}
    on:close={() => (showEditModal = false)}
  />
{/if}

<!-- Task row -->
<div class="task-item" class:completed={task.status === 'done'} class:ongoing={isPlaying}>

  <!-- Checkbox (toggles done/todo) -->
  <TaskCheckbox status={task.status} onToggle={toggleComplete} />

  <!-- Main content -->
  <div class="content">
    <div class="title-row">
      <span class="title">{task.title}</span>

      <!-- Start timer (shown on hover or when playing) -->
      <button class="start-btn" class:is-playing={isPlaying} on:click={toggleTimer} title={isPlaying ? "Pause timer" : "Start timer"}>
        {#if isPlaying}
          <Pause size={13} fill="currentColor" />
        {:else}
          <Play size={13} fill="currentColor" />
        {/if}
      </button>
    </div>

    {#if task.description}
      <p class="description">{task.description}</p>
    {/if}

    <!-- Due date, tags, priority, source badges -->
    <TaskMeta
      dueDate={task.due_date}
      tags={task.tags}
      priority={task.priority}
      source={task.source}
      project={project}
    />
  </div>

  <!-- Row actions (shown on hover) -->
  <div class="actions">
    <button class="action-btn" on:click={() => (showEditModal = true)} title="Edit task">
      <Pencil size={15} />
    </button>
    <button class="action-btn danger" on:click={deleteTask} title="Delete task" disabled={deleting}>
      <Trash2 size={15} />
    </button>
  </div>

</div>

<style>
  .task-item {
    display: flex;
    align-items: flex-start;
    gap: 0.75rem;
    padding: 0.75rem;
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 8px;
    transition: var(--transition);
    user-select: none;
  }

  .task-item:hover {
    border-color: var(--accent-blue-border);
    transform: translateY(-1px);
    box-shadow: var(--shadow);
  }

  /* Strike-through and fade completed tasks */
  .task-item.completed { opacity: 0.6; }
  .task-item.completed .title {
    text-decoration: line-through;
    color: var(--text-muted);
  }

  /* Ongoing task highlighted state */
  .task-item.ongoing {
    border-color: var(--accent-blue);
    background: var(--accent-blue-hover);
    box-shadow: 0 0 0 1px var(--accent-blue);
  }

  /* ── Content area ── */
  .content {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    min-width: 0; /* allow text-overflow to work */
  }

  .title-row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .title {
    font-size: 0.9375rem;
    font-weight: 500;
    color: var(--text);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    flex: 1;
  }

  .description {
    margin: 0;
    font-size: 0.8125rem;
    color: var(--text-muted);
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  /* ── Hover-only elements ── */
  .start-btn,
  .actions {
    opacity: 0;
    transition: opacity 0.2s;
    flex-shrink: 0;
  }

  .task-item:hover .start-btn,
  .task-item:hover .actions,
  .start-btn.is-playing { opacity: 1; }

  .start-btn {
    background: none;
    border: none;
    color: var(--accent-blue);
    cursor: pointer;
    padding: 0.2rem;
    border-radius: 4px;
    display: flex;
    align-items: center;
  }

  .start-btn:hover { background: var(--accent-blue-hover); }

  /* ── Action buttons ── */
  .actions {
    display: flex;
    align-items: flex-start;
    gap: 0.125rem;
  }

  .action-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 0.25rem;
    border-radius: 4px;
    display: flex;
    align-items: center;
    transition: all 0.15s;
  }

  .action-btn:hover { background: var(--btn-secondary-hover-bg); color: var(--text); }
  .action-btn.danger:hover { background: var(--accent-red-hover); color: var(--error-red); }
  .action-btn:disabled { opacity: 0.4; cursor: not-allowed; }
</style>
