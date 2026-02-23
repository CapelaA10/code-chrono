<script lang="ts">
  import { Plus, Search, X, FileText } from 'lucide-svelte';
  import { invoke } from '@tauri-apps/api/core';
  import TemplatePickerModal from '$lib/components/task/TemplatePickerModal.svelte';
  import { refreshTasks, searchQuery, searchTasks, projects, tags } from '$lib/stores/tasks';
  import { filterProject } from '$lib/stores/tasks';
  import { strings } from '$lib/i18n/store';
  import { get } from 'svelte/store';
  import { onMount } from 'svelte';

  let inputRef: HTMLInputElement;
  let inputValue = '';
  let isSearchMode = false;
  let showSuccess = false;
  let showTemplates = false;

  // Detect if user is searching vs adding:
  // - If it starts with '/' or selection shifts to search mode explicitly, search
  // - 'Enter' = add task, '/' prefix = search
  $: isSearchMode = inputValue.startsWith('/');
  $: displayQuery = isSearchMode ? inputValue.slice(1) : inputValue;

  async function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') {
      clearInput();
      return;
    }
    if (e.key === 'Enter') {
      const val = inputValue.trim();
      if (!val) return;
      if (isSearchMode) {
        // trigger search
        const q = val.slice(1).trim();
        if (q) {
          $searchQuery = q;
          searchTasks(q);
        }
      } else {
        // create task
        await createTask(val);
        inputValue = '';
      }
    }
  }

  async function createTask(title: string) {
    // Use current project filter if one is selected
    const currentProject = get(filterProject);
    await invoke('create_task', {
      task: {
        id: 0,
        title,
        status: 'todo',
        priority: 0,
        project_id: currentProject ?? null,
        created_at: Math.floor(Date.now() / 1000),
        position: 0,
        tags: []
      }
    });
    showSuccess = true;
    setTimeout(() => showSuccess = false, 2000);
    refreshTasks();
  }

  async function createTaskFromTemplate(t: any) {
    await invoke('create_task', {
      task: {
        id: 0,
        title: t.title,
        description: t.description,
        status: 'todo',
        priority: t.priority,
        project_id: t.project_id,
        created_at: Math.floor(Date.now() / 1000),
        position: 0,
        tags: t.tagIds ? get(tags).filter(tag => t.tagIds.includes(tag.id)) : []
      }
    });
    showSuccess = true;
    setTimeout(() => showSuccess = false, 2000);
    refreshTasks();
  }

  function applyTemplate(e: CustomEvent<any>) {
    createTaskFromTemplate(e.detail);
    showTemplates = false;
  }

  function handleInput(e: Event) {
    const val = (e.target as HTMLInputElement).value;
    inputValue = val;

    if (isSearchMode) {
      const q = val.slice(1).trim();
      $searchQuery = q;
      if (q.length > 1) {
        searchTasks(q);
      } else if (q.length === 0) {
        refreshTasks();
      }
    } else {
      // Not in search mode — clear the search if we had one
      if ($searchQuery) {
        $searchQuery = '';
        refreshTasks();
      }
    }
  }

  function clearInput() {
    inputValue = '';
    $searchQuery = '';
    refreshTasks();
    inputRef?.blur();
  }

  onMount(() => {
    const handleGlobalKeydown = (e: KeyboardEvent) => {
      if (e.altKey && e.key === 'n') {
        inputRef.focus();
        e.preventDefault();
      }
    };
    window.addEventListener('keydown', handleGlobalKeydown);
    return () => window.removeEventListener('keydown', handleGlobalKeydown);
  });
</script>

<div class="quick-add">
  <div class="input-wrapper" class:search-mode={isSearchMode}>
    {#if isSearchMode}
      <Search size={18} class="prefix-icon search" />
    {:else}
      <Plus size={18} class="prefix-icon" />
    {/if}
    <input
      bind:this={inputRef}
      type="text"
      bind:value={inputValue}
      placeholder={isSearchMode ? $strings.searchTasks : $strings.addTask}
      on:keydown={handleKeydown}
      on:input={handleInput}
    />
    {#if inputValue}
      <button class="clear-btn" on:click={clearInput} title={$strings.clear}>
        <X size={16} />
      </button>
    {:else}
      <button class="clear-btn" on:click={() => showTemplates = true} title={$strings.templates}>
        <FileText size={16} />
      </button>
    {/if}
    {#if showSuccess}
      <div class="success-indicator">✓</div>
    {/if}
  </div>
  {#if isSearchMode}
    <div class="search-hint">{$strings.searchHint}</div>
  {/if}
</div>

{#if showTemplates}
  <TemplatePickerModal
    on:selected={applyTemplate}
    on:close={() => showTemplates = false}
  />
{/if}

<style>
  .quick-add {
    position: relative;
    width: 100%;
  }

  .input-wrapper {
    position: relative;
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0 0.75rem;
    background: var(--input-bg);
    border: 1px solid var(--border);
    border-radius: 12px;
    transition: var(--transition);
  }

  .input-wrapper:focus-within {
    border-color: var(--accent-blue);
    box-shadow: 0 0 0 4px var(--focus-ring);
  }

  .input-wrapper.search-mode {
    border-color: var(--accent-green-border);
  }

  .input-wrapper.search-mode:focus-within {
    border-color: var(--accent-green);
    box-shadow: 0 0 0 4px var(--accent-green-hover);
  }

  :global(.prefix-icon) {
    color: var(--text-muted);
    flex-shrink: 0;
  }

  :global(.prefix-icon.search) {
    color: var(--accent-green);
  }

  input {
    flex: 1;
    background: none;
    border: none;
    padding: 0.75rem 0;
    color: var(--text);
    font-size: 0.9375rem;
    outline: none;
    font-family: inherit;
    min-width: 0;
  }

  input::placeholder {
    color: var(--input-placeholder);
    font-size: 0.875rem;
  }

  .clear-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 0.25rem;
    display: flex;
    align-items: center;
    transition: color 0.15s;
    flex-shrink: 0;
  }

  .clear-btn:hover {
    color: var(--text);
  }

  .success-indicator {
    color: var(--accent-green);
    font-size: 0.875rem;
    font-weight: 700;
    flex-shrink: 0;
    animation: bounceIn 0.3s cubic-bezier(0.175, 0.885, 0.32, 1.275);
  }

  .search-hint {
    font-size: 0.6875rem;
    color: var(--accent-green);
    padding: 0.25rem 0.75rem 0;
    opacity: 0.8;
  }

  @keyframes bounceIn {
    0% { transform: scale(0); opacity: 0; }
    50% { transform: scale(1.5); }
    100% { transform: scale(1); opacity: 1; }
  }
</style>
