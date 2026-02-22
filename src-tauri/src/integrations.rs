use serde::{Deserialize, Serialize};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT};

/// A task fetched from an external platform before being imported.
/// `already_imported` is set by the command layer (not here).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExternalTask {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub url: String,
    /// Labels / tags associated with the issue on the platform.
    pub labels: Vec<String>,
    /// Project / repo name the issue belongs to.
    pub project: Option<String>,
    /// Set to true by the command layer if this external_id is already in the DB.
    #[serde(default)]
    pub already_imported: bool,
}

// ── GitHub ────────────────────────────────────────────────────────────────

pub async fn fetch_github_tasks(token: &str, repo: Option<&str>) -> Result<Vec<ExternalTask>, String> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("code-chrono"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", token)).map_err(|e| e.to_string())?,
    );
    headers.insert("Accept", HeaderValue::from_static("application/vnd.github+json"));
    headers.insert("X-GitHub-Api-Version", HeaderValue::from_static("2022-11-28"));

    let url = if let Some(r) = repo.filter(|s| !s.is_empty() && s.contains('/')) {
        format!("https://api.github.com/repos/{}/issues", r)
    } else {
        "https://api.github.com/issues".to_string()
    };

    let res = client
        .get(&url)
        .headers(headers)
        .query(&[("filter", "assigned"), ("state", "open"), ("per_page", "100")])
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        let status = res.status();
        let body = res.text().await.unwrap_or_default();
        return Err(format!("GitHub API error ({}): {}", status, body));
    }

    let issues: Vec<serde_json::Value> = res.json().await.map_err(|e| e.to_string())?;

    let tasks = issues
        .into_iter()
        .filter_map(|issue| {
            // Skip pull requests
            if issue.get("pull_request").is_some() {
                return None;
            }
            let labels: Vec<String> = issue["labels"]
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .filter_map(|l| l["name"].as_str().map(|s| s.to_string()))
                .collect();

            let project = issue["repository"]
                .get("full_name")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .or_else(|| {
                    // derive from html_url: github.com/owner/repo/issues/N
                    issue["html_url"].as_str().and_then(|u| {
                        let parts: Vec<&str> = u.split('/').collect();
                        if parts.len() >= 5 {
                            Some(format!("{}/{}", parts[3], parts[4]))
                        } else {
                            None
                        }
                    })
                });

            Some(ExternalTask {
                id: issue["id"].as_i64().unwrap_or(0).to_string(),
                title: issue["title"].as_str().unwrap_or("No Title").to_string(),
                description: issue["body"].as_str().map(|s| {
                    if s.len() > 500 { format!("{}…", &s[..500]) } else { s.to_string() }
                }),
                status: "todo".to_string(),
                url: issue["html_url"].as_str().unwrap_or("").to_string(),
                labels,
                project,
                already_imported: false,
            })
        })
        .collect();

    Ok(tasks)
}

// ── Jira ──────────────────────────────────────────────────────────────────

pub async fn fetch_jira_tasks(domain: &str, email: &str, token: &str) -> Result<Vec<ExternalTask>, String> {
    let client = reqwest::Client::new();
    let auth = b64_encode(&format!("{}:{}", email, token));
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Basic {}", auth)).map_err(|e| e.to_string())?,
    );
    headers.insert("Accept", HeaderValue::from_static("application/json"));

    let clean_domain = domain.trim()
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_end_matches('/');

    let url = format!("https://{}/rest/api/3/search", clean_domain);

    let res = client
        .get(url)
        .headers(headers)
        .query(&[
            ("jql", "assignee = currentUser() AND statusCategory != Done ORDER BY updated DESC"),
            ("maxResults", "100"),
            ("fields", "summary,description,status,priority,labels,project"),
        ])
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !res.status().is_success() {
        let status = res.status();
        let body = res.text().await.unwrap_or_default();
        return Err(format!("Jira API error ({}): {}", status, body));
    }

    let data: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
    let issues = data["issues"].as_array().ok_or("Invalid Jira response: missing 'issues' field")?;

    let tasks = issues
        .iter()
        .map(|issue| {
            let key = issue["key"].as_str().unwrap_or("");
            let labels: Vec<String> = issue["fields"]["labels"]
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .filter_map(|l| l.as_str().map(|s| s.to_string()))
                .collect();

            let project = issue["fields"]["project"]["name"]
                .as_str()
                .map(|s| s.to_string());

            ExternalTask {
                id: format!("jira-{}", key),
                title: format!(
                    "[{}] {}",
                    key,
                    issue["fields"]["summary"].as_str().unwrap_or("No Title")
                ),
                description: None,
                status: issue["fields"]["status"]["statusCategory"]["name"]
                    .as_str()
                    .unwrap_or("todo")
                    .to_string(),
                url: format!("https://{}/browse/{}", clean_domain, key),
                labels,
                project,
                already_imported: false,
            }
        })
        .collect();

    Ok(tasks)
}

fn b64_encode(s: &str) -> String {
    use base64::{Engine as _, engine::general_purpose};
    general_purpose::STANDARD.encode(s)
}

// ── GitLab ────────────────────────────────────────────────────────────────

pub async fn fetch_gitlab_tasks(token: &str, host: &str) -> Result<Vec<ExternalTask>, String> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("code-chrono"));
    headers.insert(
        "PRIVATE-TOKEN",
        HeaderValue::from_str(token).map_err(|e| e.to_string())?,
    );

    let mut base_url = host.trim().trim_end_matches('/').to_string();
    if !base_url.starts_with("http://") && !base_url.starts_with("https://") {
        base_url = format!("https://{}", base_url);
    }

    let url = format!("{}/api/v4/issues", base_url);

    let res = client
        .get(url)
        .headers(headers)
        .query(&[
            ("scope", "assigned_to_me"),
            ("state", "opened"),
            ("per_page", "100"),
            ("with_labels_details", "true"),
        ])
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = res.status();
    if !status.is_success() {
        let body = res.text().await.unwrap_or_else(|_| "Could not read error body".to_string());
        return Err(format!("GitLab API error ({}): {}", status, body));
    }

    let issues: Vec<serde_json::Value> = res.json().await.map_err(|e| {
        format!("Failed to parse GitLab response: {}. Make sure your token and host are correct.", e)
    })?;

    let tasks = issues
        .into_iter()
        .map(|issue| {
            let labels: Vec<String> = issue["labels"]
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .filter_map(|l| {
                    // with_labels_details gives objects; without gives strings
                    l["name"].as_str()
                        .or_else(|| l.as_str())
                        .map(|s| s.to_string())
                })
                .collect();

            let project = issue["references"]["full"]
                .as_str()
                .and_then(|r| r.split('#').next())
                .map(|s| s.trim_start_matches('/').to_string());

            ExternalTask {
                id: format!("gl-{}", issue["id"].as_i64().unwrap_or(0)),
                title: issue["title"].as_str().unwrap_or("No Title").to_string(),
                description: issue["description"].as_str().map(|s| {
                    if s.len() > 500 { format!("{}…", &s[..500]) } else { s.to_string() }
                }),
                status: issue["state"].as_str().unwrap_or("opened").to_string(),
                url: issue["web_url"].as_str().unwrap_or("").to_string(),
                labels,
                project,
                already_imported: false,
            }
        })
        .collect();

    Ok(tasks)
}
