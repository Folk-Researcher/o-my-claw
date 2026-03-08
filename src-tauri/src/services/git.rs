use git2::{Repository, FetchOptions};
use std::path::Path;

pub fn clone_repository(url: &str, path: &str) -> Result<Repository, String> {
    let repo = Repository::clone(url, path)
        .map_err(|e| format!("Failed to clone repository: {}", e))?;
    Ok(repo)
}

pub fn checkout_tag(repo_path: &str, tag: &str) -> Result<(), String> {
    let repo = Repository::open(repo_path)
        .map_err(|e| format!("Failed to open repository: {}", e))?;
    
    // 获取 tag 对应的 commit
    let reference = repo.find_reference(&format!("refs/tags/{}", tag))
        .map_err(|e| format!("Tag not found: {}", e))?;
    
    let commit = reference.peel_to_commit()
        .map_err(|e| format!("Failed to get commit: {}", e))?;
    
    // 检出到 detached HEAD 状态
    repo.set_head_detached(commit.id())
        .map_err(|e| format!("Failed to checkout: {}", e))?;
    
    // 重置工作目录
    let mut checkout_opts = git2::build::CheckoutBuilder::new();
    checkout_opts.force();
    
    repo.checkout_head(Some(&mut checkout_opts))
        .map_err(|e| format!("Failed to checkout files: {}", e))?;
    
    Ok(())
}

pub fn pull_updates(repo_path: &str) -> Result<(), String> {
    let repo = Repository::open(repo_path)
        .map_err(|e| format!("Failed to open repository: {}", e))?;
    
    let mut remote = repo.find_remote("origin")
        .map_err(|e| format!("Failed to find remote: {}", e))?;
    
    let mut fetch_opts = FetchOptions::new();
    remote.fetch(&[] as &[&str], Some(&mut fetch_opts), None)
        .map_err(|e| format!("Failed to fetch: {}", e))?;
    
    Ok(())
}

pub fn repository_exists(path: &str) -> bool {
    Path::new(&format!("{}/.git", path)).exists()
}
