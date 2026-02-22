# Code Chrono 🕐

> Privacy-first desktop task & time tracker built for developers. Pomodoro timer, GitHub/GitLab/Jira sync, local-first SQLite storage. **No accounts. No cloud. Yours.**

![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![Tauri](https://img.shields.io/badge/backend-Rust%20%2B%20Tauri-orange)
![Svelte](https://img.shields.io/badge/frontend-Svelte-red)

---

## 🙋 The Story

Code Chrono was created by **Pedro Capela** — a developer who woke up one too many mornings unable to answer the question: *"what exactly did I work on yesterday?"*

Not just the tasks, but the **time**. The focus. The drift. The rabbit holes. As a developer spread across multiple repos and tools, the cognitive overhead of tracking any of it felt insane — so he stopped trying, and that made things worse.

The idea was simple: attach Pomodoro sessions to real tasks so at the end of the day there's actual *data* to look back at. Seeing patterns made it possible to improve them.

Then the repos multiplied. GitHub issues here, GitLab tickets there, Jira boards somewhere else. Copy-pasting issue titles got old fast — so integrations landed. The import drawer was born so you can selectively pull in only what matters, not sync everything blindly.

The whole thing was built with **Tauri + SvelteKit + Rust**, with AI as a pair-programmer to keep momentum high and the code clean. It follows best practices as closely as possible and ships as a native desktop app — **fully offline, no cloud, no account required**.

The community gave Pedro everything — every library, tutorial, and answered Stack Overflow post. Code Chrono is his way of giving something back. If it helps you, contributing, forking, or even leaving a ⭐ is a meaningful thing to do.

---

## ✨ Features

| Feature | Description |
|---|---|
| ⏱ **Pomodoro Timer** | Customizable session durations, idle detection, system notifications |
| 📋 **Task Management** | Projects, tags, priorities, due dates, drag-to-reorder |
| 🔗 **Integrations** | Selectively import issues from GitHub, GitLab, and Jira with preview & filter |
| 🔍 **Task Filters** | Filter your task list by project, tag, and status with dismissible pill chips |
| 📊 **Statistics** | Time-by-task, daily breakdown, custom date ranges, CSV export |
| ⌨️ **Global Hotkey** | `Ctrl+Shift+P` (or `⌘⇧P` on Mac) to pause/resume from anywhere |
| 🌙 **Dark Mode** | Light and dark themes, persisted per device |
| 🔒 **Privacy First** | All data stored locally in SQLite — nothing sent to any server |

---

## 📥 Installation

### 🍎 macOS (Apple Silicon & Intel)
The simplest way to install `Code Chrono` and safely bypass Apple's "Damaged App" Gatekeeper warning is by running this single command in your Terminal:

```bash
curl -sL https://raw.githubusercontent.com/CapelaA10/code-chrono/main/install.sh | bash
```
> *This automatically downloads the latest release, extracts it, removes the quarantine attribute, and moves it to your `/Applications` folder.*

### 🪟 Windows
1. Head over to the [Releases](https://github.com/CapelaA10/code-chrono/releases/latest) page.
2. Download the `Code.Chrono_x.x.x_x64-setup.exe` file.
3. Double-click to install. *(If SmartScreen blocks it, click "More info" > "Run anyway")*.

### 🐧 Linux
1. Head over to the [Releases](https://github.com/CapelaA10/code-chrono/releases/latest) page.
2. Download either the `.deb` or `.AppImage` file depending on your distribution.

---

## 🚀 Quick Start

### Prerequisites

- [Node.js](https://nodejs.org) ≥ 18
- [Rust](https://rustup.rs) (stable toolchain)
- Platform prerequisites for Tauri: [tauri.app/start/prerequisites](https://tauri.app/start/prerequisites/)

### Development

```bash
# Clone the repo
git clone https://github.com/CapelaA10/code-chrono
cd code-chrono

# Install frontend dependencies
npm install

# Run in development mode (auto-reloads on changes)
npm run tauri dev
```

### Building a Release

```bash
npm run tauri build
```

Binaries are output to `src-tauri/target/release/bundle/`.

---

## 🔗 Integration Setup

### GitHub
1. Go to **Settings → Integrations → GitHub**
2. Create a [Personal Access Token](https://github.com/settings/tokens) with `repo` (or `public_repo`) scope
3. Optionally specify a repository (`owner/repo`) — leave blank to fetch all issues assigned to you
4. Click **Save**, then click **GitHub** in the sidebar to open the import drawer
5. Filter, select issues, and optionally import GitHub labels as local tags

### GitLab
1. Go to **Settings → Integrations → GitLab**
2. Create a [Personal Access Token](https://gitlab.com/-/profile/personal_access_tokens) with `read_api` scope
3. Set the host (default: `https://gitlab.com`) and your token
4. Click **Save**, then click **GitLab** in the sidebar to open the import drawer
5. Filter issues by project or label, select what you want, and optionally import labels as tags

### Jira
1. Go to **Settings → Integrations → Jira**
2. Enter your Atlassian domain (e.g. `company.atlassian.net`)
3. Create an [API Token](https://id.atlassian.com/manage-profile/security/api-tokens) and enter your email + token
4. Click **Save**, then click **Jira** in the sidebar to open the import drawer
5. Select the issues you want; Jira labels can be imported as local tags

---

## 🛠 Tech Stack

| Layer | Technology |
|---|---|
| Frontend | [Svelte 5](https://svelte.dev) + [SvelteKit](https://kit.svelte.dev) |
| Backend | [Rust](https://rust-lang.org) + [Tauri 2](https://tauri.app) |
| Database | [SQLite](https://sqlite.org) via `rusqlite` |
| HTTP Client | [reqwest](https://github.com/seanmonstar/reqwest) |
| Icons | [lucide-svelte](https://lucide.dev) |

---

## 📁 Project Structure

```text
code-chrono/
├── src/                          # Svelte frontend
│   ├── lib/
│   │   ├── components/
│   │   │   ├── integrations/
│   │   │   │   ├── syncTypes.ts                # Shared ExternalTask interface
│   │   │   │   ├── SyncPreviewModal.svelte     # Orchestrator: state + async logic
│   │   │   │   ├── SyncDrawerHeader.svelte     # Source badge, title, count badges, close
│   │   │   │   ├── SyncFilterBar.svelte        # Search, project/label selects, hide-imported
│   │   │   │   ├── SyncIssueList.svelte        # Select-all bar + scrollable issue rows
│   │   │   │   └── SyncDrawerFooter.svelte     # Import options checkboxes + action buttons
│   │   │   ├── settings/
│   │   │   │   ├── SettingsAppearance.svelte   # Compact inline light/dark pill toggle
│   │   │   │   ├── SettingsProductivity.svelte # Hotkey, idle, timer, auto-import projects
│   │   │   │   ├── SettingsIntegrations.svelte # GitHub / GitLab / Jira credential forms
│   │   │   │   ├── SettingsDataManagement.svelte
│   │   │   │   └── SettingsDangerZone.svelte
│   │   │   ├── sidebar/
│   │   │   │   ├── SidebarIntegrations.svelte  # Platform buttons → opens SyncPreviewModal
│   │   │   │   ├── SidebarNav.svelte
│   │   │   │   ├── SidebarLogo.svelte
│   │   │   │   └── InlineCreateForm.svelte
│   │   │   ├── stats/
│   │   │   │   ├── StatsSummary.svelte
│   │   │   │   ├── StatsTimeByTask.svelte
│   │   │   │   └── StatsDailyBreakdown.svelte
│   │   │   ├── task/
│   │   │   │   ├── TaskFilterBar.svelte        # Project / tag / status filter bar
│   │   │   │   ├── TaskEditModal.svelte
│   │   │   │   ├── TaskCheckbox.svelte
│   │   │   │   └── TaskMeta.svelte
│   │   │   ├── timer/
│   │   │   │   └── TimerWidget.svelte
│   │   │   ├── Header.svelte
│   │   │   ├── QuickAdd.svelte
│   │   │   ├── Sidebar.svelte
│   │   │   ├── TaskItem.svelte
│   │   │   ├── TaskList.svelte
│   │   │   └── ThemeToggle.svelte
│   │   ├── stores/
│   │   │   ├── tasks.ts           # Reactive stores + refreshAll() + filter stores
│   │   │   ├── theme.ts
│   │   │   ├── timer.ts
│   │   │   ├── timerSettings.ts
│   │   │   └── idle.ts
│   │   ├── types/
│   │   │   └── index.ts           # Single source of truth for all TS interfaces
│   │   └── utils/
│   │       └── format.ts          # formatTime, formatDuration, formatDate
│   └── routes/
│       ├── +layout.svelte         # App shell (sidebar + main area)
│       ├── +page.svelte           # Main task view (includes TaskFilterBar)
│       ├── settings/
│       │   └── +page.svelte       # Thin orchestrator; composes settings sub-components
│       └── stats/
│           └── +page.svelte       # Statistics page
└── src-tauri/                     # Rust backend
    └── src/
        ├── main.rs
        ├── lib.rs                 # Command registration & wiring
        ├── integrations.rs        # GitHub / GitLab / Jira API clients + ExternalTask
        ├── commands/
        │   ├── timer.rs
        │   ├── tasks.rs
        │   ├── projects.rs
        │   ├── tags.rs
        │   ├── settings.rs
        │   ├── stats.rs
        │   ├── data.rs
        │   └── sync.rs            # preview_sync_*, import_selected (labels + projects)
        └── database/
            ├── mod.rs             # Database façade
            ├── models.rs
            ├── sessions.rs
            ├── tasks.rs           # + is_imported()
            ├── projects.rs        # + find_or_create() — idempotent project creation
            ├── tags.rs
            └── settings.rs
```

---

## 🚀 Releasing

This project uses an automated GitHub Actions CI/CD pipeline.

To create a new release across all platforms (macOS, Windows, Linux) and generate installable binaries (`.dmg`, `.exe`, `.msi`, `.deb`, `.AppImage`):

1. **Update the Version:** Manually update the `"version"` field inside `src-tauri/tauri.conf.json` (e.g., from `"0.1.0"` to `"0.2.0"`).
2. **Commit with 'release:':** The pipeline is strictly triggered by commits containing the exact word `release:` in the message.
3. **Push to Main:** Push the commit to the `main` or `master` branch.

```bash
git add src-tauri/tauri.conf.json
git commit -m "release: 0.2.0 prod"
git push origin main
```

4. Go to your repository's [Actions tab](https://github.com/CapelaA10/code-chrono/actions) to watch the CI runners compile the binaries for each OS natively.
5. In ~15 minutes, a new **Draft Release** with all your application installers attached will be automatically published on your GitHub repository!

*Note: Since these binaries are currently unsigned, Windows users may need to bypass SmartScreen by clicking "More info" > "Run anyway". macOS users are highly encouraged to use the `install.sh` script described in the Installation section to automatically sidestep Gatekeeper.*

---

## 🤝 Contributing

Contributions are welcome! Here's how:

1. **Fork** the repository
2. **Create a branch**: `git checkout -b feature/my-feature`
3. **Make your changes** and test them with `npm run tauri dev`
4. **Commit**: `git commit -m 'feat: add my feature'`
5. **Push**: `git push origin feature/my-feature`
6. **Open a Pull Request**

### Areas Welcome for Contribution
- Improved statistics and charts
- Calendar view for tasks with due dates
- Break timer / Pomodoro break phase
- Task templates
- Localization / i18n

---

## 📄 License

MIT © [Pedro Capela](https://github.com/CapelaA10)

See [LICENSE](./LICENSE) for details.