<!--
  TaskEditModal.svelte
  ───────────────────
  Full-screen overlay for editing a task's fields.
  Emits:
    - "save"   with { task: Task }  when the user clicks Save / presses Ctrl+Enter
    - "close"              when the user cancels or clicks the backdrop
-->
<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { X, Check } from 'lucide-svelte';
  import type { Task, Tag, Project } from '$lib/types';

  export let task: Task;
  export let tags: Tag[];
  export let projects: Project[];

  const dispatch = createEventDispatcher<{
    save: { task: Task };
    close: void;
  }>();

  // ── Local edit state (initialised from the task prop) ─────────────────────
  let title        = task.title;
  let description  = task.description ?? '';
  let priority     = task.priority;
  let projectId: number | null = task.project_id ?? null;
  let dueDate      = task.due_date
    ? new Date(task.due_date * 1000).toISOString().split('T')[0]
    : '';
  let selectedTagIds: number[] = task.tags.map(t => t.id);

  let titleInput: HTMLInputElement;

  // Focus the title field as soon as the modal mounts
  $: if (titleInput) setTimeout(() => titleInput.focus(), 50);

  const PRIORITY_OPTIONS = ['None', 'Low', 'Medium', 'High'] as const;

  // ── Event handlers ─────────────────────────────────────────────────────────

  function toggleTag(id: number) {
    selectedTagIds = selectedTagIds.includes(id)
      ? selectedTagIds.filter(t => t !== id)
      : [...selectedTagIds, id];
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape')              dispatch('close');
    if (e.key === 'Enter' && e.ctrlKey)  submit();
  }

  function submit() {
    if (!title.trim()) return;

    const due_date = dueDate
      ? Math.floor(new Date(dueDate).getTime() / 1000)
      : null;

    const chosenTags = tags.filter(t => selectedTagIds.includes(t.id));

    dispatch('save', {
      task: {
        ...task,
        title:       title.trim(),
        description: description.trim() || null,
        priority,
        project_id:  projectId,
        due_date,
        tags:        chosenTags
      }
    });
  }
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<div
  class="overlay"
  role="dialog"
  aria-modal="true"
  aria-label="Edit task"
  tabindex="-1"
  on:click|self={() => dispatch('close')}
  on:keydown={(e) => e.key === 'Escape' && dispatch('close')}
>
  <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
  <div class="modal" role="document" on:keydown={handleKeydown}>

    <!-- Header -->
    <div class="modal-header">
      <h2>Edit Task</h2>
      <button class="icon-btn" on:click={() => dispatch('close')} title="Close">
        <X size={18} />
      </button>
    </div>

    <!-- Title -->
    <div class="field">
      <label class="field-label" for="edit-title">Title <span class="required">*</span></label>
      <input
        id="edit-title"
        bind:this={titleInput}
        bind:value={title}
        type="text"
        class="input"
        placeholder="Task title"
        maxlength="100"
      />
    </div>

    <!-- Description -->
    <div class="field">
      <label class="field-label" for="edit-desc">Description</label>
      <textarea
        id="edit-desc"
        bind:value={description}
        class="input textarea"
        placeholder="Optional notes…"
        rows="2"
      ></textarea>
    </div>

    <!-- Priority · Project · Due Date (3-column row) -->
    <div class="field-row">
      <div class="field">
        <label class="field-label" for="edit-priority">Priority</label>
        <select id="edit-priority" bind:value={priority} class="input">
          {#each PRIORITY_OPTIONS as label, i}
            <option value={i}>{label}</option>
          {/each}
        </select>
      </div>

      <div class="field">
        <label class="field-label" for="edit-project">Project</label>
        <select id="edit-project" bind:value={projectId} class="input">
          <option value={null}>None</option>
          {#each projects as p (p.id)}
            <option value={p.id}>{p.name}</option>
          {/each}
        </select>
      </div>

      <div class="field">
        <label class="field-label" for="edit-due">Due Date</label>
        <input id="edit-due" bind:value={dueDate} type="date" class="input" />
      </div>
    </div>

    <!-- Tags (fieldset for a11y) -->
    {#if tags.length > 0}
      <fieldset class="field tags-fieldset">
        <legend class="field-label">Tags</legend>
        <div class="tag-picker">
          {#each tags as tag (tag.id)}
            <button
              class="tag-btn"
              class:selected={selectedTagIds.includes(tag.id)}
              style="--tc: {tag.color ?? '#64748b'}"
              on:click={() => toggleTag(tag.id)}
            >
              {tag.name}
            </button>
          {/each}
        </div>
      </fieldset>
    {/if}

    <!-- Footer -->
    <div class="modal-footer">
      <span class="hint">Ctrl+Enter to save · Esc to cancel</span>
      <div class="footer-actions">
        <button class="btn-secondary" on:click={() => dispatch('close')}>Cancel</button>
        <button class="btn-primary" on:click={submit} disabled={!title.trim()}>
          <Check size={16} />
          Save
        </button>
      </div>
    </div>

  </div>
</div>

<style>
  /* ── Overlay ── */
  .overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.5);
    backdrop-filter: blur(4px);
    z-index: 100;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 1rem;
  }

  /* ── Modal shell ── */
  .modal {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 16px;
    padding: 1.5rem;
    width: 100%;
    max-width: 520px;
    box-shadow: var(--shadow-lg);
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  /* ── Modal header ── */
  .modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .modal-header h2 {
    margin: 0;
    font-size: 1.0625rem;
    font-weight: 700;
    color: var(--text);
  }

  .icon-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 4px;
    border-radius: 6px;
    display: flex;
    align-items: center;
    transition: color 0.15s;
  }

  .icon-btn:hover { color: var(--text); }

  /* ── Fields ── */
  .field {
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
  }

  .field-label {
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .required { color: var(--error-red); }

  .field-row {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 0.75rem;
  }

  /* ── Input reset ── */
  .input {
    background: var(--input-bg);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 0.5rem 0.625rem;
    color: var(--text);
    font-size: 0.875rem;
    font-family: inherit;
    outline: none;
    width: 100%;
    box-sizing: border-box;
    transition: border-color 0.15s;
  }

  .input:focus { border-color: var(--accent-blue); }

  .textarea {
    resize: vertical;
    min-height: 60px;
  }

  /* ── Tags ── */
  .tags-fieldset {
    border: none;
    padding: 0;
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
  }

  .tags-fieldset legend { float: left; width: 100%; margin-bottom: 0.35rem; }

  .tag-picker {
    display: flex;
    flex-wrap: wrap;
    gap: 0.35rem;
    clear: left;
  }

  .tag-btn {
    padding: 0.25rem 0.625rem;
    border-radius: 6px;
    border: 1px solid var(--border);
    background: var(--bg-page);
    color: var(--text-muted);
    font-size: 0.75rem;
    cursor: pointer;
    font-family: inherit;
    transition: all 0.15s;
  }

  .tag-btn:hover {
    border-color: var(--tc);
    color: var(--tc);
  }

  .tag-btn.selected {
    background: color-mix(in srgb, var(--tc) 15%, transparent);
    border-color: var(--tc);
    color: var(--tc);
    font-weight: 600;
  }

  /* ── Footer ── */
  .modal-footer {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 0.5rem;
    padding-top: 0.5rem;
    border-top: 1px solid var(--border);
  }

  .hint {
    font-size: 0.7rem;
    color: var(--text-muted);
    opacity: 0.7;
  }

  .footer-actions { display: flex; gap: 0.5rem; }

  .btn-secondary {
    padding: 0.5rem 1rem;
    border: 1px solid var(--border);
    border-radius: 8px;
    background: none;
    color: var(--text-muted);
    font-size: 0.875rem;
    font-weight: 500;
    cursor: pointer;
    font-family: inherit;
    transition: all 0.15s;
  }

  .btn-secondary:hover {
    background: var(--btn-secondary-hover-bg);
    color: var(--text);
  }

  .btn-primary {
    padding: 0.5rem 1rem;
    border: none;
    border-radius: 8px;
    background: var(--accent-blue);
    color: white;
    font-size: 0.875rem;
    font-weight: 600;
    cursor: pointer;
    font-family: inherit;
    display: flex;
    align-items: center;
    gap: 0.35rem;
    transition: background 0.15s;
  }

  .btn-primary:hover:not(:disabled) { background: #2563eb; }
  .btn-primary:disabled { opacity: 0.5; cursor: not-allowed; }
</style>
