<!-- AboutDialog.svelte
     A personal story modal explaining why Code Chrono was created by Pedro Capela.
     Emits: close -->
<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { X, Github, Heart, Clock, GitBranch, Zap } from 'lucide-svelte';

  const dispatch = createEventDispatcher<{ close: void }>();
  function close() { dispatch('close'); }
</script>

<button class="backdrop" on:click={close} aria-label="Close"></button>

<div class="dialog" role="dialog" aria-modal="true" aria-label="About Code Chrono">

  <!-- Close -->
  <button class="close-btn" on:click={close} title="Close"><X size={18} /></button>

  <!-- Header — logo mark + name -->
  <div class="hero">
    <div class="logo-ring">
      <Clock size={28} strokeWidth={1.75} />
    </div>
    <div>
      <h1 class="app-name">Code Chrono</h1>
      <span class="version-pill">v0.2.0</span>
    </div>
  </div>

  <!-- Story -->
  <div class="story">

    <div class="story-block">
      <div class="story-icon"><Clock size={16} /></div>
      <div class="story-text">
        <strong>It started with a simple frustration.</strong>
        <p>
          I'm Pedro — a developer who, every morning, stares at the screen trying to remember
          what I actually <em>did</em> the day before. Not just tasks, but time. Where did my day go?
          What was I focusing on? Was I even productive?
        </p>
      </div>
    </div>

    <div class="story-block">
      <div class="story-icon"><Zap size={16} /></div>
      <div class="story-text">
        <strong>So I built a tracker for myself.</strong>
        <p>
          Code Chrono started as a personal Pomodoro log — a way to attach sessions to real tasks
          and finally answer the question <em>"what did I do today?"</em> at the end of every day.
          Seeing the data made me understand my own patterns and where I could improve.
        </p>
      </div>
    </div>

    <div class="story-block">
      <div class="story-icon"><GitBranch size={16} /></div>
      <div class="story-text">
        <strong>Then the repos multiplied.</strong>
        <p>
          Like any developer I was juggling GitHub, GitLab, Jira — all at once. Switching tabs
          to copy issue titles got old fast. So I added integrations and the ability to selectively
          pull in issues and turn them into trackable tasks, right here.
        </p>
      </div>
    </div>

    <div class="story-block">
      <div class="story-icon">
        <!-- sparkle SVG because lucide doesn't have one -->
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
          <path d="M12 2L9.5 9.5 2 12l7.5 2.5L12 22l2.5-7.5L22 12l-7.5-2.5z"/>
        </svg>
      </div>
      <div class="story-text">
        <strong>Built with a little help from AI.</strong>
        <p>
          This app was crafted using <strong>Tauri</strong> + <strong>SvelteKit</strong> on the
          frontend and <strong>Rust</strong> on the backend — following best practices as much as
          possible. AI was a huge part of the process, helping me move faster while keeping the
          code clean and intentional.
        </p>
      </div>
    </div>

    <div class="story-block">
      <div class="story-icon"><Heart size={16} /></div>
      <div class="story-text">
        <strong>Open source — because the community gave me everything.</strong>
        <p>
          Every library, tutorial, answered Stack Overflow question, and open-source tool I've ever
          used was someone else's gift to me. Code Chrono is my way of giving something back.
          If you find it useful, <a href="https://github.com/CapelaA10/code-chrono" target="_blank" rel="noopener" class="link">contribute, fork, or just leave a ⭐</a>.
        </p>
      </div>
    </div>
  </div>

  <!-- Footer -->
  <div class="dialog-footer">
    <a
      href="https://github.com/CapelaA10/code-chrono"
      target="_blank"
      rel="noopener"
      class="gh-btn"
    >
      <Github size={16} />
      <span>View on GitHub</span>
    </a>
    <span class="made-by">Made with <Heart size={12} class="heart-icon" /> by Pedro Capela</span>
  </div>
</div>

<style>
  /* ── Backdrop ───────────────────────────────────────────────────────────── */
  .backdrop {
    position: fixed; inset: 0;
    background: rgba(0, 0, 0, 0.5);
    backdrop-filter: blur(6px);
    z-index: 300; border: none; cursor: default;
  }

  /* ── Dialog ─────────────────────────────────────────────────────────────── */
  .dialog {
    position: fixed;
    top: 50%; left: 50%;
    transform: translate(-50%, -50%);
    z-index: 301;
    width: min(520px, 92vw);
    max-height: 88vh;
    background: var(--bg-card);
    border: 1px solid var(--border);
    border-radius: 1.5rem;
    overflow-y: auto;
    box-shadow: 0 32px 80px rgba(0, 0, 0, 0.25);
    animation: appear 0.28s cubic-bezier(0.16, 1, 0.3, 1);
    scrollbar-width: none;
  }
  .dialog::-webkit-scrollbar { display: none; }

  @keyframes appear {
    from { opacity: 0; transform: translate(-50%, -47%) scale(0.96); }
    to   { opacity: 1; transform: translate(-50%, -50%) scale(1); }
  }

  /* ── Close button ───────────────────────────────────────────────────────── */
  .close-btn {
    position: absolute; top: 1.1rem; right: 1.1rem;
    width: 32px; height: 32px;
    display: flex; align-items: center; justify-content: center;
    border: none; background: none; border-radius: 8px;
    color: var(--text-muted); cursor: pointer; transition: all 0.15s;
  }
  .close-btn:hover { background: var(--btn-secondary-hover-bg); color: var(--text); }

  /* ── Hero ───────────────────────────────────────────────────────────────── */
  .hero {
    display: flex; align-items: center; gap: 1rem;
    padding: 2rem 2rem 1.5rem;
  }

  .logo-ring {
    width: 56px; height: 56px; border-radius: 16px; flex-shrink: 0;
    display: flex; align-items: center; justify-content: center;
    background: linear-gradient(135deg, #3b82f6 0%, #6366f1 100%);
    color: white;
    box-shadow: 0 8px 24px rgba(99, 102, 241, 0.35);
  }

  .app-name {
    font-size: 1.5rem; font-weight: 800;
    margin: 0; letter-spacing: -0.02em; color: var(--text);
    line-height: 1.1;
  }

  .version-pill {
    display: inline-block;
    margin-top: 0.3rem;
    font-size: 0.68rem; font-weight: 700; text-transform: uppercase;
    letter-spacing: 0.08em; padding: 0.2rem 0.6rem; border-radius: 99px;
    background: rgba(99, 102, 241, 0.1); color: #6366f1;
    border: 1px solid rgba(99, 102, 241, 0.2);
  }

  /* ── Story blocks ───────────────────────────────────────────────────────── */
  .story {
    padding: 0 2rem;
    display: flex; flex-direction: column; gap: 0;
  }

  .story-block {
    display: flex; gap: 1rem; align-items: flex-start;
    padding: 1.1rem 0;
    border-bottom: 1px solid var(--border);
  }
  .story-block:last-child { border-bottom: none; }

  .story-icon {
    flex-shrink: 0;
    width: 30px; height: 30px; border-radius: 8px;
    display: flex; align-items: center; justify-content: center;
    background: var(--bg-page); color: var(--accent-blue);
    border: 1px solid var(--border);
    margin-top: 0.125rem;
  }

  .story-text { flex: 1; min-width: 0; }

  .story-text strong {
    display: block;
    font-size: 0.8875rem; font-weight: 700; color: var(--text);
    margin-bottom: 0.35rem;
  }

  .story-text p {
    margin: 0;
    font-size: 0.8125rem; line-height: 1.6;
    color: var(--text-muted);
  }

  .story-text em {
    font-style: italic; color: var(--text);
  }

  .link {
    color: var(--accent-blue);
    text-decoration: none;
    font-weight: 600;
  }
  .link:hover { text-decoration: underline; }

  /* ── Footer ─────────────────────────────────────────────────────────────── */
  .dialog-footer {
    display: flex; align-items: center; justify-content: space-between;
    padding: 1.25rem 2rem 1.75rem;
    margin-top: 0.5rem;
    gap: 1rem;
    flex-wrap: wrap;
  }

  .gh-btn {
    display: flex; align-items: center; gap: 0.5rem;
    padding: 0.5rem 1rem; border-radius: 9px;
    background: var(--text); color: var(--bg-page);
    font-size: 0.8125rem; font-weight: 600;
    text-decoration: none; transition: all 0.2s;
    border: 1px solid transparent;
  }
  .gh-btn:hover { opacity: 0.85; transform: translateY(-1px); }

  .made-by {
    display: flex; align-items: center; gap: 0.3rem;
    font-size: 0.75rem; color: var(--text-muted); font-weight: 500;
  }

  :global(.heart-icon) { color: #ef4444; flex-shrink: 0; }
</style>
