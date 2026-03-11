//! GitHub API 服务
//!
//! 提供与 GitHub API 交互的功能，用于获取 OpenClaw 项目的 releases 信息。

use serde::{Deserialize, Serialize};

/// GitHub Release 信息结构体
///
/// 包含从 GitHub API 获取的 release 详细信息
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Release {
    /// 版本标签名，例如 "v1.0.0"
    pub tag_name: String,
    /// 版本名称
    pub name: String,
    /// 版本说明（Markdown 格式）
    pub body: String,
    /// 发布日期（ISO 8601 格式）
    pub published_at: String,
    /// GitHub 上的 release 页面 URL
    pub html_url: String,
    /// 源码压缩包（tar.gz）下载链接
    pub tarball_url: String,
    /// 源码压缩包（zip）下载链接
    pub zipball_url: String,
}

/// 获取 GitHub 仓库的所有 releases
///
/// # 参数
/// - `owner`: 仓库所有者用户名
/// - `repo`: 仓库名称
///
/// # 返回值
/// - `Ok(Vec<Release>)`: 成功时返回 release 列表
/// - `Err(String)`: 失败时返回错误信息
///
/// # 示例
/// ```rust
/// let releases = get_releases("openclaw", "openclaw").await?;
/// ```
pub async fn get_releases(owner: &str, repo: &str) -> Result<Vec<Release>, String> {
    // 构建 GitHub API URL
    let url = format!("https://api.github.com/repos/{}/{}/releases", owner, repo);
    
    // 创建 HTTP 客户端
    let client = reqwest::Client::new();
    
    // 发送 GET 请求，设置 User-Agent 头（GitHub API 要求）
    let response = client
        .get(&url)
        .header("User-Agent", "o-my-claw")
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;
    
    // 检查响应状态
    if !response.status().is_success() {
        return Err(format!("GitHub API error: {}", response.status()));
    }
    
    // 解析 JSON 响应
    let releases: Vec<Release> = response.json().await.map_err(|e| format!("Parse error: {}", e))?;
    Ok(releases)
}

/// 获取指定标签的 release 信息
///
/// # 参数
/// - `owner`: 仓库所有者用户名
/// - `repo`: 仓库名称
/// - `tag`: 版本标签名，例如 "v1.0.0"
///
/// # 返回值
/// - `Ok(Release)`: 成功时返回 release 信息
/// - `Err(String)`: 失败时返回错误信息
#[allow(dead_code)]
pub async fn get_release_by_tag(owner: &str, repo: &str, tag: &str) -> Result<Release, String> {
    // 构建 GitHub API URL
    let url = format!("https://api.github.com/repos/{}/{}/releases/tags/{}", owner, repo, tag);
    
    // 创建 HTTP 客户端
    let client = reqwest::Client::new();
    
    // 发送 GET 请求
    let response = client
        .get(&url)
        .header("User-Agent", "o-my-claw")
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;
    
    // 检查响应状态
    if !response.status().is_success() {
        return Err(format!("GitHub API error: {}", response.status()));
    }
    
    // 解析 JSON 响应
    let release: Release = response.json().await.map_err(|e| format!("Parse error: {}", e))?;
    Ok(release)
}
