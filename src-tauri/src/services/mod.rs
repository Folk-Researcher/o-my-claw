//! 服务模块
//! 
//! 这个模块包含所有的业务逻辑服务。
//! 每个子模块提供特定的功能服务，被命令模块调用。

/// GitHub API 服务
/// 
/// 提供与 GitHub API 交互的功能，用于获取 OpenClaw 的 releases 信息
pub mod github;

/// Git 操作服务
/// 
/// 提供 Git 仓库操作功能，包括克隆仓库、检出标签等
pub mod git;

/// 进程管理服务
/// 
/// 提供进程管理功能，包括编译和运行 OpenClaw
pub mod process;
