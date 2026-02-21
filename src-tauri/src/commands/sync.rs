// commands/sync.rs — External integration sync (GitHub, GitLab, Jira)
//
// Each `sync_*` command:
//   1. Reads the required credentials from the settings table.
//   2. Calls the corresponding async fetcher in `crate::integrations`.
//   3. Stores new tasks via `db.save_external_task` (skips duplicates by external_id).
//   4. Returns the count of newly inserted tasks.

use std::sync::{Arc, Mutex};
use tauri::State;

use crate::database::{Database, Task};
use crate::integrations::ExternalTask;

// ── Helper ────────────────────────────────────────────────────────────────

/// Convert an ExternalTask + source label into a database Task ready to insert.
fn to_db_task(et: ExternalTask, source: &str) -> Task {
    Task {
        id:          0,
        title:       et.title,
        description: et.description,
        due_date:    None,
        priority:    1,
        status:      "todo".to_string(),
        project_id:  None,
        parent_id:   None,
        position:    0,
        external_id: Some(et.id),
        source:      Some(source.to_string()),
        created_at:  0,
        completed_at: None,
        tags:        vec![],
    }
}

/// Insert a batch of external tasks and return the count of new records inserted.
fn save_all(db: &Database, tasks: Vec<ExternalTask>, source: &str) -> usize {
    tasks
        .into_iter()
        .filter_map(|et| db.save_external_task(to_db_task(et, source)).ok())
        .count()
}

// ── Commands ──────────────────────────────────────────────────────────────

/// Sync open GitHub issues.
/// Requires: `github_token` setting. Optionally `github_repo` ("owner/repo").
/// If no repo is set, fetches all issues assigned to the authenticated user.
#[tauri::command]
pub async fn sync_github(
    db_state: State<'_, Arc<Mutex<Database>>>,
) -> Result<usize, String> {
    let (token, repo) = {
        let db = db_state.lock().unwrap();
        (
            db.get_setting("github_token").unwrap_or(None),
            db.get_setting("github_repo").unwrap_or(None),
        )
    };
    let token = token.ok_or("GitHub token not configured. Go to Settings → Integrations.")?;

    let tasks = crate::integrations::fetch_github_tasks(&token, repo.as_deref()).await?;
    let db = db_state.lock().unwrap();
    Ok(save_all(&db, tasks, "GitHub"))
}

/// Sync open Jira issues assigned to the authenticated user.
/// Requires: `jira_domain`, `jira_email`, `jira_token` settings.
#[tauri::command]
pub async fn sync_jira(
    db_state: State<'_, Arc<Mutex<Database>>>,
) -> Result<usize, String> {
    let (domain, email, token) = {
        let db = db_state.lock().unwrap();
        (
            db.get_setting("jira_domain").unwrap_or(None),
            db.get_setting("jira_email").unwrap_or(None),
            db.get_setting("jira_token").unwrap_or(None),
        )
    };
    let domain = domain.ok_or("Jira domain not configured. Go to Settings → Integrations.")?;
    let email  = email.ok_or("Jira email not configured. Go to Settings → Integrations.")?;
    let token  = token.ok_or("Jira API token not configured. Go to Settings → Integrations.")?;

    let tasks = crate::integrations::fetch_jira_tasks(&domain, &email, &token).await?;
    let db = db_state.lock().unwrap();
    Ok(save_all(&db, tasks, "Jira"))
}

/// Sync open GitLab issues assigned to the authenticated user.
/// Requires: `gitlab_token` setting. Optionally `gitlab_host` (default: https://gitlab.com).
#[tauri::command]
pub async fn sync_gitlab(
    db_state: State<'_, Arc<Mutex<Database>>>,
) -> Result<usize, String> {
    let (token, host) = {
        let db = db_state.lock().unwrap();
        (
            db.get_setting("gitlab_token").unwrap_or(None),
            db.get_setting("gitlab_host").unwrap_or(None),
        )
    };
    let token = token.ok_or("GitLab token not configured. Go to Settings → Integrations.")?;
    let host  = host.unwrap_or_else(|| "https://gitlab.com".to_string());

    let tasks = crate::integrations::fetch_gitlab_tasks(&token, &host).await?;
    let db = db_state.lock().unwrap();
    Ok(save_all(&db, tasks, "GitLab"))
}
