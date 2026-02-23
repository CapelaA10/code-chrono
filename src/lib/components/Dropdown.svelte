<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { ChevronDown, Check } from 'lucide-svelte';

  export let value: any;
  export let options: { value: any; label: string }[] = [];
  export let placeholder: string = "Select option";
  export let disabled = false;
  export let id: string = "";

  const dispatch = createEventDispatcher();
  let open = false;
  let popupRef: HTMLElement;

  $: selectedLabel = options.find(o => o.value === value)?.label ?? placeholder;

  function toggle() {
    if (disabled) return;
    open = !open;
  }
  
  function selectOption(val: any) {
    value = val;
    open = false;
    dispatch('change', value);
  }

  function closePopup(e: MouseEvent) {
    if (open && popupRef && !popupRef.contains(e.target as Node)) {
      open = false;
    }
  }
</script>

<svelte:window on:click={closePopup} />

<div class="dropdown-wrapper" bind:this={popupRef}>
  <button {id} type="button" class="dropdown-trigger" on:click|stopPropagation={toggle} class:disabled>
    <span class="value" class:placeholder={value === null || value === undefined || value === ''}>{selectedLabel}</span>
    <ChevronDown size={14} class="icon" />
  </button>

  {#if open}
    <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
    <div class="dropdown-menu" on:click|stopPropagation>
      {#each options as opt}
        <button type="button" class="dropdown-item" class:selected={value === opt.value} on:click={() => selectOption(opt.value)}>
          <span class="label">{opt.label}</span>
          {#if value === opt.value}
            <Check size={14} class="check-icon" />
          {/if}
        </button>
      {/each}
    </div>
  {/if}
</div>

<style>
  .dropdown-wrapper {
    position: relative;
    width: 100%;
  }

  .dropdown-trigger {
    width: 100%;
    background: var(--input-bg, var(--bg-card));
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 0.5rem 0.625rem;
    color: var(--text);
    font-size: 0.875rem;
    font-family: inherit;
    display: flex;
    justify-content: space-between;
    align-items: center;
    cursor: pointer;
    transition: border-color 0.15s, background-color 0.15s;
    user-select: none;
    text-align: left;
  }

  .dropdown-trigger:hover:not(.disabled) {
    border-color: var(--accent-blue);
  }

  .dropdown-trigger:focus-visible {
    outline: none;
    border-color: var(--accent-blue);
    box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.2);
  }

  .dropdown-trigger.disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .value {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .value.placeholder {
    color: var(--text-muted);
  }

  :global(.icon) {
    color: var(--text-muted);
    flex-shrink: 0;
    margin-left: 0.5rem;
  }

  .dropdown-menu {
    position: absolute;
    top: calc(100% + 4px);
    left: 0;
    width: 100%;
    z-index: 200;
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 0.35rem 0;
    box-shadow: 0 10px 25px -5px rgba(0, 0, 0, 0.4), 0 0 0 1px var(--border);
    max-height: 250px;
    overflow-y: auto;
  }

  .dropdown-item {
    width: 100%;
    display: flex;
    justify-content: space-between;
    align-items: center;
    background: transparent;
    border: none;
    padding: 0.5rem 0.75rem;
    color: var(--text);
    font-size: 0.875rem;
    font-family: inherit;
    cursor: pointer;
    text-align: left;
    transition: background 0.15s;
  }

  .dropdown-item:hover {
    background: var(--bg-page);
  }

  .dropdown-item.selected {
    background: var(--accent-blue-hover, rgba(59, 130, 246, 0.1));
    color: var(--accent-blue);
    font-weight: 500;
  }

  :global(.check-icon) {
    color: var(--accent-blue);
    flex-shrink: 0;
  }
</style>
