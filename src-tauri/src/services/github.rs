use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Release {
    pub tag_name: String,
    pub name: String,
    pub body: String,
    pub published_at: String,
    pub html_url: String,
    pub tarball_url: String,
    pub zipball_url: String,
}

pub async fn get_releases(owner: &str, repo: &str) -> Result<Vec<Release>, String> {
    let url = format!("https://api.github.com/repos/{}/{}/releases", owner, repo);
    
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "o-my-claw")
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("GitHub API error: {}", response.status()));
    }
    
    let releases: Vec<Release> = response.json().await.map_err(|e| format!("Parse error: {}", e))?;
    Ok(releases)
}

pub async fn get_release_by_tag(owner: &str, repo: &str, tag: &str) -> Result<Release, String> {
    let url = format!("https://api.github.com/repos/{}/{}/releases/tags/{}", owner, repo, tag);
    
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "o-my-claw")
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("GitHub API error: {}", response.status()));
    }
    
    let release: Release = response.json().await.map_err(|e| format!("Parse error: {}", e))?;
    Ok(release)
}
