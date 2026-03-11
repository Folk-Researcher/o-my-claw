//! 进程管理服务
//!
//! 提供进程管理功能，用于编译和运行 OpenClaw。
//! 支持 Node.js 项目的安装依赖、构建和启动。

use std::process::Stdio;
use tokio::process::{Child, Command};
use std::sync::Mutex;
use std::collections::VecDeque;
use std::path::PathBuf;

/// 进程管理器
///
/// 管理 OpenClaw 的运行进程和日志缓冲区
pub struct ProcessManager {
    /// 当前运行的进程（如果有）
    pub current_process: Mutex<Option<Child>>,
    /// 日志缓冲区，用于存储进程输出
    pub log_buffer: Mutex<VecDeque<String>>,
    /// Node.js 路径（如果已下载）
    pub node_path: Mutex<Option<PathBuf>>,
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
            node_path: Mutex::new(None),
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

    /// 获取 Node.js 可执行文件路径
    ///
    /// 优先返回已下载的 Node.js 路径，如果没有则返回 "node"
    ///
    /// # 返回值
    /// Node.js 可执行文件路径或命令名
    pub fn get_node_path(&self) -> String {
        let node_path = self.node_path.lock().unwrap();
        node_path.as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "node".to_string())
    }

    /// 获取 npm 可执行文件路径
    ///
    /// 优先返回已下载的 npm 路径，如果没有则返回 "npm"
    ///
    /// # 返回值
    /// npm 可执行文件路径或命令名
    pub fn get_npm_path(&self) -> String {
        let node_path = self.node_path.lock().unwrap();
        node_path.as_ref()
            .map(|p| {
                let npm_path = p.parent()
                    .map(|parent| parent.join("npm.cmd"))
                    .unwrap_or_else(|| p.clone());
                npm_path.to_string_lossy().to_string()
            })
            .unwrap_or_else(|| "npm".to_string())
    }

    /// 设置 Node.js 路径
    ///
    /// # 参数
    /// - `path`: Node.js 可执行文件的路径
    pub fn set_node_path(&self, path: PathBuf) {
        let mut node_path = self.node_path.lock().unwrap();
        *node_path = Some(path);
    }
}

/// Node.js 版本配置
const NODE_VERSION: &str = "24.0.0";

/// 中国镜像站下载地址
const NODE_DOWNLOAD_URL_CN: &str = "https://registry.npmmirror.com/-/binary/node/v24.0.0/node-v24.0.0-win-x64.zip";

/// 获取 Node.js 安装目录路径
///
/// # 返回值
/// Node.js 安装目录的 PathBuf
fn get_node_install_dir() -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push("o-my-claw-node");
    path
}

/// 获取 Node.js 可执行文件路径
///
/// # 返回值
/// Node.js 可执行文件的 PathBuf
fn get_node_exe_path() -> PathBuf {
    let mut path = get_node_install_dir();
    path.push(format!("node-v{}-win-x64", NODE_VERSION));
    path.push("node.exe");
    path
}

/// 检查系统是否已安装 Node.js
///
/// # 返回值
/// - `Ok((true, version))`: 已安装，返回版本号
/// - `Ok((false, _))`: 未安装
/// - `Err(String)`: 检查失败
async fn check_system_node() -> Result<(bool, String), String> {
    let output = Command::new("node")
        .arg("--version")
        .output()
        .await;

    match output {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            Ok((true, version.trim().to_string()))
        }
        _ => Ok((false, String::new())),
    }
}

/// 检查已下载的 Node.js 是否可用
///
/// # 返回值
/// - `Ok((true, path))`: 可用，返回路径
/// - `Ok((false, _))`: 不可用
async fn check_downloaded_node() -> Result<(bool, PathBuf), String> {
    let node_exe = get_node_exe_path();
    if !node_exe.exists() {
        return Ok((false, node_exe));
    }

    // 验证 Node.js 是否可以运行
    let output = Command::new(&node_exe)
        .arg("--version")
        .output()
        .await;

    match output {
        Ok(output) if output.status.success() => Ok((true, node_exe)),
        _ => Ok((false, node_exe)),
    }
}

/// 下载并安装 Node.js
///
/// 从中国镜像站下载 Node.js 并解压到临时目录
///
/// # 参数
/// - `manager`: 进程管理器实例，用于记录日志
///
/// # 返回值
/// - `Ok(PathBuf)`: 成功，返回 Node.js 可执行文件路径
/// - `Err(String)`: 失败，返回错误信息
async fn download_nodejs(manager: &ProcessManager) -> Result<PathBuf, String> {
    manager.add_log("Downloading Node.js from China mirror...".to_string());

    let install_dir = get_node_install_dir();
    let node_exe = get_node_exe_path();

    // 如果已经存在，直接返回
    if node_exe.exists() {
        manager.add_log("Node.js already downloaded, reusing...".to_string());
        return Ok(node_exe);
    }

    // 确保安装目录存在
    std::fs::create_dir_all(&install_dir)
        .map_err(|e| format!("Failed to create install directory: {}", e))?;

    manager.add_log(format!("Downloading Node.js v{} from npmmirror...", NODE_VERSION));

    // 使用 PowerShell 下载并解压
    let download_script = format!(
        r#"
        $url = "{}"
        $output = "{}\node.zip"
        Write-Host "Downloading from $url..."
        Invoke-WebRequest -Uri $url -OutFile $output -UseBasicParsing
        Write-Host "Extracting to {}..."
        Expand-Archive -Path $output -DestinationPath "{}" -Force
        Remove-Item $output
        Write-Host "Download and extraction completed!"
        "#,
        NODE_DOWNLOAD_URL_CN,
        install_dir.to_string_lossy(),
        install_dir.to_string_lossy(),
        install_dir.to_string_lossy()
    );

    let mut download_child = Command::new("powershell")
        .args(&["-ExecutionPolicy", "Bypass", "-Command", &download_script])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start download: {}", e))?;

    // 读取输出并添加到日志
    if let Some(stdout) = download_child.stdout.take() {
        use tokio::io::{AsyncBufReadExt, BufReader};
        let reader = BufReader::new(stdout);
        let mut lines = reader.lines();
        while let Ok(Some(line)) = lines.next_line().await {
            manager.add_log(line);
        }
    }

    let download_status = download_child.wait().await
        .map_err(|e| format!("Download failed: {}", e))?;

    if !download_status.success() {
        return Err("Failed to download Node.js from mirror".to_string());
    }

    // 验证下载的 Node.js
    if !node_exe.exists() {
        return Err("Node.js executable not found after download".to_string());
    }

    manager.add_log(format!("Node.js v{} downloaded successfully!", NODE_VERSION));
    manager.add_log(format!("Installed at: {}", node_exe.display()));

    Ok(node_exe)
}

/// 编译/安装 OpenClaw
///
/// 执行以下步骤：
/// 1. 检查 Node.js 和 npm 是否安装，如果没有则自动下载安装
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
    manager.add_log("Checking Node.js environment...".to_string());

    // 首先检查系统是否已安装 Node.js
    let (has_system_node, node_version) = check_system_node().await?;

    let (node_path, npm_path) = if has_system_node {
        manager.add_log(format!("Found system Node.js: {}", node_version));
        ("node".to_string(), "npm".to_string())
    } else {
        // 系统没有 Node.js，检查是否已下载
        manager.add_log("System Node.js not found. Checking downloaded Node.js...".to_string());

        let (has_downloaded, downloaded_path) = check_downloaded_node().await?;

        let node_exe = if has_downloaded {
            manager.add_log("Using previously downloaded Node.js".to_string());
            downloaded_path
        } else {
            // 下载 Node.js
            download_nodejs(manager).await?
        };

        // 获取 npm 路径（与 node.exe 在同一目录）
        let npm_exe = node_exe.parent()
            .map(|p| p.join("npm.cmd"))
            .ok_or("Failed to get npm path")?;

        // 保存路径到管理器
        manager.set_node_path(node_exe.clone());

        // 验证版本
        let version_output = Command::new(&node_exe)
            .arg("--version")
            .output()
            .await
            .map_err(|e| format!("Failed to check downloaded Node.js: {}", e))?;

        let version = String::from_utf8_lossy(&version_output.stdout);
        manager.add_log(format!("Using downloaded Node.js: {}", version.trim()));

        (node_exe.to_string_lossy().to_string(), npm_exe.to_string_lossy().to_string())
    };

    manager.add_log(format!("Node path: {}", node_path));
    manager.add_log(format!("npm path: {}", npm_path));

    // 检查 npm 是否可用
    let npm_check = Command::new(&npm_path)
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
    let mut install_child = Command::new(&npm_path)
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
    let build_child = Command::new(&npm_path)
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

    // 获取 npm 路径（优先使用已下载的）
    let npm_path = manager.get_npm_path();
    manager.add_log(format!("Using npm: {}", npm_path));

    // 启动 OpenClaw (Node.js 项目)
    // 使用 npm start 启动应用
    let child = Command::new(&npm_path)
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
