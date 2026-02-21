<script lang="ts">
  import { theme } from '$lib/stores/theme';
  import { Sun, Moon } from 'lucide-svelte';
  import { fade } from 'svelte/transition';

  function toggle() {
    theme.toggle();
  }
</script>

<button class="theme-toggle" on:click={toggle} aria-label="Toggle theme">
  {#if $theme === 'dark'}
    <div in:fade={{ duration: 200 }}>
      <Sun size={20} />
    </div>
  {:else}
    <div in:fade={{ duration: 200 }}>
      <Moon size={20} />
    </div>
  {/if}
</button>

<style>
  .theme-toggle {
    background: var(--bg-card);
    border: 1px solid var(--border);
    color: var(--text);
    width: 40px;
    height: 40px;
    border-radius: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.2s;
    box-shadow: var(--shadow-sm);
  }

  .theme-toggle:hover {
    border-color: var(--accent-blue);
    transform: translateY(-1px);
    box-shadow: var(--shadow);
  }

  .theme-toggle:active {
    transform: translateY(0);
  }
</style>
