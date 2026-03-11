//! 命令模块
//! 
//! 这个模块包含所有 Tauri 命令的定义。
//! 每个子模块对应一组相关的功能命令。

/// 版本管理命令
/// 
/// 提供 OpenClaw 版本管理相关的命令，包括：
/// - 获取 GitHub releases 列表
/// - 检出指定版本的源码
/// - 编译 OpenClaw
/// - 启动/停止 OpenClaw
/// - 查看进程日志
pub mod version_manager;
