<!--
  TemplatePickerModal.svelte
  ──────────────────────────
  Overlay listing saved templates. Emits:
    - "selected" with the chosen TaskTemplate
    - "close"
-->
<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { X, Trash2 } from 'lucide-svelte';
  import { templates, deleteTemplate } from '$lib/stores/templates';
  import type { TaskTemplate } from '$lib/stores/templates';
  import { strings } from '$lib/i18n/store';

  const dispatch = createEventDispatcher<{
    selected: TaskTemplate;
    close: void;
  }>();
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<div
  class="overlay"
  role="dialog"
  aria-modal="true"
  aria-label="Choose template"
  tabindex="-1"
  on:click|self={() => dispatch('close')}
  on:keydown={(e) => e.key === 'Escape' && dispatch('close')}
>
  <div class="modal">
    <div class="modal-header">
      <h2>{$strings.templates}</h2>
      <button class="icon-btn" on:click={() => dispatch('close')} title={$strings.cancel}>
        <X size={18} />
      </button>
    </div>

    {#if $templates.length === 0}
      <p class="empty">{$strings.noTemplates}</p>
    {:else}
      <ul class="template-list">
        {#each $templates as t (t.id)}
          <li class="template-row">
            <button class="template-name" on:click={() => dispatch('selected', t)}>
              {t.name}
            </button>
            <button class="delete-btn" on:click={() => deleteTemplate(t.id)} title="Delete">
              <Trash2 size={14} />
            </button>
          </li>
        {/each}
      </ul>
    {/if}
  </div>
</div>

<style>
  .overlay {
    position: fixed; inset: 0;
    background: rgba(0,0,0,0.45);
    backdrop-filter: blur(4px);
    z-index: 110;
    display: flex; align-items: center; justify-content: center;
    padding: 1rem;
  }

  .modal {
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 16px;
    padding: 1.5rem;
    width: 100%; max-width: 400px;
    box-shadow: var(--shadow-lg);
    display: flex; flex-direction: column; gap: 1rem;
  }

  .modal-header {
    display: flex; justify-content: space-between; align-items: center;
  }

  .modal-header h2 { margin: 0; font-size: 1rem; font-weight: 700; }

  .icon-btn {
    background: none; border: none; color: var(--text-muted);
    cursor: pointer; padding: 4px; border-radius: 6px;
    display: flex; align-items: center; transition: color 0.15s;
  }
  .icon-btn:hover { color: var(--text); }

  .empty { color: var(--text-muted); font-size: 0.875rem; margin: 0; text-align: center; }

  .template-list { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; gap: 4px; }

  .template-row {
    display: flex; align-items: center; gap: 0.5rem;
    border-radius: 8px; padding: 0.25rem 0.5rem;
    transition: background 0.15s;
  }
  .template-row:hover { background: var(--btn-secondary-hover-bg); }

  .template-name {
    flex: 1; text-align: left; background: none; border: none;
    font-size: 0.875rem; font-weight: 500; color: var(--text);
    cursor: pointer; font-family: inherit; padding: 4px;
  }

  .delete-btn {
    opacity: 0; background: none; border: none;
    color: var(--text-muted); cursor: pointer; padding: 4px;
    border-radius: 4px; display: flex; align-items: center;
    transition: opacity 0.15s, color 0.15s;
  }
  .template-row:hover .delete-btn { opacity: 1; }
  .delete-btn:hover { color: var(--error-red); }
</style>
