<!--
  Header.svelte
  ─────────────
  Application top bar. Composes three focused sub-components:
    - QuickAdd     (task creation / search input)
    - TimerWidget  (Pomodoro timer, self-managed)
    - ThemeToggle  (light/dark switch)
-->
<script lang="ts">
  import QuickAdd     from './QuickAdd.svelte';
  import TimerWidget  from './timer/TimerWidget.svelte';
  import ThemeToggle  from './ThemeToggle.svelte';
</script>

<header class="app-header">
  <div class="search-area">
    <QuickAdd />
  </div>

  <TimerWidget />

  <div class="actions-area">
    <ThemeToggle />
  </div>
</header>

<style>
  .app-header {
    display: flex;
    align-items: center;
    gap: 1.5rem;
    padding: 1rem 2rem;
    background: var(--bg-card);
    border-bottom: 1px solid var(--border);
    height: 72px;
    box-sizing: border-box;
    flex-shrink: 0;
  }

  .search-area {
    flex: 1;
    max-width: 580px;
    min-width: 0;
  }

  .actions-area {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    flex-shrink: 0;
  }
</style>
