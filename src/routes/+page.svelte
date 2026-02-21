<script lang="ts">
  import Header from '$lib/components/Header.svelte';
  import TaskList from '$lib/components/TaskList.svelte';
  import { onMount } from 'svelte';
  import { refreshAll, filterProject, filterTag, filterStatus, projects, tags } from '$lib/stores/tasks';
  import { invoke } from '@tauri-apps/api/core';

  onMount(() => {
    // Load everything on startup
    refreshAll();

    const handleActivity = () => {
      invoke('record_activity').catch(() => {});
    };

    // Throttle activity recording to avoid hammering the backend
    let activityTimer: ReturnType<typeof setTimeout> | null = null;
    const throttledActivity = () => {
      if (!activityTimer) {
        handleActivity();
        activityTimer = setTimeout(() => { activityTimer = null; }, 5000);
      }
    };

    window.addEventListener('mousemove', throttledActivity);
    window.addEventListener('keydown', throttledActivity);
    window.addEventListener('mousedown', throttledActivity);

    return () => {
      window.removeEventListener('mousemove', throttledActivity);
      window.removeEventListener('keydown', throttledActivity);
      window.removeEventListener('mousedown', throttledActivity);
      if (activityTimer) clearTimeout(activityTimer);
    };
  });

  $: activeProject = $filterProject ? $projects.find(p => p.id === $filterProject) : null;
  $: activeTag = $filterTag ? $tags.find(t => t.id === $filterTag) : null;

  $: viewTitle = activeProject
    ? activeProject.name
    : activeTag
    ? `#${activeTag.name}`
    : 'All Tasks';

  $: viewSubtitle = activeProject
    ? 'Project'
    : activeTag
    ? 'Tag'
    : null;
</script>

<Header />

<div class="scroll-container">
    <div class="view-content">
      <div class="view-header">
        <div class="view-title-group">
          <h1>
            {#if activeProject}
              <span class="color-dot" style="background: {activeProject.color ?? '#3b82f6'}"></span>
            {/if}
            {viewTitle}
          </h1>
          {#if viewSubtitle}
            <span class="view-subtitle">{viewSubtitle}</span>
          {/if}
        </div>
      </div>
      
      <TaskList />
    </div>
  </div>

<style>
  .scroll-container {
    flex: 1;
    overflow-y: auto;
    padding: 2rem;
  }

  .view-content {
    max-width: 900px;
    margin: 0 auto;
    width: 100%;
  }

  .view-header {
    margin-bottom: 1.5rem;
  }

  .view-title-group {
    display: flex;
    align-items: baseline;
    gap: 0.75rem;
  }

  .view-header h1 {
    font-size: 1.75rem;
    font-weight: 800;
    color: var(--text);
    margin: 0;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .color-dot {
    width: 14px;
    height: 14px;
    border-radius: 50%;
    display: inline-block;
    flex-shrink: 0;
  }

  .view-subtitle {
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--text-muted);
  }

  .scroll-container::-webkit-scrollbar {
    width: 8px;
  }

  .scroll-container::-webkit-scrollbar-track {
    background: transparent;
  }

  .scroll-container::-webkit-scrollbar-thumb {
    background: var(--border);
    border-radius: 4px;
  }

  .scroll-container::-webkit-scrollbar-thumb:hover {
    background: var(--text-muted);
  }
</style>
