<script lang="ts">
  import { dndzone } from 'svelte-dnd-action';
  import { tasks, refreshTasks } from '$lib/stores/tasks';
  import TaskItem from './TaskItem.svelte';
  import { flip } from 'svelte/animate';
  import { invoke } from '@tauri-apps/api/core';

  const flipDurationMs = 300;

  function handleDndConsider(e: CustomEvent) {
    tasks.set(e.detail.items);
  }

  async function handleDndFinalize(e: CustomEvent) {
    tasks.set(e.detail.items);
    const items = e.detail.items;
    for (let i = 0; i < items.length; i++) {
      if (items[i].position !== i) {
        await invoke('update_task', { task: { ...items[i], position: i } });
      }
    }
    refreshTasks();
  }
</script>

<div class="task-list" 
     use:dndzone={{items: $tasks, flipDurationMs}} 
     on:consider={handleDndConsider} 
     on:finalize={handleDndFinalize}>
  {#each $tasks as task (task.id)}
    <div class="list-item" animate:flip={{duration: flipDurationMs}}>
      <TaskItem {task} />
    </div>
  {/each}
</div>

<style>
  .task-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    min-height: 100px;
    padding-bottom: 2rem;
  }

  .list-item {
    outline: none;
  }
</style>
