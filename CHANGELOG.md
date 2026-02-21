# Changelog

All notable changes to **Code Chrono** are documented here.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
Versions follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.0] — 2026-02-21 · Initial Release 🚀

### Features

#### ⏱ Pomodoro Timer
- Customizable session duration (5 – 120 min) via a dropdown selector in the header
- Global keyboard shortcut `Ctrl+Shift+P` (⌘⇧P on macOS) to pause/resume from any window
- Idle auto-pause: timer pauses automatically after 2 minutes of inactivity
- System notification on session completion
- Real-time tick events pushed from Rust to the frontend (no polling)

#### 📋 Task Management
- Create tasks from the QuickAdd bar (Enter to add, `/` prefix to search)
- Full edit modal: title, description, priority, project, due date, tags
- Overdue due dates highlighted in red
- Priority badge (None / Low / Medium / High)
- Mark tasks complete / incomplete with a single click
- Delete tasks instantly (no confirmation dialog)
- Filter tasks by project, tag, or status via the sidebar

#### 🗂 Projects & Tags
- Create projects and tags inline in the sidebar (color picker included)
- Delete projects/tags with automatic cleanup of task associations
- Color-coded project folders and tag badges

#### 🔗 Integrations
- **GitHub** — sync open issues; optionally target a single `owner/repo`
- **GitLab** — sync issues assigned to you (self-hosted or gitlab.com)
- **Jira** — sync issues assigned to you via Atlassian API token
- Inline sync status (spinner → ✓ success / ✗ error with message) — no alert dialogs
- Source badge on synced tasks (GitHub / GitLab / Jira)

#### 📊 Statistics
- Time tracked per task with relative bar chart
- Daily breakdown grouped by date
- Filter by Today / This Week / This Month / Custom range
- Export session records to CSV (with automatic blob URL cleanup)

#### ⚙️ Settings
- Per-integration configuration: tokens, domain, host, GitHub repo
- Idle detection threshold (minutes)
- All settings persisted in local SQLite (no cloud, no accounts)

#### 🎨 UI & Theming
- Light and dark themes, toggleable from the header
- Smooth transitions, hover micro-animations
- Fully keyboard navigable modals (Esc to close, Ctrl+Enter to save)

### Architecture

#### Frontend (`src/`)
| Path | Responsibility |
|---|---|
| `lib/utils/format.ts` | Shared `formatTime`, `formatDuration`, `formatDate` utilities |
| `lib/types/index.ts` | Single source of truth for all TypeScript interfaces |
| `lib/stores/tasks.ts` | Reactive stores + `refreshAll()` helper |
| `components/timer/TimerWidget.svelte` | Self-contained Pomodoro widget |
| `components/task/TaskEditModal.svelte` | Reusable task edit modal |
| `components/sidebar/InlineCreateForm.svelte` | Reusable color-picker + name form |

#### Backend (`src-tauri/src/`)
| Path | Responsibility |
|---|---|
| `commands/timer.rs` | Timer state, session loop, idle detection |
| `commands/tasks.rs` | Task CRUD |
| `commands/projects.rs` | Project CRUD |
| `commands/tags.rs` | Tag CRUD |
| `commands/settings.rs` | Key-value settings |
| `commands/stats.rs` | Session statistics queries |
| `commands/data.rs` | CSV import/export, database reset |
| `commands/sync.rs` | GitHub, GitLab, Jira sync with shared helpers |
| `database/models.rs` | Domain structs (no logic) |
| `database/sessions.rs` | Pomodoro session log |
| `database/tasks.rs` | Task queries + tag linking |
| `database/projects.rs` | Project queries |
| `database/tags.rs` | Tag queries |
| `database/settings.rs` | Settings get/set |

---

*Future versions will be documented here as they are released.*
