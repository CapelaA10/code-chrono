# Changelog

All notable changes to **Code Chrono** are documented here.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
Versions follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.4.0] — 2026-03-02 · Bug Fixes & IDE Improvements

### Fixed

#### Jira Integration — API 410 Gone
- Migrated the Jira sync from the **removed** `GET /rest/api/3/search` endpoint to the new
  **`POST /rest/api/3/search/jql`** endpoint with a JSON body, as required by Atlassian
  changelog [CHANGE-2046](https://developer.atlassian.com/changelog/#CHANGE-2046).
- Both the legacy `sync_jira` command and the `preview_sync_jira` / `import_selected` flow
  now use the correct endpoint, resolving the _"410 Gone"_ error that made Jira syncing
  completely non-functional.

#### Notifications — macOS
- Added `NSUserNotificationUsageDescription` to the macOS `Info.plist` bundle (via
  `src-tauri/Info.plist`). Without this key macOS silently denies notification permission
  on newer OS versions — which is why no system alerts appeared on Mac.
- `request_notification_permission` now correctly inspects the `PermissionState` enum
  returned by `tauri_plugin_notification` and returns `true` only when the state is
  `Granted`, instead of always returning `true` unconditionally.

#### Program / IDE Detection — multiple fixes
- **Stale modal on IDE close**: the frontend `ProgramNotificationModal` now listens for a
  new `program-closed` Tauri event and auto-dismisses if the program shown was closed while
  the modal was still open. Previously it stayed on screen indefinitely.
- **False-positive suppression**: rewrote the `prev_detected` tracking to store only *tracked
  program executable keys* instead of all system process names. The old approach stored
  every running process (hundreds), and the `exe.contains(proc_name)` check would falsely
  match short process names like `"c"` or `"od"`, making `just_opened` always `false` and
  silently suppressing every detection event.
- **Digit suffix matching** (`webstorm64`, `studio64`): the `starts_with` guard used
  `!is_ascii_alphanumeric()` which blocked digits, preventing `"webstorm64.exe"` from
  matching the `"webstorm"` key. Changed to `!is_ascii_alphabetic()` so digit suffixes
  (version numbers) are allowed while pure-letter continuations (`"codecov"` ≠ `"code"`)
  are still correctly blocked.
- **`.exe` / `.app` extension stripping**: introduced a dedicated `process_is_running()`
  helper that trims platform suffixes before comparing, so `"cursor.exe"` reliably matches
  `"cursor"` on Windows and `"Visual Studio Code.app"` matches on macOS.
- **Spurious notification after cooldown**: the watcher now only emits `program-opened`
  when a program transitions from not-running → running within a single 5-second poll
  interval. Previously it would re-fire every time the 15-minute cooldown expired if the
  IDE happened to still be open.

#### Auto-Update — root cause documented
- Identified the root cause of auto-update never working: `latest.json` was absent from
  all GitHub releases because `TAURI_SIGNING_PRIVATE_KEY` was not set as a GitHub Secret.
  `tauri-action` silently skips generating the signed update manifest when the key is
  missing. Added inline documentation to the workflow with instructions.
- Updated `UpdateChecker.svelte` to silently ignore 404 errors (expected in dev builds
  and before the first signed release), avoiding noisy console errors.

### Added

#### Expanded AI IDE Detection
The `KNOWN_IDES` list now includes all major AI-first code editors that ship as standalone
desktop applications:

| IDE | Executable stem |
|---|---|
| PearAI | `pearai` |
| Void | `void` |
| Trae (ByteDance) | `trae` |
| Aide | `aide` |
| Melty | `melty` |
| Antigravity | `antigravity` |

The list is now organised into labelled sections: AI-first IDEs, VS Code variants,
JetBrains, Apple, text editors, terminal editors, and dev tools.

### Changed
- **Release workflow** (`release.yml`): replaced the generic `releaseBody` with a direct
  link to `CHANGELOG.md` so users always know what changed in each release.
- **`Info.plist`** added at `src-tauri/Info.plist` for macOS bundle metadata.

---

## [0.3.0] — 2026-03-01 · Themes, Notifications & IDE Detection

### Added
- New **Debug** page (accessible only in development mode) for testing system integrations.
- Integrated **Notification Tests** within the Debug page to verify OS-level alerts across all platforms.
- Complete localization for all debug-related UI strings in English, Portuguese, Spanish, and Greek.

#### Thematic Window Integration
- The native OS title bar now dynamically matches the program's theme. Whether you're in light or dark mode, the window's top area syncs to the application color automatically.
- On **macOS**, the title bar is set to `Transparent` mode, allowing the app's background to flow naturally underneath the native traffic light buttons.
- On **Windows and Linux**, the native border and title area colors are updated via the Tauri Window API to stay in sync with the current theme.
- The window retains its standard native behavior, including standard button alignment and native dragging.

#### Cross-Platform Notifications
- New **Notifications** settings card with four toggles: enable notifications globally, notify on session start, notify on session end, and recommend a break after 4 consecutive Pomodoros (based on OMS rest guidelines).
- The Rust backend checks `notifications_enabled` and each per-event flag before firing any notification, so nothing pops up if you have it turned off.
- On macOS, notification permission is requested at startup. On Windows and Linux this is a no-op since permission is always granted.
- New `commands/notifications.rs` with `request_notification_permission` and `show_notification`.
- `TimerState` now tracks `pomodoro_session_count: u32`, which increments on each completed work session and resets when a break starts.

#### Program / IDE Detection
- New **Program Detection** settings card. Click **Scan for IDEs** to find installed developer tools on your machine:
  - macOS: scans `.app` bundles in `/Applications` and `~/Applications`
  - Linux: reads `.desktop` files from XDG app directories
  - Windows: queries uninstall registry keys
  - Results are matched against 30+ known tools including VS Code, Cursor, all JetBrains IDEs, Xcode, Android Studio, Zed, Neovim, Sublime Text, Postman, and more.
- Detected IDEs are saved to a `tracked_programs` SQLite table. Duplicates are skipped automatically.
- Each tracked program has its own enable/disable toggle and a delete button.
- You can also add any program manually via a file-picker dialog.
- A background watcher (`spawn_program_watcher`) uses `sysinfo` to poll running processes every 5 seconds. When a tracked program is detected, it emits a `program-opened` event to the frontend. There is a 15-minute cooldown per program to avoid repeated prompts.
- New `ProgramNotificationModal.svelte` slides in from the bottom-right when a tracked program opens. It shows a dropdown of your 20 most recent tasks and lets you start tracking immediately.

#### Localization
- All new UI strings are translated into all 5 supported languages: English, European Portuguese, Brazilian Portuguese, Spanish, and Greek.
- Because `StringKey = keyof typeof en`, missing translations are caught at build time as TypeScript errors.
- Notification body text is now fully translated. The frontend passes already-translated strings from the active locale to the Rust backend when starting a session or break, so every system notification appears in the user's chosen language.
- Flag emojis in the language selector (🇬🇧 🇵🇹 🇧🇷 🇪🇸 🇬🇷) render correctly on all supported platforms. Tauri uses a Chromium-based WebView on Windows (WebView2), WKWebView on macOS, and WebKitGTK on Linux, all of which handle flag and other emoji sequences natively without any OS font dependency.

### New Files

#### Frontend
| File | What it does |
|---|---|
| `SettingsNotifications.svelte` | Notification preferences card |
| `SettingsPrograms.svelte` | Program detection settings card |
| `ProgramNotificationModal.svelte` | Task-tracking prompt shown on program open |

#### Backend
| File | What it does |
|---|---|
| `commands/notifications.rs` | Permission request and show_notification command |
| `commands/programs.rs` | Cross-platform IDE scan, CRUD, and background watcher |
| `database/programs.rs` | CRUD for the tracked_programs SQLite table |

### Struct Updates
- `TimerState` gains `pomodoro_session_count: u32` to track consecutive work sessions.

### New Dependencies
| Crate | Version | Purpose |
|---|---|---|
| `sysinfo` | 0.33 | Cross-platform process enumeration |

---

## [0.2.0] — 2026-02-22 · Feature Expansion & UX Polish

### Added

#### Localization (i18n)
- Full interface translation support added.
- Available languages: English, European Portuguese, Brazilian Portuguese, Spanish, and Greek.
- Language switcher available in Settings > Appearance.

#### Calendar View
- New `/calendar` page to view tasks by due date.
- Monthly grid showing up to 3 task pills per day with overdue highlighting.
- Navigation link added to the left sidebar under "All Tasks".

#### Break Timer
- Pomodoro sessions now transition into breaks automatically.
- A break banner appears after each session, offering a Short Break (5 min) or Long Break (15 min).
- Break phases are tracked in the Rust backend state.

#### Statistics
- **Activity Heatmap**: 12-week GitHub-style contribution grid for daily time logged.
- **Daily Totals**: scaled SVG bar chart showing tracked time history.
- Toggle between Charts and Details views on the Statistics page.

#### Task Templates
- Save any task configuration as a reusable template from the Task Edit modal.
- Apply templates with one click via the Template Picker modal.
- Templates are stored offline in `localStorage`.

#### Automatic Updates
- Integrated `@tauri-apps/plugin-updater` for silent background updates.
- App checks for updates on launch and downloads them automatically.
- A small pill toast in the bottom-right shows download progress and prompts for restart when ready.
- Updates are signed with Minisign and verified via a public key in `tauri.conf.json`.

#### UI and Theme Improvements
- Removed all hardcoded hex colors and replaced them with CSS variables.
- Used `color-mix(in srgb, ...)` for translucent hover states that work correctly in both light and dark themes.
- Unified drop shadows across modals, toasts, dropdowns, and date pickers using global shadow tokens.
- Interactive buttons now use a filter-based hover so they respond correctly regardless of their base color.

#### About Dialog
- New **About** button in the sidebar footer.
- Opens a dialog with Pedro's personal story: where the idea came from, how the project evolved, and why it's open source.
- Links to GitHub and credits Pedro as the author.

#### Integrations: Selective Issue Import
- A compact right-side drawer opens when you click a platform in the sidebar.
- Issues that are already in the database are shown with a check mark and greyed out.
- Filter bar with search, project/label selects, and a toggle to hide already-imported issues.
- Select individual issues or use "Select All" for the current filtered view.
- Optional: import platform labels as local tags.
- Optional: auto-create a local project when an issue has an associated project or repo.

#### Settings: Auto-import Projects Toggle
- New **Auto-import Projects** toggle in Productivity settings (on by default).
- When enabled, importing an issue that has an associated project will automatically create or reuse a matching local project.

#### Task View: Filter Bar
- New filter bar on the main task view with dropdowns for Project, Tag, and Status.
- Active filters shown as dismissible pills. Click the × on a pill to remove that filter.
- A "Clear" button resets all filters at once.
- Dropdowns only render if the relevant data actually exists.

### Changed

#### Settings Page
The original 868-line settings page was split into focused components:
- `SettingsAppearance.svelte` — light/dark toggle
- `SettingsProductivity.svelte` — hotkey, idle threshold, timer duration, auto-import projects
- `SettingsIntegrations.svelte` — GitHub, GitLab, and Jira credential forms
- `SettingsDataManagement.svelte` — CSV export and import
- `SettingsDangerZone.svelte` — database reset
- `settings/+page.svelte` is now about 60 lines

#### Import Drawer
The original 456-line `SyncPreviewModal.svelte` was split into:
- `SyncDrawerHeader.svelte`
- `SyncFilterBar.svelte`
- `SyncIssueList.svelte`
- `SyncDrawerFooter.svelte`
- `syncTypes.ts` for the shared `ExternalTask` interface

#### Integration Architecture
- `ExternalTask` now carries `labels`, `project`, and `already_imported` fields.
- `import_selected` accepts `import_projects: bool` and auto-creates local projects from issue metadata.
- New `find_or_create(name)` in `database/projects.rs` for idempotent project creation.
- New `is_imported(external_id, source)` in `database/tasks.rs` for fast duplicate detection.
- Per-page issue limit raised from 50 to 100 across all platforms.

### Struct Updates
- `TimerState` gains `phase: number` (0 = work, 1 = short break, 2 = long break) to orchestrate Pomodoro cycles.

---

## [0.1.0] — 2026-02-21 · Initial Release

### Added

#### Pomodoro Timer
- Customizable session duration (5 to 120 min) via a dropdown in the header.
- Global keyboard shortcut `Ctrl+Shift+P` (or `Cmd+Shift+P` on macOS) to pause/resume from any window.
- Idle auto-pause after 2 minutes of inactivity.
- System notification on session completion.
- Real-time tick events pushed from Rust to the frontend with no polling.

#### Task Management
- Create tasks from the QuickAdd bar (Enter to add, `/` prefix to search).
- Full edit modal with title, description, priority, project, due date, and tags.
- Overdue dates highlighted in red.
- Priority badge (None, Low, Medium, High).
- Mark tasks complete or incomplete with one click.
- Delete tasks instantly.
- Filter by project, tag, or status via the sidebar.

#### Projects & Tags
- Create projects and tags inline in the sidebar with a color picker.
- Delete projects or tags with automatic cleanup of task associations.
- Color-coded folders and badges.

#### Integrations
- **GitHub**: sync open issues, optionally scoped to a single `owner/repo`.
- **GitLab**: sync issues assigned to you (self-hosted or gitlab.com).
- **Jira**: sync issues assigned to you via an Atlassian API token.
- Inline sync status that shows a spinner, then success or error. No alert dialogs.
- Source badge on synced tasks (GitHub, GitLab, or Jira).

#### Statistics
- Time tracked per task with a relative bar chart.
- Daily breakdown grouped by date.
- Filter by Today, This Week, This Month, or a custom range.
- Export session records to CSV.

#### Settings
- Per-integration config: tokens, domain, host, GitHub repo path.
- Idle detection threshold in minutes.
- All settings stored in local SQLite.

#### UI and Theming
- Light and dark themes, toggled from the header.
- Smooth transitions and hover micro-animations.
- Keyboard-navigable modals (Esc to close, Ctrl+Enter to save).

### Architecture

#### Frontend
| File | What it does |
|---|---|
| `lib/utils/format.ts` | Shared formatTime, formatDuration, formatDate utilities |
| `lib/types/index.ts` | Single source of truth for all TypeScript interfaces |
| `lib/stores/tasks.ts` | Reactive stores and refreshAll helper |
| `components/timer/TimerWidget.svelte` | Self-contained Pomodoro widget |
| `components/task/TaskEditModal.svelte` | Reusable task edit modal |
| `components/sidebar/InlineCreateForm.svelte` | Reusable color-picker and name form |

#### Backend
| File | What it does |
|---|---|
| `commands/timer.rs` | Timer state, session loop, idle detection |
| `commands/tasks.rs` | Task CRUD |
| `commands/projects.rs` | Project CRUD |
| `commands/tags.rs` | Tag CRUD |
| `commands/settings.rs` | Key-value settings |
| `commands/stats.rs` | Session statistics queries |
| `commands/data.rs` | CSV import/export and database reset |
| `commands/sync.rs` | GitHub, GitLab, and Jira sync |
| `database/models.rs` | Domain structs |
| `database/sessions.rs` | Pomodoro session log |
| `database/tasks.rs` | Task queries and tag linking |
| `database/projects.rs` | Project queries |
| `database/tags.rs` | Tag queries |
| `database/settings.rs` | Settings get/set |

---

*Future versions will be documented here as they are released.*
