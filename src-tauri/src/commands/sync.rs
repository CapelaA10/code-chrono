// commands/sync.rs — External integration sync (GitHub, GitLab, Jira)
//
// Commands:
//   preview_sync_*     — fetch issues from the platform WITHOUT saving them.
//                        Each returned ExternalTask has `already_imported` set.
//   import_selected_*  — import only the user-chosen external_ids.
//   sync_*             — legacy one-shot sync (kept for compatibility); now
//                        delegates to preview + import all non-imported.
//
// Deduplication is always enforced by `external_id + source` in the DB.

use std::sync::{Arc, Mutex};
use tauri::State;

use crate::database::{Database, Task};
use crate::integrations::ExternalTask;

// ── Private helpers ───────────────────────────────────────────────────────

/// Convert an ExternalTask + source label into a database Task ready to insert.
fn to_db_task(et: ExternalTask, source: &str) -> Task {
    Task {
        id:           0,
        title:        et.title,
        description:  et.description,
        due_date:     None,
        priority:     1,
        status:       "todo".to_string(),
        project_id:   None,
        parent_id:    None,
        position:     0,
        external_id:  Some(et.id),
        source:       Some(source.to_string()),
        created_at:   0,
        completed_at: None,
        tags:         vec![],
    }
}

/// Check which external_ids from a list are already present in the DB.
fn mark_imported(db: &Database, tasks: Vec<ExternalTask>, source: &str) -> Vec<ExternalTask> {
    tasks
        .into_iter()
        .map(|mut et| {
            et.already_imported = db.is_external_task_imported(&et.id, source).unwrap_or(false);
            et
        })
        .collect()
}

/// Insert only the tasks whose id is in `ids`, optionally resolving each task's
/// `project` name to a local project id via find_or_create_project.
fn save_by_ids(db: &Database, tasks: Vec<ExternalTask>, ids: &[String], source: &str, import_projects: bool) -> usize {
    tasks
        .into_iter()
        .filter(|et| ids.contains(&et.id))
        .filter_map(|et| {
            let mut db_task = to_db_task(et.clone(), source);
            // Resolve the external project name → local project id
            if import_projects {
                if let Some(ref proj_name) = et.project {
                    if !proj_name.is_empty() {
                        // Strip owner prefix for GitHub ("owner/repo" → "repo")
                        let short = proj_name.split('/').last().unwrap_or(proj_name);
                        db_task.project_id = db.find_or_create_project(short).ok();
                    }
                }
            }
            db.save_external_task(db_task).ok()
        })
        .count()
}

/// Insert all tasks (legacy behaviour) and return count of newly inserted rows.
fn save_all(db: &Database, tasks: Vec<ExternalTask>, source: &str) -> usize {
    tasks
        .into_iter()
        .filter_map(|et| db.save_external_task(to_db_task(et, source)).ok())
        .count()
}

// ── Preview commands ──────────────────────────────────────────────────────

/// Fetch GitHub issues without importing them. Returns a list annotated with
/// `already_imported: true` for any issue already present in the local DB.
#[tauri::command]
pub async fn preview_sync_github(
    db_state: State<'_, Arc<Mutex<Database>>>,
) -> Result<Vec<ExternalTask>, String> {
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
    Ok(mark_imported(&db, tasks, "GitHub"))
}

/// Fetch Jira issues without importing them.
#[tauri::command]
pub async fn preview_sync_jira(
    db_state: State<'_, Arc<Mutex<Database>>>,
) -> Result<Vec<ExternalTask>, String> {
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
    Ok(mark_imported(&db, tasks, "Jira"))
}

/// Fetch GitLab issues without importing them.
#[tauri::command]
pub async fn preview_sync_gitlab(
    db_state: State<'_, Arc<Mutex<Database>>>,
) -> Result<Vec<ExternalTask>, String> {
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
    Ok(mark_imported(&db, tasks, "GitLab"))
}

// ── Selective import commands ─────────────────────────────────────────────

/// Import only the issues whose external_ids are listed in `selected_ids`.
/// `source` must be "GitHub", "GitLab", or "Jira".
/// Returns the count of newly inserted tasks (duplicates are silently skipped).
#[tauri::command]
pub async fn import_selected(
    db_state:      State<'_, Arc<Mutex<Database>>>,
    source:        String,
    selected_ids:  Vec<String>,
    import_labels:   bool,
    import_projects: bool,
) -> Result<usize, String> {
    if selected_ids.is_empty() {
        return Ok(0);
    }

    // Re-fetch so we have the full task data to insert
    let tasks: Vec<ExternalTask> = match source.as_str() {
        "GitHub" => {
            let (token, repo) = {
                let db = db_state.lock().unwrap();
                (
                    db.get_setting("github_token").unwrap_or(None),
                    db.get_setting("github_repo").unwrap_or(None),
                )
            };
            let token = token.ok_or("GitHub token not configured.")?;
            crate::integrations::fetch_github_tasks(&token, repo.as_deref()).await?
        }
        "Jira" => {
            let (domain, email, token) = {
                let db = db_state.lock().unwrap();
                (
                    db.get_setting("jira_domain").unwrap_or(None),
                    db.get_setting("jira_email").unwrap_or(None),
                    db.get_setting("jira_token").unwrap_or(None),
                )
            };
            crate::integrations::fetch_jira_tasks(
                &domain.ok_or("Jira domain not configured.")?,
                &email.ok_or("Jira email not configured.")?,
                &token.ok_or("Jira token not configured.")?,
            ).await?
        }
        "GitLab" => {
            let (token, host) = {
                let db = db_state.lock().unwrap();
                (
                    db.get_setting("gitlab_token").unwrap_or(None),
                    db.get_setting("gitlab_host").unwrap_or(None),
                )
            };
            let token = token.ok_or("GitLab token not configured.")?;
            let host  = host.unwrap_or_else(|| "https://gitlab.com".to_string());
            crate::integrations::fetch_gitlab_tasks(&token, &host).await?
        }
        other => return Err(format!("Unknown source: {}", other)),
    };

    let db = db_state.lock().unwrap();

    // Optionally create tags from labels
    if import_labels {
        let all_labels: Vec<String> = tasks
            .iter()
            .filter(|t| selected_ids.contains(&t.id))
            .flat_map(|t| t.labels.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        for label in all_labels {
            // create_tag uses INSERT OR IGNORE so existing tags are safe
            let _ = db.create_tag(&label, None);
        }
    }

    Ok(save_by_ids(&db, tasks, &selected_ids, &source, import_projects))
}

// ── Legacy one-shot sync commands (kept for backwards compatibility) ───────

/// Sync GitHub issues — imports everything not already imported.
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

/// Sync Jira issues — imports everything not already imported.
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

/// Sync GitLab issues — imports everything not already imported.
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
