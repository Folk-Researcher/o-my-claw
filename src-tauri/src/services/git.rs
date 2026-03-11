//! Git 操作服务
//!
//! 提供 Git 仓库操作功能，包括克隆仓库、检出标签等。

use git2::{Repository, FetchOptions};
use std::path::Path;

/// 克隆 Git 仓库到指定路径
///
/// # 参数
/// - `url`: 仓库的 Git URL
/// - `path`: 本地存放路径
///
/// # 返回值
/// - `Ok(Repository)`: 成功时返回仓库对象
/// - `Err(String)`: 失败时返回错误信息
///
/// # 示例
/// ```rust
/// let repo = clone_repository("https://github.com/openclaw/openclaw.git", "/path/to/clone")?;
/// ```
pub fn clone_repository(url: &str, path: &str) -> Result<Repository, String> {
    // 使用 git2 库克隆仓库
    let repo = Repository::clone(url, path)
        .map_err(|e| format!("Failed to clone repository: {}", e))?;
    Ok(repo)
}

/// 检出指定标签到工作目录
///
/// 将仓库切换到指定标签对应的提交，使用 detached HEAD 模式
///
/// # 参数
/// - `repo_path`: 仓库本地路径
/// - `tag`: 要检出的标签名，例如 "v1.0.0"
///
/// # 返回值
/// - `Ok(())`: 成功
/// - `Err(String)`: 失败时返回错误信息
///
/// # 注意
/// 这会进入 detached HEAD 状态，适合用于查看特定版本代码
pub fn checkout_tag(repo_path: &str, tag: &str) -> Result<(), String> {
    // 打开仓库
    let repo = Repository::open(repo_path)
        .map_err(|e| format!("Failed to open repository: {}", e))?;
    
    // 获取 tag 对应的引用
    let reference = repo.find_reference(&format!("refs/tags/{}", tag))
        .map_err(|e| format!("Tag not found: {}", e))?;
    
    // 获取 tag 指向的提交
    let commit = reference.peel_to_commit()
        .map_err(|e| format!("Failed to get commit: {}", e))?;
    
    // 设置 HEAD 为 detached 状态，指向该提交
    repo.set_head_detached(commit.id())
        .map_err(|e| format!("Failed to checkout: {}", e))?;
    
    // 强制检出文件到工作目录
    let mut checkout_opts = git2::build::CheckoutBuilder::new();
    checkout_opts.force();
    
    repo.checkout_head(Some(&mut checkout_opts))
        .map_err(|e| format!("Failed to checkout files: {}", e))?;
    
    Ok(())
}

/// 拉取远程仓库更新
///
/// 从 origin 远程拉取最新提交，但不合并到当前分支
///
/// # 参数
/// - `repo_path`: 仓库本地路径
///
/// # 返回值
/// - `Ok(())`: 成功
/// - `Err(String)`: 失败时返回错误信息
#[allow(dead_code)]
pub fn pull_updates(repo_path: &str) -> Result<(), String> {
    // 打开仓库
    let repo = Repository::open(repo_path)
        .map_err(|e| format!("Failed to open repository: {}", e))?;
    
    // 获取 origin 远程
    let mut remote = repo.find_remote("origin")
        .map_err(|e| format!("Failed to find remote: {}", e))?;
    
    // 获取 fetch 选项
    let mut fetch_opts = FetchOptions::new();
    
    // 执行 fetch，拉取远程更新
    remote.fetch(&[] as &[&str], Some(&mut fetch_opts), None)
        .map_err(|e| format!("Failed to fetch: {}", e))?;
    
    Ok(())
}

/// 检查指定路径是否为 Git 仓库
///
/// # 参数
/// - `path`: 要检查的路径
///
/// # 返回值
/// - `true`: 是 Git 仓库（存在 .git 目录）
/// - `false`: 不是 Git 仓库
pub fn repository_exists(path: &str) -> bool {
    Path::new(&format!("{}/.git", path)).exists()
}
