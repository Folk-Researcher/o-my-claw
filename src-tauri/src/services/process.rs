use std::process::Stdio;
use tokio::process::{Child, Command};
use std::sync::Mutex;
use std::collections::VecDeque;

pub struct ProcessManager {
    pub current_process: Mutex<Option<Child>>,
    pub log_buffer: Mutex<VecDeque<String>>,
}

impl ProcessManager {
    pub fn new() -> Self {
        ProcessManager {
            current_process: Mutex::new(None),
            log_buffer: Mutex::new(VecDeque::with_capacity(1000)),
        }
    }

    pub fn add_log(&self, line: String) {
        let mut buffer = self.log_buffer.lock().unwrap();
        buffer.push_back(line);
        if buffer.len() > 1000 {
            buffer.pop_front();
        }
    }

    pub fn get_logs(&self) -> Vec<String> {
        let buffer = self.log_buffer.lock().unwrap();
        buffer.iter().cloned().collect()
    }

    pub fn clear_logs(&self) {
        let mut buffer = self.log_buffer.lock().unwrap();
        buffer.clear();
    }
}

pub async fn compile_openclaw(
    source_path: &str,
    manager: &ProcessManager,
) -> Result<(), String> {
    manager.clear_logs();
    manager.add_log("Starting compilation...".to_string());

    // 这里简化处理，实际应该根据 OpenClaw 的构建方式调整
    // 假设 OpenClaw 使用 cargo 构建
    let mut child = Command::new("cargo")
        .args(&["build", "--release"])
        .current_dir(source_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start compilation: {}", e))?;

    // 简化：等待进程完成
    let status = child.wait().await
        .map_err(|e| format!("Compilation failed: {}", e))?;

    if status.success() {
        manager.add_log("Compilation completed successfully!".to_string());
        Ok(())
    } else {
        Err("Compilation failed".to_string())
    }
}

pub async fn start_openclaw(
    source_path: &str,
    manager: &ProcessManager,
) -> Result<(), String> {
    // 检查是否已有进程在运行
    {
        let mut current = manager.current_process.lock().unwrap();
        if current.is_some() {
            return Err("OpenClaw is already running".to_string());
        }
    }

    manager.add_log("Starting OpenClaw...".to_string());

    // 启动 OpenClaw（路径需要根据实际构建输出调整）
    let child = Command::new(format!("{}/target/release/openclaw", source_path))
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

pub async fn stop_openclaw(manager: &ProcessManager) -> Result<(), String> {
    let mut current = manager.current_process.lock().unwrap();
    
    if let Some(mut child) = current.take() {
        child.kill().await
            .map_err(|e| format!("Failed to stop OpenClaw: {}", e))?;
        manager.add_log("OpenClaw stopped".to_string());
        Ok(())
    } else {
        Err("OpenClaw is not running".to_string())
    }
}

pub fn is_running(manager: &ProcessManager) -> bool {
    let current = manager.current_process.lock().unwrap();
    current.is_some()
}
