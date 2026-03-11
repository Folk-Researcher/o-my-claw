//! 进程管理服务
//!
//! 提供进程管理功能，用于编译和运行 OpenClaw。
//! 支持 Node.js 项目的安装依赖、构建和启动。

use std::process::Stdio;
use tokio::process::{Child, Command};
use std::sync::Mutex;
use std::collections::VecDeque;

/// 进程管理器
///
/// 管理 OpenClaw 的运行进程和日志缓冲区
pub struct ProcessManager {
    /// 当前运行的进程（如果有）
    pub current_process: Mutex<Option<Child>>,
    /// 日志缓冲区，用于存储进程输出
    pub log_buffer: Mutex<VecDeque<String>>,
}

impl ProcessManager {
    /// 创建新的进程管理器实例
    ///
    /// # 返回值
    /// 返回初始化的 ProcessManager 实例
    pub fn new() -> Self {
        ProcessManager {
            current_process: Mutex::new(None),
            log_buffer: Mutex::new(VecDeque::with_capacity(1000)),
        }
    }

    /// 添加日志到缓冲区
    ///
    /// # 参数
    /// - `line`: 要添加的日志行
    ///
    /// # 说明
    /// 当日志超过 1000 行时，会自动移除最旧的日志
    pub fn add_log(&self, line: String) {
        let mut buffer = self.log_buffer.lock().unwrap();
        buffer.push_back(line);
        if buffer.len() > 1000 {
            buffer.pop_front();
        }
    }

    /// 获取所有日志
    ///
    /// # 返回值
    /// 返回日志字符串的向量
    pub fn get_logs(&self) -> Vec<String> {
        let buffer = self.log_buffer.lock().unwrap();
        buffer.iter().cloned().collect()
    }

    /// 清空日志缓冲区
    pub fn clear_logs(&self) {
        let mut buffer = self.log_buffer.lock().unwrap();
        buffer.clear();
    }
}

/// 编译/安装 OpenClaw
///
/// 执行以下步骤：
/// 1. 检查 Node.js 和 npm 是否安装
/// 2. 运行 `npm install` 安装依赖
/// 3. 尝试运行 `npm run build`（如果有构建脚本）
///
/// # 参数
/// - `source_path`: OpenClaw 源码路径
/// - `manager`: 进程管理器实例，用于记录日志
///
/// # 返回值
/// - `Ok(())`: 安装成功
/// - `Err(String)`: 安装失败，返回错误信息
pub async fn compile_openclaw(
    source_path: &str,
    manager: &ProcessManager,
) -> Result<(), String> {
    manager.clear_logs();
    manager.add_log("Installing Node.js dependencies...".to_string());

    // 检查 Node.js 是否安装
    let node_check = Command::new("node")
        .arg("--version")
        .output()
        .await
        .map_err(|e| format!("Node.js not found. Please install Node.js: {}", e))?;

    if !node_check.status.success() {
        return Err("Node.js is not installed or not accessible".to_string());
    }

    let node_version = String::from_utf8_lossy(&node_check.stdout);
    manager.add_log(format!("Found Node.js: {}", node_version.trim()));

    // 检查 npm 是否可用
    let npm_check = Command::new("npm")
        .arg("--version")
        .output()
        .await
        .map_err(|e| format!("npm not found: {}", e))?;

    if !npm_check.status.success() {
        return Err("npm is not accessible".to_string());
    }

    let npm_version = String::from_utf8_lossy(&npm_check.stdout);
    manager.add_log(format!("Found npm: {}", npm_version.trim()));

    // 安装依赖
    manager.add_log("Running npm install...".to_string());
    let mut install_child = Command::new("npm")
        .args(&["install"])
        .current_dir(source_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start npm install: {}", e))?;

    let install_status = install_child.wait().await
        .map_err(|e| format!("npm install failed: {}", e))?;

    if !install_status.success() {
        return Err("npm install failed".to_string());
    }

    manager.add_log("npm install completed!".to_string());

    // 如果有 build 脚本，执行构建
    manager.add_log("Checking for build script...".to_string());
    let build_child = Command::new("npm")
        .args(&["run", "build"])
        .current_dir(source_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    if let Ok(mut child) = build_child {
        let build_status = child.wait().await;
        if let Ok(status) = build_status {
            if status.success() {
                manager.add_log("Build completed!".to_string());
            } else {
                manager.add_log("Build script failed or not available (this is OK for some projects)".to_string());
            }
        }
    } else {
        manager.add_log("No build script found (this is OK for some projects)".to_string());
    }

    manager.add_log("Compilation/setup completed successfully!".to_string());
    Ok(())
}

/// 启动 OpenClaw
///
/// 使用 `npm start` 启动 OpenClaw 应用
///
/// # 参数
/// - `source_path`: OpenClaw 源码路径
/// - `manager`: 进程管理器实例
///
/// # 返回值
/// - `Ok(())`: 启动成功
/// - `Err(String)`: 启动失败，返回错误信息
///
/// # 注意
/// 如果 OpenClaw 已经在运行，会返回错误
pub async fn start_openclaw(
    source_path: &str,
    manager: &ProcessManager,
) -> Result<(), String> {
    // 检查是否已有进程在运行
    {
        let current = manager.current_process.lock().unwrap();
        if current.is_some() {
            return Err("OpenClaw is already running".to_string());
        }
    }

    manager.add_log("Starting OpenClaw...".to_string());

    // 启动 OpenClaw (Node.js 项目)
    // 使用 npm start 启动应用
    let child = Command::new("npm")
        .args(&["start"])
        .current_dir(source_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start OpenClaw: {}", e))?;

    {
        let mut current = manager.current_process.lock().unwrap();
        *current = Some(child);
    }

    manager.add_log("OpenClaw started!".to_string());
    Ok(())
}

/// 停止 OpenClaw
///
/// 终止当前运行的 OpenClaw 进程
///
/// # 参数
/// - `manager`: 进程管理器实例
///
/// # 返回值
/// - `Ok(())`: 停止成功
/// - `Err(String)`: 停止失败，返回错误信息
///
/// # 注意
/// 如果 OpenClaw 没有运行，会返回错误
pub async fn stop_openclaw(manager: &ProcessManager) -> Result<(), String> {
    // 先取出进程，释放锁
    let child_opt = {
        let mut current = manager.current_process.lock().unwrap();
        current.take()
    };
    
    if let Some(mut child) = child_opt {
        child.kill().await
            .map_err(|e| format!("Failed to stop OpenClaw: {}", e))?;
        manager.add_log("OpenClaw stopped".to_string());
        Ok(())
    } else {
        Err("OpenClaw is not running".to_string())
    }
}

/// 检查 OpenClaw 是否正在运行
///
/// # 参数
/// - `manager`: 进程管理器实例
///
/// # 返回值
/// - `true`: OpenClaw 正在运行
/// - `false`: OpenClaw 没有运行
pub fn is_running(manager: &ProcessManager) -> bool {
    let current = manager.current_process.lock().unwrap();
    current.is_some()
}
