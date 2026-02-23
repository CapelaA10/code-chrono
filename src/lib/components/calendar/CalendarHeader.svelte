<!--
  CalendarHeader.svelte
  ─────────────────────
  Month title and prev/next navigation.
  Events: prev, next
  Props: year, month (0-indexed)
-->
<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { ChevronLeft, ChevronRight } from 'lucide-svelte';
  import { locale } from '$lib/i18n/store';

  export let year:  number;
  export let month: number;

  const dispatch = createEventDispatcher<{ prev: void; next: void }>();

  $: label = new Date(year, month, 1).toLocaleDateString($locale, {
    month: 'long',
    year:  'numeric'
  });
</script>

<div class="cal-header">
  <button class="nav-btn" on:click={() => dispatch('prev')} title="Previous month">
    <ChevronLeft size={18} />
  </button>
  <span class="month-label">{label}</span>
  <button class="nav-btn" on:click={() => dispatch('next')} title="Next month">
    <ChevronRight size={18} />
  </button>
</div>

<style>
  .cal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 1rem;
  }

  .month-label {
    font-size: 1.125rem;
    font-weight: 700;
    color: var(--text);
  }

  .nav-btn {
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 6px;
    border-radius: 8px;
    display: flex;
    align-items: center;
    transition: color 0.15s, background 0.15s;
  }

  .nav-btn:hover {
    background: var(--btn-secondary-hover-bg);
    color: var(--text);
  }
</style>
