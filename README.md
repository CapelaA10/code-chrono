# Code Chrono 🕐

> A privacy-first desktop task and time tracker built for developers. Pomodoro timer, GitHub/GitLab/Jira sync, local SQLite storage. **No accounts. No cloud. Yours.**

![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![Tauri](https://img.shields.io/badge/backend-Rust%20%2B%20Tauri-orange)
![Svelte](https://img.shields.io/badge/frontend-Svelte-red)

---

## 🙋 The Story

Code Chrono was created by **Pedro Capela**, a developer who woke up one too many mornings unable to answer the question: *"what exactly did I work on yesterday?"*

Not just the tasks, but the time. The focus. The drift. The rabbit holes. As a developer juggling multiple repos and tools, the overhead of tracking any of it felt impossible, so he stopped trying. That made things worse.

The idea was simple: attach Pomodoro sessions to real tasks so at the end of the day there's actual *data* to look back at. Seeing patterns made it possible to improve them.

Then the repos multiplied. GitHub issues here, GitLab tickets there, Jira boards somewhere else. Copy-pasting issue titles got old fast, so integrations landed. The import drawer was born so you can selectively pull in only what matters.

The whole thing was built with **Tauri + SvelteKit + Rust**, with AI as a pair programmer to keep momentum high and the code clean. It ships as a native desktop app, fully offline, no cloud, no account required.

The community gave Pedro everything: every library, tutorial, and answered Stack Overflow post. Code Chrono is his way of giving something back. If it helps you, contributing, forking, or even leaving a ⭐ is a meaningful thing to do.

---

## ✨ Features

| Feature | Description |
|---|---|
| ⏱ **Pomodoro Timer** | Sessions, automatic breaks (short/long), idle detection, notifications |
| 📋 **Task Management** | Projects, tags, priorities, task templates, drag-to-reorder |
| ☕ **Break Timer** | Seamless transitions from Pomodoro sessions into short or long breaks |
| 📅 **Calendar** | Monthly grid view tracking tasks with scheduled due dates |
| 🔗 **Integrations** | Selectively import issues from GitHub, GitLab, and Jira |
| 🔍 **Task Filters** | Filter your task list by project, tag, and status with dismissible chips |
| 📊 **Statistics** | Time-by-task, daily breakdown, 12-week heatmap, bar charts, CSV export |
| 🌍 **Localization** | Interface available in English, Portuguese (PT/BR), Spanish, and Greek |
| 🔄 **Auto Updates** | Silent, secure background updates powered by Minisign and Tauri |
| ⌨️ **Global Hotkey** | `Ctrl+Shift+P` (or `⌘⇧P` on Mac) to pause/resume from anywhere |
| 🌙 **Dark Mode** | Light and dark themes, persisted per device |
| 🔒 **Privacy First** | All data stored locally in SQLite. Nothing sent to any server. |
| 🪟 **Thematic Windows** | Native title bar colors automatically sync with light/dark themes. |
| 🔔 **Smart Notifications** | Configurable notifications for session start, session end, and 4-Pomodoro break reminders |
| 🖥️ **IDE Detection** | Auto-detects 40+ IDEs including VS Code, Cursor, Windsurf, PearAI, Void, Trae, all JetBrains IDEs, Xcode, and more — prompts you to start tracking when you open one |

---

## 📥 Installation

### 🍎 macOS (Apple Silicon & Intel)
The easiest way to install Code Chrono and bypass the Gatekeeper warning is to run this in your Terminal:

```bash
curl -sL https://raw.githubusercontent.com/CapelaA10/code-chrono/main/install.sh | bash
```
> *Downloads the latest release, removes the quarantine attribute, and moves it to `/Applications` automatically.*

### 🪟 Windows
1. Go to the [Releases](https://github.com/CapelaA10/code-chrono/releases/latest) page.
2. Download `Code.Chrono_x.x.x_x64-setup.exe`.
3. Double-click to install. If SmartScreen blocks it, click "More info" then "Run anyway".

### 🐧 Linux
1. Go to the [Releases](https://github.com/CapelaA10/code-chrono/releases/latest) page.
2. Download either the `.deb` or `.AppImage` depending on your distribution.

---

## 🚀 Quick Start

### Prerequisites

- [Node.js](https://nodejs.org) 18 or newer
- [Rust](https://rustup.rs) stable toolchain
- Tauri system dependencies: [tauri.app/start/prerequisites](https://tauri.app/start/prerequisites/)

### Development

```bash
git clone https://github.com/CapelaA10/code-chrono
cd code-chrono
npm install
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
1. Go to **Settings > Integrations > GitHub**
2. Create a [Personal Access Token](https://github.com/settings/tokens) with `repo` or `public_repo` scope
3. Optionally enter a repository in `owner/repo` format (leave blank to fetch all issues assigned to you)
4. Click **Save**, then click **GitHub** in the sidebar to open the import drawer
5. Filter, select issues, and optionally import GitHub labels as local tags

### GitLab
1. Go to **Settings > Integrations > GitLab**
2. Create a [Personal Access Token](https://gitlab.com/-/profile/personal_access_tokens) with `read_api` scope
3. Set the host (default: `https://gitlab.com`) and your token
4. Click **Save**, then click **GitLab** in the sidebar
5. Filter by project or label, select what you want, and optionally import labels as tags

### Jira
1. Go to **Settings > Integrations > Jira**
2. Enter your Atlassian domain (e.g. `company.atlassian.net`)
3. Create an [API Token](https://id.atlassian.com/manage-profile/security/api-tokens) and enter your email and token
4. Click **Save**, then click **Jira** in the sidebar
5. Select issues and optionally import Jira labels as local tags

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
│   │   │   ├── calendar/         # CalendarCell, CalendarGrid, CalendarHeader
│   │   │   ├── integrations/     # SyncPreviewModal + sub-components, syncTypes.ts
│   │   │   ├── settings/
│   │   │   │   ├── SettingsAppearance.svelte
│   │   │   │   ├── SettingsProductivity.svelte
│   │   │   │   ├── SettingsNotifications.svelte   # v0.3.0
│   │   │   │   ├── SettingsPrograms.svelte        # v0.3.0
│   │   │   │   ├── SettingsIntegrations.svelte
│   │   │   │   ├── SettingsDataManagement.svelte
│   │   │   │   └── SettingsDangerZone.svelte
│   │   │   ├── sidebar/          # SidebarLogo, SidebarNav, SidebarIntegrations
│   │   │   ├── stats/            # StatsSummary, StatsTimeByTask, StatsDailyBreakdown, StatsHeatmap, StatsBarChart
│   │   │   ├── task/             # TaskFilterBar, TaskEditModal, TaskCheckbox, TaskMeta, templates
│   │   │   ├── timer/            # TimerWidget, BreakBanner
│   │   │   ├── ProgramNotificationModal.svelte    # v0.3.0
│   │   │   ├── Header.svelte
│   │   │   ├── QuickAdd.svelte
│   │   │   ├── Sidebar.svelte
│   │   │   ├── TaskItem.svelte
│   │   │   ├── TaskList.svelte
│   │   │   └── ThemeToggle.svelte
│   │   ├── i18n/
│   │   │   ├── locales/ (en, pt, br, es, el)
│   │   │   └── store.ts
│   │   ├── stores/
│   │   ├── types/
│   │   └── utils/
│   └── routes/
│       ├── +layout.svelte
│       ├── +page.svelte
│       ├── calendar/
│       ├── settings/
│       └── stats/
└── src-tauri/                     # Rust backend
    └── src/
        ├── main.rs
        ├── lib.rs
        ├── integrations.rs
        ├── commands/
        │   ├── timer.rs
        │   ├── notifications.rs   # v0.3.0
        │   ├── programs.rs        # v0.3.0
        │   ├── tasks.rs
        │   ├── projects.rs
        │   ├── tags.rs
        │   ├── settings.rs
        │   ├── stats.rs
        │   ├── data.rs
        │   └── sync.rs
        └── database/
            ├── mod.rs
            ├── programs.rs        # v0.3.0
            ├── models.rs
            ├── sessions.rs
            ├── tasks.rs
            ├── projects.rs
            ├── tags.rs
            └── settings.rs
```

---

## 🚀 Releasing

This project uses an automated GitHub Actions CI/CD pipeline.

To ship a new release for macOS, Windows, and Linux:

1. Update the `"version"` field in `src-tauri/tauri.conf.json`.
2. Commit with the word `release:` in the message.
3. Push to `main`.

```bash
git add src-tauri/tauri.conf.json
git commit -m "release: 0.3.0"
git push origin main
```

4. Watch the build on your [Actions tab](https://github.com/CapelaA10/code-chrono/actions).
5. In about 15 minutes a draft release with all installers will be published automatically.

*Releases are signed with Minisign for secure auto-updates. For auto-update to work you **must** set the `TAURI_SIGNING_PRIVATE_KEY` secret in your GitHub repository (Settings → Secrets → Actions) — without it, `tauri-action` silently omits `latest.json` from the release and the updater cannot find new versions. Binaries are not yet signed with an EV certificate, so Windows may show a SmartScreen prompt and macOS users should use the `install.sh` script to bypass Gatekeeper.*

---

## 🤝 Contributing

Contributions are welcome!

1. **Fork** the repository
2. **Create a branch**: `git checkout -b feature/my-feature`
3. **Make your changes** and test with `npm run tauri dev`
4. **Commit**: `git commit -m 'feat: add my feature'`
5. **Push**: `git push origin feature/my-feature`
6. **Open a Pull Request**

Ideas worth contributing:
- Advanced reporting and data visualizations
- Additional integrations (Trello, Asana, Linear)
- Custom notification sounds
- Desktop widgets

---

## 📄 License

MIT © [Pedro Capela](https://github.com/CapelaA10)

See [LICENSE](./LICENSE) for details.