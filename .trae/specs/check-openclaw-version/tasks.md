# Tasks

- [ ] Task 1: 创建 OpenClaw 版本检测服务
  - [ ] SubTask 1.1: 在 `src-tauri/src/services/` 创建 `openclaw.rs` 服务模块
  - [ ] SubTask 1.2: 实现 `check_openclaw_installation()` 函数，检测 OpenClaw 是否安装
  - [ ] SubTask 1.3: 实现 `get_openclaw_version()` 函数，获取 OpenClaw 版本号
  - [ ] SubTask 1.4: 实现 `get_openclaw_path()` 函数，获取 OpenClaw 安装路径
  - [ ] SubTask 1.5: 定义 `OpenClawVersionInfo` 结构体返回检测结果

- [ ] Task 2: 创建 Tauri 命令
  - [ ] SubTask 2.1: 在 `src-tauri/src/commands/` 创建 `version.rs` 命令模块
  - [ ] SubTask 2.2: 实现 `check_openclaw_version` 命令函数
  - [ ] SubTask 2.3: 在 `main.rs` 中注册新命令

- [ ] Task 3: 创建前端版本检查组件
  - [ ] SubTask 3.1: 在 `src/components/` 创建 `OpenClawVersionChecker.tsx` 组件
  - [ ] SubTask 3.2: 实现版本检查按钮和状态显示
  - [ ] SubTask 3.3: 添加版本信息展示 UI（版本号、路径、状态）
  - [ ] SubTask 3.4: 添加未安装状态的提示 UI

- [ ] Task 4: 集成到主界面
  - [ ] SubTask 4.1: 在 `App.tsx` 或 Dashboard 页面中添加版本检查组件
  - [ ] SubTask 4.2: 测试前后端通信是否正常

# Task Dependencies
- Task 2 依赖 Task 1
- Task 3 依赖 Task 2
- Task 4 依赖 Task 3
