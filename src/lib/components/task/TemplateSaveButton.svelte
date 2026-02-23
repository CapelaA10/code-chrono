<!--
  TemplateSaveButton.svelte
  ─────────────────────────
  Icon button that prompts for a template name and emits "save".
  Events: save with { name: string }
-->
<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { BookmarkPlus } from 'lucide-svelte';
  import { strings } from '$lib/i18n/store';

  const dispatch = createEventDispatcher<{ save: { name: string } }>();

  let naming = false;
  let templateName = '';

  function confirm() {
    const n = templateName.trim();
    if (n) dispatch('save', { name: n });
    naming = false;
    templateName = '';
  }

  function handleKey(e: KeyboardEvent) {
    if (e.key === 'Enter') confirm();
    if (e.key === 'Escape') { naming = false; templateName = ''; }
  }
</script>

{#if naming}
  <div class="inline-form">
    <!-- svelte-ignore a11y_autofocus -->
    <input
      class="name-input"
      bind:value={templateName}
      placeholder={$strings.templateNameHint}
      maxlength="50"
      autofocus
      on:keydown={handleKey}
    />
    <button class="confirm-btn" on:click={confirm} disabled={!templateName.trim()}>{$strings.save}</button>
    <button class="cancel-btn" on:click={() => { naming = false; templateName = ''; }}>✕</button>
  </div>
{:else}
  <button class="icon-btn" on:click={() => naming = true} title={$strings.saveAsTemplate}>
    <BookmarkPlus size={16} />
  </button>
{/if}

<style>
  .icon-btn {
    background: none; border: none; color: var(--text-muted);
    cursor: pointer; padding: 4px; border-radius: 6px;
    display: flex; align-items: center; transition: color 0.15s;
  }
  .icon-btn:hover { color: var(--accent-blue); }

  .inline-form {
    display: flex; align-items: center; gap: 0.35rem;
  }

  .name-input {
    background: var(--input-bg); border: 1px solid var(--accent-blue);
    border-radius: 6px; padding: 0.25rem 0.5rem;
    color: var(--text); font-size: 0.8125rem; font-family: inherit;
    outline: none; width: 140px;
  }

  .confirm-btn {
    background: var(--accent-blue); color: white;
    border: none; border-radius: 6px;
    padding: 0.25rem 0.6rem; font-size: 0.75rem; font-weight: 600;
    cursor: pointer; font-family: inherit;
  }
  .confirm-btn:disabled { opacity: 0.5; cursor: not-allowed; }

  .cancel-btn {
    background: none; border: none; color: var(--text-muted);
    cursor: pointer; font-size: 0.75rem; padding: 0.25rem;
  }
  .cancel-btn:hover { color: var(--text); }
</style>
