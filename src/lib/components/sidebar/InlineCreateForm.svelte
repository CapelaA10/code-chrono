<script lang="ts">
  import { Check, X } from 'lucide-svelte';
  import { createEventDispatcher } from 'svelte';

  export let show = false;
  export let color = '#3b82f6';
  export let placeholder = 'Name';
  export let maxlength = 40;

  const dispatch = createEventDispatcher<{
    confirm: { name: string; color: string };
    cancel: void;
  }>();

  const colorPresets = [
    '#ef4444', '#f97316', '#f59e0b', '#10b981',
    '#3b82f6', '#8b5cf6', '#ec4899', '#64748b'
  ];

  let name = '';
  let inputEl: HTMLInputElement;

  $: if (show) {
    name = '';
    // Focus on next tick after mount
    setTimeout(() => inputEl?.focus(), 50);
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter' && name.trim()) {
      dispatch('confirm', { name: name.trim(), color });
    }
    if (e.key === 'Escape') {
      dispatch('cancel');
    }
  }

  function submit() {
    if (name.trim()) dispatch('confirm', { name: name.trim(), color });
  }

  function cancel() {
    dispatch('cancel');
  }
</script>

{#if show}
  <div class="inline-form">
    <div class="color-row" role="group" aria-label="Pick color">
      {#each colorPresets as c}
        <button
          class="color-dot"
          class:selected={color === c}
          style="background:{c}"
          on:click={() => color = c}
          title={c}
          aria-label={c}
          aria-pressed={color === c}
        ></button>
      {/each}
    </div>
    <div class="form-row">
      <input
        bind:this={inputEl}
        bind:value={name}
        type="text"
        {placeholder}
        class="inline-input"
        on:keydown={handleKeydown}
        {maxlength}
      />
      <button class="confirm-btn" on:click={submit} title="Create" disabled={!name.trim()}>
        <Check size={14} />
      </button>
      <button class="cancel-btn" on:click={cancel} title="Cancel">
        <X size={14} />
      </button>
    </div>
  </div>
{/if}

<style>
  .inline-form {
    margin: 0.25rem 0 0.5rem;
    padding: 0.5rem;
    background: var(--btn-secondary-hover-bg);
    border-radius: 8px;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .color-row {
    display: flex;
    gap: 0.35rem;
    flex-wrap: wrap;
  }

  .color-dot {
    width: 18px;
    height: 18px;
    border-radius: 50%;
    border: 2px solid transparent;
    cursor: pointer;
    padding: 0;
    transition: transform 0.15s, border-color 0.15s;
    flex-shrink: 0;
  }

  .color-dot:hover { transform: scale(1.2); }
  .color-dot.selected {
    border-color: var(--text);
    transform: scale(1.1);
  }

  .form-row {
    display: flex;
    gap: 0.25rem;
    align-items: center;
  }

  .inline-input {
    flex: 1;
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 0.35rem 0.5rem;
    color: var(--text);
    font-size: 0.8125rem;
    outline: none;
    font-family: inherit;
    min-width: 0;
  }

  .inline-input:focus { border-color: var(--accent-blue); }

  .confirm-btn, .cancel-btn {
    background: none;
    border: none;
    cursor: pointer;
    padding: 4px;
    border-radius: 4px;
    display: flex;
    align-items: center;
    transition: all 0.15s;
    flex-shrink: 0;
  }

  .confirm-btn { color: var(--accent-green); }
  .confirm-btn:hover:not(:disabled) { background: var(--accent-green-hover); }
  .confirm-btn:disabled { opacity: 0.4; cursor: not-allowed; }

  .cancel-btn { color: var(--text-muted); }
  .cancel-btn:hover { color: var(--text); background: var(--btn-secondary-hover-bg); }
</style>
