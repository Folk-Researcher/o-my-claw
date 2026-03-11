//! 版本管理命令
//!
//! 提供 OpenClaw 版本管理相关的 Tauri 命令。
//! 包括获取 releases、检出版本、编译和运行等功能。

use crate::services::github::{self, Release};
use crate::services::git;
use crate::services::process::{self, ProcessManager};
use std::sync::Arc;
use tauri::State;

/// GitHub 仓库配置
/// OpenClaw 的 GitHub 仓库信息
const GITHUB_OWNER: &str = "openclaw";
const GITHUB_REPO: &str = "openclaw";

/// 源码存放目录名
const SOURCE_DIR: &str = "openclaw_src";

/// 获取 OpenClaw 的所有 releases 列表
///
/// # 返回值
/// - `Ok(Vec<Release>)`: 成功时返回 release 列表
/// - `Err(String)`: 失败时返回错误信息
///
/// # 示例
/// 前端调用：
/// ```javascript
/// const releases = await invoke('list_releases');
/// ```
#[tauri::command]
pub async fn list_releases() -> Result<Vec<Release>, String> {
    github::get_releases(GITHUB_OWNER, GITHUB_REPO).await
}

/// 检出指定版本的源码
///
/// 如果本地没有仓库，会先克隆；如果已有仓库，直接检出指定 tag
///
/// # 参数
/// - `tag`: 要检出的版本标签，例如 "v1.0.0"
///
/// # 返回值
/// - `Ok(())`: 检出成功
/// - `Err(String)`: 检出失败，返回错误信息
#[tauri::command]
pub async fn checkout_version(tag: String) -> Result<(), String> {
    // 构建源码存放路径（系统临时目录下）
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

/// 编译指定版本的 OpenClaw
///
/// 先检出正确版本，然后执行编译/安装流程
///
/// # 参数
/// - `tag`: 要编译的版本标签
/// - `process_manager`: 进程管理器状态（由 Tauri 自动注入）
///
/// # 返回值
/// - `Ok(())`: 编译成功
/// - `Err(String)`: 编译失败，返回错误信息
#[tauri::command]
pub async fn compile_version(
    tag: String,
    process_manager: State<'_, Arc<ProcessManager>>,
) -> Result<(), String> {
    let source_path = format!("{}/{}", std::env::temp_dir().to_string_lossy(), SOURCE_DIR);
    
    // 确保已检出正确版本
    git::checkout_tag(&source_path, &tag)?;
    
    // 执行编译
    process::compile_openclaw(&source_path, &process_manager).await
}

/// 启动 OpenClaw
///
/// 先检出正确版本，然后启动应用
///
/// # 参数
/// - `tag`: 要启动的版本标签
/// - `process_manager`: 进程管理器状态（由 Tauri 自动注入）
///
/// # 返回值
/// - `Ok(())`: 启动成功
/// - `Err(String)`: 启动失败，返回错误信息
#[tauri::command]
pub async fn start_openclaw(
    tag: String,
    process_manager: State<'_, Arc<ProcessManager>>,
) -> Result<(), String> {
    let source_path = format!("{}/{}", std::env::temp_dir().to_string_lossy(), SOURCE_DIR);
    
    // 确保已检出正确版本
    git::checkout_tag(&source_path, &tag)?;
    
    // 启动应用
    process::start_openclaw(&source_path, &process_manager).await
}

/// 停止 OpenClaw
///
/// 终止当前运行的 OpenClaw 进程
///
/// # 参数
/// - `process_manager`: 进程管理器状态（由 Tauri 自动注入）
///
/// # 返回值
/// - `Ok(())`: 停止成功
/// - `Err(String)`: 停止失败，返回错误信息
#[tauri::command]
pub async fn stop_openclaw(
    process_manager: State<'_, Arc<ProcessManager>>,
) -> Result<(), String> {
    process::stop_openclaw(&process_manager).await
}

/// 获取进程日志
///
/// 获取 OpenClaw 编译和运行过程中的日志输出
///
/// # 参数
/// - `process_manager`: 进程管理器状态（由 Tauri 自动注入）
///
/// # 返回值
/// 返回日志字符串的向量
#[tauri::command]
pub fn get_process_logs(process_manager: State<'_, Arc<ProcessManager>>) -> Vec<String> {
    process_manager.get_logs()
}

/// 检查 OpenClaw 是否正在运行
///
/// # 参数
/// - `process_manager`: 进程管理器状态（由 Tauri 自动注入）
///
/// # 返回值
/// - `true`: OpenClaw 正在运行
/// - `false`: OpenClaw 没有运行
#[tauri::command]
pub fn is_openclaw_running(process_manager: State<'_, Arc<ProcessManager>>) -> bool {
    process::is_running(&process_manager)
}

/// 检查本地版本状态
///
/// 检查指定版本的 OpenClaw 在本地是什么状态
///
/// # 参数
/// - `tag`: 要检查的版本标签（当前未使用，但保留用于未来扩展）
///
/// # 返回值
/// - `"not_cloned"`: 未下载
/// - `"cloned"`: 已下载但未编译
/// - `"compiled"`: 已编译
/// - `Err(String)`: 检查失败
#[tauri::command]
pub fn check_local_version(_tag: String) -> Result<String, String> {
    let source_path = format!("{}/{}", std::env::temp_dir().to_string_lossy(), SOURCE_DIR);
    
    // 检查是否已克隆
    if !git::repository_exists(&source_path) {
        return Ok("not_cloned".to_string());
    }
    
    // 检查是否已编译（检查可执行文件是否存在）
    // 注意：这里假设编译后会生成特定的文件或目录
    // 实际路径可能需要根据 OpenClaw 的项目结构调整
    let executable_path = format!("{}/node_modules/.bin/openclaw", source_path);
    if std::path::Path::new(&executable_path).exists() {
        Ok("compiled".to_string())
    } else {
        Ok("cloned".to_string())
    }
}
