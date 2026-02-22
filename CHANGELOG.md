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

## [0.2.0] — 2026-02-22 · Selective Import & UX Polish

### Added

#### 🔄 Automatic Updates
- Bundled **@tauri-apps/plugin-updater** for completely silent background updates.
- App checks for updates silently on launch. If found, automatically downloads them.
- Beautiful unobtrusive pill toast appears bottom-right while downloading, then turns green to prompt restart when ready.

#### ℹ️ About Dialog
- New **About** button in the sidebar footer (below Settings)
- Opens a personal story dialog explaining why Code Chrono was created: Pedro's frustration with forgetting what he worked on, the evolution from a personal tracker to a multi-repo integration tool, the Tauri + AI build story, and the open-source spirit
- Links to GitHub and credits Pedro as the author

#### 🔗 Integrations — Selective Issue Import
- **Compact right-side drawer** opens when clicking a platform in the sidebar (slides in, 360 px wide)
- Fetches issues without importing; issues already in the DB are greyed out with a check mark
- **Filter bar**: search by title, filter by project and label, toggle to hide already-imported issues
- **Select / deselect individual issues** or use "Select All" for the current filtered view
- **Optional label import**: a checkbox brings platform labels in as local tags
- **Auto-import projects**: when an issue has a project/repo, a matching local project is automatically created on first import
- External-link (↗) visible on hover per row — keeps the list uncluttered

#### ⚙️ Settings — Auto-import Projects Toggle
- New **"Auto-import Projects"** toggle in Productivity settings (default: on)
- When enabled, importing any issue that has an associated project will automatically create a matching local project (and use it if it already exists — no duplicates)
- The import drawer reads this setting on open as its default; the user can override per-import session

#### 🗂 Task View — Filter Bar
- New **filter bar** on the main All Tasks view with dropdowns for Project, Tag, and Status
- Active filters shown as **dismissible pills** (click the × on a pill to remove that one filter)
- "Clear" button resets all active filters at once
- Filter bar only renders dropdowns for data that actually exists (no empty project list when no projects)

### Changed

#### ⚙️ Settings Page — Refactored into Components
The monolithic 868-line `settings/+page.svelte` has been split into focused sub-components:
- `SettingsAppearance.svelte` — compact inline light/dark pill toggle (no longer a full-width switcher card)
- `SettingsProductivity.svelte` — hotkey, idle threshold, timer duration, **auto-import projects** toggle
- `SettingsIntegrations.svelte` — GitHub / GitLab / Jira credential forms
- `SettingsDataManagement.svelte` — CSV export / import
- `SettingsDangerZone.svelte` — database reset
- `settings/+page.svelte` is now a thin orchestrator (~60 lines)

#### 🗂 Import Drawer — Refactored into Sub-components
The monolithic `SyncPreviewModal.svelte` (456 lines) has been split into:
- `SyncDrawerHeader.svelte` — source badge, title, count badges, close button
- `SyncFilterBar.svelte` — search, project/label selects, hide-imported toggle, refresh
- `SyncIssueList.svelte` — select-all bar + scrollable issue rows
- `SyncDrawerFooter.svelte` — import-option checkboxes, error display, Cancel/Import buttons
- `syncTypes.ts` — shared `ExternalTask` interface (single source of truth)
- `SyncPreviewModal.svelte` is now a thin orchestrator (~170 lines of pure logic + shell CSS)

#### 🔄 Integration Architecture
- `integrations.rs` — `ExternalTask` now carries `labels`, `project`, and `already_imported` fields
- `commands/sync.rs` — `import_selected` now accepts `import_projects: bool`; auto-creates local projects from issue metadata
- `database/projects.rs` — new `find_or_create(name)` helper; idempotent project creation
- `database/tasks.rs` — new `is_imported(external_id, source)` helper for fast duplicate detection
- Per-page limit raised from 50 → 100 issues per sync across all platforms
- Legacy `sync_github/jira/gitlab` commands kept for backwards compatibility

### Architecture — New Files

#### Frontend (`src/`)
| Path | Responsibility |
|---|---|
| `lib/components/integrations/syncTypes.ts` | Shared `ExternalTask` TypeScript interface |
| `lib/components/integrations/SyncPreviewModal.svelte` | Orchestrator: state + async logic |
| `lib/components/integrations/SyncDrawerHeader.svelte` | Drawer header zone |
| `lib/components/integrations/SyncFilterBar.svelte` | Search + filter dropdowns |
| `lib/components/integrations/SyncIssueList.svelte` | Select-all bar + issue rows |
| `lib/components/integrations/SyncDrawerFooter.svelte` | Import options + action buttons |
| `lib/components/task/TaskFilterBar.svelte` | Project / tag / status filter bar |
| `lib/components/settings/SettingsAppearance.svelte` | Compact appearance card |
| `lib/components/settings/SettingsProductivity.svelte` | Productivity card (+ auto-import projects toggle) |
| `lib/components/settings/SettingsIntegrations.svelte` | Integrations card |
| `lib/components/settings/SettingsDataManagement.svelte` | Data management card |
| `lib/components/settings/SettingsDangerZone.svelte` | Danger zone card |

#### Backend (`src-tauri/src/`)
| Path | Change |
|---|---|
| `integrations.rs` | `ExternalTask` extended with `labels`, `project`, `already_imported` |
| `commands/sync.rs` | Added `preview_sync_*`, `import_selected` (with `import_projects` param) |
| `database/projects.rs` | Added `find_or_create()` — idempotent project creation |
| `database/tasks.rs` | Added `is_imported()` helper |

---

*Future versions will be documented here as they are released.*
