<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount, createEventDispatcher } from 'svelte';
  import { Github, Gitlab, CloudIcon, Check } from 'lucide-svelte';
  import { strings } from '$lib/i18n/store';

  const dispatch = createEventDispatcher<{ message: { text: string; type: 'success' | 'error' } }>();

  let githubToken = '';
  let githubRepo  = '';
  let gitlabToken = '';
  let gitlabHost  = '';
  let jiraDomain  = '';
  let jiraEmail   = '';
  let jiraToken   = '';

  onMount(async () => {
    githubToken = (await invoke<string | null>('get_setting', { key: 'github_token' })) || '';
    githubRepo  = (await invoke<string | null>('get_setting', { key: 'github_repo'  })) || '';
    gitlabToken = (await invoke<string | null>('get_setting', { key: 'gitlab_token' })) || '';
    gitlabHost  = (await invoke<string | null>('get_setting', { key: 'gitlab_host'  })) || '';
    jiraDomain  = (await invoke<string | null>('get_setting', { key: 'jira_domain'  })) || '';
    jiraEmail   = (await invoke<string | null>('get_setting', { key: 'jira_email'   })) || '';
    jiraToken   = (await invoke<string | null>('get_setting', { key: 'jira_token'   })) || '';
  });

  async function saveSetting(key: string, value: string) {
    try {
      await invoke('set_setting', { key, value });
    } catch (e) {
      dispatch('message', { text: $strings.errorPrefix + e, type: 'error' });
      throw e;
    }
  }

  async function saveGithub() {
    await saveSetting('github_token', githubToken);
    await saveSetting('github_repo',  githubRepo);
    dispatch('message', { text: $strings.githubSettingsSaved, type: 'success' });
  }

  async function saveGitlab() {
    await saveSetting('gitlab_host',  gitlabHost);
    await saveSetting('gitlab_token', gitlabToken);
    dispatch('message', { text: $strings.gitlabSettingsSaved, type: 'success' });
  }

  async function saveJira() {
    await saveSetting('jira_domain', jiraDomain);
    await saveSetting('jira_email',  jiraEmail);
    await saveSetting('jira_token',  jiraToken);
    dispatch('message', { text: $strings.jiraSettingsSaved, type: 'success' });
  }
</script>

<div class="settings-card full-width">
  <div class="card-header">
    <div class="header-icon integration">
      <CloudIcon size={20} />
    </div>
    <div class="header-text">
      <h3>{$strings.integrations}</h3>
      <p>{$strings.integrationsDesc}</p>
    </div>
  </div>
  <div class="card-content">
    <div class="integrations-grid">

      <!-- GitHub -->
      <div class="integration-subcard">
        <div class="subcard-title"><Github size={18} /><span>GitHub</span></div>
        <div class="field">
          <label for="gh-token">{$strings.personalAccessToken}</label>
          <p class="field-hint">{@html $strings.needsRepoScope}</p>
          <input id="gh-token" type="password" bind:value={githubToken} placeholder="ghp_..." class="premium-input-field mb-2" />
          <label for="gh-repo">{$strings.repositoryOptional}</label>
          <p class="field-hint">{@html $strings.repoFormatHint}</p>
          <div class="input-with-action">
            <input id="gh-repo" type="text" bind:value={githubRepo} placeholder="owner/repo" class="premium-input-field" />
            <button class="save-icon-btn" on:click={saveGithub} title={$strings.saveGithubSettings}><Check size={18} /></button>
          </div>
        </div>
      </div>

      <!-- GitLab -->
      <div class="integration-subcard">
        <div class="subcard-title"><Gitlab size={18} /><span>{$strings.gitlab}</span></div>
        <div class="field">
          <label for="gl-host">{$strings.gitlabHost}</label>
          <input id="gl-host" type="text" bind:value={gitlabHost} placeholder="https://gitlab.com" class="premium-input-field mb-2" />
          <label for="gl-token">{$strings.personalAccessToken}</label>
          <div class="input-with-action">
            <input id="gl-token" type="password" bind:value={gitlabToken} placeholder="glpat-..." class="premium-input-field" />
            <button class="save-icon-btn" on:click={saveGitlab} title={$strings.saveGitlabSettings}><Check size={18} /></button>
          </div>
        </div>
      </div>

      <!-- Jira -->
      <div class="integration-subcard">
        <div class="subcard-title"><span>{$strings.jira}</span></div>
        <div class="field">
          <label for="jr-domain">{$strings.domain}</label>
          <input id="jr-domain" type="text" bind:value={jiraDomain} placeholder="company.atlassian.net" class="premium-input-field mb-2" />
          <label for="jr-email">{$strings.email}</label>
          <input id="jr-email" type="text" bind:value={jiraEmail} placeholder="name@company.com" class="premium-input-field mb-2" />
          <label for="jr-token">{$strings.apiToken}</label>
          <div class="input-with-action">
            <input id="jr-token" type="password" bind:value={jiraToken} placeholder="ATATT..." class="premium-input-field" />
            <button class="save-icon-btn" on:click={saveJira} title={$strings.saveJiraSettings}><Check size={18} /></button>
          </div>
        </div>
      </div>

    </div>
  </div>
</div>

<style>
  .settings-card {
    background: var(--bg-card); border: 1px solid var(--border);
    border-radius: 1.25rem; padding: 1.5rem;
    box-shadow: var(--shadow-lg); transition: box-shadow 0.2s;
  }
  .settings-card:hover { box-shadow: var(--shadow); }
  .full-width { grid-column: span 2; }
  .card-header { display: flex; gap: 1rem; margin-bottom: 2rem; }
  .header-icon {
    width: 42px; height: 42px; border-radius: 12px;
    display: flex; align-items: center; justify-content: center; flex-shrink: 0;
  }
  .header-icon.integration { background: color-mix(in srgb, var(--accent-blue) 10%, transparent); color: var(--accent-blue); }
  .header-text h3 { margin: 0; font-size: 1.125rem; font-weight: 700; color: var(--text); }
  .header-text p  { margin: 0.25rem 0 0 0; font-size: 0.8125rem; color: var(--text-muted); }
  .card-content   { display: flex; flex-direction: column; gap: 1rem; }
  .integrations-grid { display: grid; grid-template-columns: repeat(3,1fr); gap: 1.25rem; }
  .integration-subcard {
    background: var(--bg-page); border: 1px solid var(--border);
    border-radius: 14px; padding: 1.25rem;
  }
  .subcard-title {
    display: flex; align-items: center; gap: 0.75rem;
    font-weight: 700; margin-bottom: 1.25rem; color: var(--text);
  }
  .field { display: flex; flex-direction: column; gap: 0.35rem; }
  .field label { font-size: 0.75rem; font-weight: 600; color: var(--text-muted); }
  .field-hint {
    font-size: 0.7rem; color: var(--text-muted); margin: 0; line-height: 1.4; opacity: 0.8;
  }
  :global(.field-hint code) {
    font-family: 'SF Mono','Roboto Mono',monospace;
    background: var(--btn-secondary-hover-bg); padding: 0.1em 0.3em;
    border-radius: 3px; font-size: 0.85em;
  }
  .premium-input-field {
    width: 100%; background: var(--bg-card); border: 1px solid var(--border);
    border-radius: 10px; padding: 0.625rem; color: var(--text); font-size: 0.875rem;
    outline: none; transition: all 0.2s; box-sizing: border-box; font-family: inherit;
  }
  .premium-input-field:focus { border-color: var(--accent-blue); background: var(--bg-page); }
  .input-with-action { display: flex; gap: 0.5rem; }
  .save-icon-btn {
    width: 38px; height: 38px; display: flex; align-items: center; justify-content: center;
    border-radius: 10px; border: 1px solid color-mix(in srgb, var(--accent-blue) 20%, transparent);
    background: color-mix(in srgb, var(--accent-blue) 10%, transparent); color: var(--accent-blue);
    cursor: pointer; transition: all 0.2s; flex-shrink: 0;
  }
  .save-icon-btn:hover { background: var(--accent-blue); color: white; }
  .mb-2 { margin-bottom: 1rem; }
  @media (max-width: 800px) {
    .full-width { grid-column: span 1; }
    .integrations-grid { grid-template-columns: 1fr; }
  }
</style>
