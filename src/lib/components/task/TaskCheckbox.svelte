<script lang="ts">
  import { CheckCircle2, Circle } from 'lucide-svelte';

  export let status: 'todo' | 'doing' | 'done';
  export let onToggle: () => void;
</script>

<button class="checkbox" on:click={onToggle}>
  {#if status === 'done'}
    <CheckCircle2 size={20} class="completed-icon" />
  {:else}
    <Circle size={20} />
  {/if}
</button>

<style>
  .checkbox {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 0;
    display: flex;
    align-items: flex-start;
    margin-top: 2px;
  }

  .checkbox:hover {
    color: var(--accent-blue);
  }

  :global(.completed-icon) {
    color: var(--accent-green, #10b981) !important;
  }
</style>
