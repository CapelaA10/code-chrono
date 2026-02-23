<script lang="ts">
  import { onMount } from 'svelte';
  import { check } from '@tauri-apps/plugin-updater';
  import { Download, RefreshCw } from 'lucide-svelte';

  let status: 'idle' | 'downloading' | 'ready' = 'idle';
  let updateVersion = '';

  onMount(async () => {
    // Only run this logic if we are inside a Tauri window instance
    if (!(window as any).__TAURI_INTERNALS__) return;

    try {
      console.log('Checking for updates...');
      const update = await check();

      if (update) {
        console.log(`Update ${update.version} available. Downloading silently...`);
        updateVersion = update.version;
        status = 'downloading';

        let downloaded = 0;
        let contentLength = 0;

        // Perform the download and install headlessly
        await update.downloadAndInstall((progress) => {
          switch (progress.event) {
            case 'Started':
              contentLength = progress.data.contentLength || 0;
              break;
            case 'Progress':
              downloaded += progress.data.chunkLength;
              break;
            case 'Finished':
              console.log('Update downloaded and installed successfully');
              break;
          }
        });

        status = 'ready';
      } else {
        console.log('No update available.');
      }
    } catch (e) {
      console.error('Update check failed:', e);
      status = 'idle'; // Reset on error so we don't get stuck
    }
  });
</script>

{#if status === 'downloading'}
  <div class="update-toast">
    <RefreshCw size={16} class="spin" />
    <span class="text">Downloading update <strong>v{updateVersion}</strong>...</span>
  </div>
{:else if status === 'ready'}
  <div class="update-toast ready">
    <Download size={16} />
    <span class="text">Update ready! Restart app to apply.</span>
  </div>
{/if}

<style>
  .update-toast {
    position: fixed;
    bottom: 24px;
    right: 24px;
    background: var(--bg-card);
    border: 1px solid var(--border);
    padding: 12px 18px;
    border-radius: 99px; /* pill styling */
    display: flex;
    align-items: center;
    gap: 12px;
    font-size: 0.875rem;
    color: var(--text);
    box-shadow: 0 4px 14px rgba(0, 0, 0, 0.15);
    z-index: 9999;
    animation: slideUp 0.4s cubic-bezier(0.16, 1, 0.3, 1);
  }
  
  .update-toast.ready {
    border-color: var(--green);
    color: var(--green);
    background: color-mix(in srgb, var(--green) 10%, var(--bg-card));
  }

  .text {
    font-weight: 500;
  }

  :global(.spin) {
    animation: spin 2s linear infinite;
    color: var(--primary);
  }

  @keyframes slideUp {
    from { opacity: 0; transform: translateY(20px) scale(0.95); }
    to { opacity: 1; transform: translateY(0) scale(1); }
  }
  
  @keyframes spin {
    100% { transform: rotate(360deg); }
  }
</style>
