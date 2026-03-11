use crate::services::github::{self, Release};
use crate::services::git;
use crate::services::process::{self, ProcessManager};
use std::sync::Arc;
use tauri::State;

// GitHub 仓库配置
const GITHUB_OWNER: &str = "openclaw";
const GITHUB_REPO: &str = "openclaw";
const SOURCE_DIR: &str = "openclaw_src";

// 获取 releases 列表
#[tauri::command]
pub async fn list_releases() -> Result<Vec<Release>, String> {
    github::get_releases(GITHUB_OWNER, GITHUB_REPO).await
}

// 检出指定版本
#[tauri::command]
pub async fn checkout_version(tag: String) -> Result<(), String> {
    let source_path = format!("{}/{}", std::env::temp_dir().to_string_lossy(), SOURCE_DIR);
    
    // 检查仓库是否已存在
    if git::repository_exists(&source_path) {
        // 已存在，直接检出 tag
        git::checkout_tag(&source_path, &tag)
    } else {
        // 不存在，先克隆仓库
        let repo_url = format!("https://github.com/{}/{}.git", GITHUB_OWNER, GITHUB_REPO);
        git::clone_repository(&repo_url, &source_path)
            .map_err(|e| format!("Failed to clone: {}", e))?;
        
        // 检出指定 tag
        git::checkout_tag(&source_path, &tag)
    }
}

// 编译指定版本
#[tauri::command]
pub async fn compile_version(
    tag: String,
    process_manager: State<'_, Arc<ProcessManager>>,
) -> Result<(), String> {
    let source_path = format!("{}/{}", std::env::temp_dir().to_string_lossy(), SOURCE_DIR);
    
    // 确保已检出正确版本
    git::checkout_tag(&source_path, &tag)?;
    
    // 编译
    process::compile_openclaw(&source_path, &process_manager).await
}

// 启动 OpenClaw
#[tauri::command]
pub async fn start_openclaw(
    tag: String,
    process_manager: State<'_, Arc<ProcessManager>>,
) -> Result<(), String> {
    let source_path = format!("{}/{}", std::env::temp_dir().to_string_lossy(), SOURCE_DIR);
    
    // 确保已检出正确版本
    git::checkout_tag(&source_path, &tag)?;
    
    // 启动
    process::start_openclaw(&source_path, &process_manager).await
}

// 停止 OpenClaw
#[tauri::command]
pub async fn stop_openclaw(
    process_manager: State<'_, Arc<ProcessManager>>,
) -> Result<(), String> {
    process::stop_openclaw(&process_manager).await
}

// 获取进程日志
#[tauri::command]
pub fn get_process_logs(process_manager: State<'_, Arc<ProcessManager>>) -> Vec<String> {
    process_manager.get_logs()
}

// 检查是否正在运行
#[tauri::command]
pub fn is_openclaw_running(process_manager: State<'_, Arc<ProcessManager>>) -> bool {
    process::is_running(&process_manager)
}

// 检查本地版本状态
#[tauri::command]
pub fn check_local_version(_tag: String) -> Result<String, String> {
    let source_path = format!("{}/{}", std::env::temp_dir().to_string_lossy(), SOURCE_DIR);
    
    if !git::repository_exists(&source_path) {
        return Ok("not_cloned".to_string());
    }
    
    // 检查是否已编译（简化检查）
    let executable_path = format!("{}/target/release/openclaw", source_path);
    if std::path::Path::new(&executable_path).exists() {
        Ok("compiled".to_string())
    } else {
        Ok("cloned".to_string())
    }
}
