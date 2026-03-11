//! O My Claw - OpenClaw 桌面管理器
//! 
//! 这是 Tauri 应用的入口文件，负责初始化应用程序并注册所有命令。

// 在非调试构建时隐藏 Windows 控制台窗口
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// 引入命令模块和服务模块
mod commands;
mod services;

use services::process::ProcessManager;
use std::sync::Arc;

/// 应用程序入口点
/// 
/// 初始化进程管理器并启动 Tauri 应用程序
fn main() {
  // 创建进程管理器实例，用于管理 OpenClaw 的编译和运行进程
  // 使用 Arc 包装以便在多个命令之间共享
  let process_manager = Arc::new(ProcessManager::new());

  // 构建 Tauri 应用程序
  tauri::Builder::default()
    // 将进程管理器添加到 Tauri 状态管理中
    // 这样可以在命令处理器中访问它
    .manage(process_manager)
    // 注册所有可用的命令
    .invoke_handler(tauri::generate_handler![
      // 版本管理命令
      commands::version_manager::list_releases,
      commands::version_manager::checkout_version,
      commands::version_manager::compile_version,
      commands::version_manager::start_openclaw,
      commands::version_manager::stop_openclaw,
      commands::version_manager::get_process_logs,
      commands::version_manager::is_openclaw_running,
      commands::version_manager::check_local_version,
    ])
    // 运行应用程序
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
