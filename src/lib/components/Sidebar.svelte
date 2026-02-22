<script lang="ts">
  import SidebarLogo from './sidebar/SidebarLogo.svelte';
  import SidebarNav from './sidebar/SidebarNav.svelte';
  import SidebarIntegrations from './sidebar/SidebarIntegrations.svelte';
  import AboutDialog from './AboutDialog.svelte';
  import { Settings, BarChart2, Info } from 'lucide-svelte';

  let showAbout = false;
</script>

<aside class="sidebar">
  <SidebarLogo />
  <div class="sidebar-scroll">
    <SidebarNav />
    <SidebarIntegrations />
  </div>
  <div class="sidebar-footer">
    <a href="/stats" class="footer-item">
      <BarChart2 size={18} />
      <span>Statistics</span>
    </a>
    <a href="/settings" class="footer-item">
      <Settings size={18} />
      <span>Settings</span>
    </a>
    <button class="footer-item about-btn" on:click={() => showAbout = true}>
      <Info size={18} />
      <span>About</span>
    </button>
  </div>
</aside>

{#if showAbout}
  <AboutDialog on:close={() => showAbout = false} />
{/if}

<style>
  .sidebar {
    width: 280px;
    height: 100vh;
    background: var(--bg-card);
    border-right: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    padding: 1.5rem 0;
    flex-shrink: 0;
  }

  .sidebar-scroll {
    flex: 1;
    overflow-y: auto;
  }

  .sidebar-scroll::-webkit-scrollbar {
    width: 4px;
  }

  .sidebar-scroll::-webkit-scrollbar-thumb {
    background: transparent;
    border-radius: 10px;
  }

  .sidebar:hover .sidebar-scroll::-webkit-scrollbar-thumb {
    background: var(--border);
  }

  .sidebar-footer {
    padding: 1rem 0.75rem 0;
    border-top: 1px solid var(--border);
    margin-top: 0.5rem;
  }

  .footer-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.625rem 0.75rem;
    color: var(--text-muted);
    text-decoration: none;
    font-size: 0.875rem;
    border-radius: 8px;
    transition: all 0.15s;
  }

  .footer-item:hover {
    background: var(--btn-secondary-hover-bg);
    color: var(--text);
  }

  /* Reset button to look like the link items */
  .about-btn {
    width: 100%;
    border: none;
    background: none;
    cursor: pointer;
    font-family: inherit;
    text-align: left;
  }
</style>
