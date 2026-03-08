#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod services;

use services::process::ProcessManager;
use std::sync::Arc;

fn main() {
  // 创建进程管理器
  let process_manager = Arc::new(ProcessManager::new());

  tauri::Builder::default()
    .manage(process_manager)
    .invoke_handler(tauri::generate_handler![
      commands::greet::greet,
      commands::version_manager::list_releases,
      commands::version_manager::checkout_version,
      commands::version_manager::compile_version,
      commands::version_manager::start_openclaw,
      commands::version_manager::stop_openclaw,
      commands::version_manager::get_process_logs,
      commands::version_manager::is_openclaw_running,
      commands::version_manager::check_local_version,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
