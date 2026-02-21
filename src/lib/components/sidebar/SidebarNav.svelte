<script lang="ts">
  import { onMount } from 'svelte';
  import { projects, tags, filterProject, filterTag, refreshProjects, refreshTags, refreshTasks } from '$lib/stores/tasks';
  import { Plus, Hash, Folder, LayoutGrid, Trash2 } from 'lucide-svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import InlineCreateForm from './InlineCreateForm.svelte';

  let showProjectForm = false;
  let showTagForm = false;
  let newProjectColor = '#3b82f6';
  let newTagColor = '#10b981';

  onMount(() => {
    refreshProjects();
    refreshTags();
  });

  async function handleAddProject(e: CustomEvent<{ name: string; color: string }>) {
    await invoke('create_project', { name: e.detail.name, color: e.detail.color });
    showProjectForm = false;
    refreshProjects();
    navHome();
  }

  async function handleAddTag(e: CustomEvent<{ name: string; color: string }>) {
    await invoke('create_tag', { name: e.detail.name, color: e.detail.color });
    showTagForm = false;
    refreshTags();
    navHome();
  }

  async function deleteProject(id: number) {
    await invoke('delete_project', { id });
    if ($filterProject === id) $filterProject = null;
    refreshProjects();
    refreshTasks();
  }

  async function deleteTag(id: number) {
    await invoke('delete_tag', { id });
    if ($filterTag === id) $filterTag = null;
    refreshTags();
    refreshTasks();
  }

  function selectProject(id: number) {
    $filterProject = $filterProject === id ? null : id;
    $filterTag = null;
    refreshTasks();
  }

  function selectTag(id: number) {
    $filterTag = $filterTag === id ? null : id;
    $filterProject = null;
    refreshTasks();
  }

  function openProjectForm() {
    showProjectForm = true;
    showTagForm = false;
  }

  function openTagForm() {
    showTagForm = true;
    showProjectForm = false;
  }

  function navHome() {
    if ($page.url.pathname !== '/') {
      goto('/');
    }
  }

  function selectAll() {
    $filterProject = null;
    $filterTag = null;
    refreshTasks();
    navHome();
  }

  function selectProjectItem(id: number) {
    selectProject(id);
    navHome();
  }

  function selectTagItem(id: number) {
    selectTag(id);
    navHome();
  }
</script>

<!-- All Tasks -->
<nav class="nav-section">
  <button
    class="nav-item"
    class:active={!$filterProject && !$filterTag}
    on:click={selectAll}
  >
    <LayoutGrid size={18} />
    <span>All Tasks</span>
  </button>
</nav>

<!-- Projects -->
<div class="nav-section">
  <div class="section-header">
    <h3>Projects</h3>
    <button class="add-btn" on:click={openProjectForm} title="Add project">
      <Plus size={16} />
    </button>
  </div>

  <InlineCreateForm
    show={showProjectForm}
    bind:color={newProjectColor}
    placeholder="Project name"
    maxlength={40}
    on:confirm={handleAddProject}
    on:cancel={() => { showProjectForm = false; }}
  />

  <div class="section-list">
    {#each $projects as project (project.id)}
      <div class="item-wrapper" class:active={$filterProject === project.id}>
        <button class="nav-item flex-1" on:click={() => selectProjectItem(project.id)}>
          <Folder size={18} color={project.color ?? undefined} />
          <span>{project.name}</span>
        </button>
        <button class="delete-btn" on:click={() => deleteProject(project.id)} title="Delete project">
          <Trash2 size={14} />
        </button>
      </div>
    {/each}
  </div>
</div>

<!-- Tags -->
<div class="nav-section">
  <div class="section-header">
    <h3>Tags</h3>
    <button class="add-btn" on:click={openTagForm} title="Add tag">
      <Plus size={16} />
    </button>
  </div>

  <InlineCreateForm
    show={showTagForm}
    bind:color={newTagColor}
    placeholder="Tag name"
    maxlength={30}
    on:confirm={handleAddTag}
    on:cancel={() => { showTagForm = false; }}
  />

  <div class="section-list">
    {#each $tags as tag (tag.id)}
      <div class="item-wrapper" class:active={$filterTag === tag.id}>
        <button class="nav-item flex-1" on:click={() => selectTagItem(tag.id)}>
          <Hash size={18} color={tag.color ?? undefined} />
          <span>{tag.name}</span>
        </button>
        <button class="delete-btn" on:click={() => deleteTag(tag.id)} title="Delete tag">
          <Trash2 size={14} />
        </button>
      </div>
    {/each}
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

  .add-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 4px;
    border-radius: 4px;
    display: flex;
    align-items: center;
    transition: all 0.15s;
  }

  .add-btn:hover {
    background: var(--btn-secondary-hover-bg);
    color: var(--text);
  }

  .section-list {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .item-wrapper {
    display: flex;
    align-items: center;
    border-radius: 8px;
    transition: background 0.15s;
  }

  .item-wrapper:hover { background: var(--btn-secondary-hover-bg); }

  .item-wrapper.active { background: var(--accent-blue-hover); }

  .item-wrapper.active .nav-item {
    color: var(--accent-blue);
    font-weight: 600;
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
    font-family: inherit;
  }

  .flex-1 { flex: 1; min-width: 0; }

  .nav-item span {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .delete-btn {
    opacity: 0;
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 0.5rem;
    transition: opacity 0.2s, color 0.2s;
    flex-shrink: 0;
  }

  .item-wrapper:hover .delete-btn { opacity: 1; }
  .delete-btn:hover { color: var(--error-red); }
</style>
