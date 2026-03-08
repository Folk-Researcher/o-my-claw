# Checklist

- [ ] OpenClaw 版本检测服务代码实现完成
  - [ ] `src-tauri/src/services/openclaw.rs` 文件存在
  - [ ] `check_openclaw_installation()` 函数能正确检测 OpenClaw 是否安装
  - [ ] `get_openclaw_version()` 函数能正确获取版本号
  - [ ] `get_openclaw_path()` 函数能正确获取安装路径
  - [ ] `OpenClawVersionInfo` 结构体定义正确

- [ ] Tauri 命令实现完成
  - [ ] `src-tauri/src/commands/version.rs` 文件存在
  - [ ] `check_openclaw_version` 命令能正确调用服务
  - [ ] 命令在 `main.rs` 中正确注册

- [ ] 前端版本检查组件实现完成
  - [ ] `src/components/OpenClawVersionChecker.tsx` 文件存在
  - [ ] 组件能正确调用后端命令
  - [ ] 版本信息展示 UI 正常显示
  - [ ] 未安装状态的提示 UI 正常显示

- [ ] 功能集成完成
  - [ ] 版本检查组件已添加到主界面
  - [ ] 前后端通信测试通过
  - [ ] 在 Tauri 桌面窗口中能正常工作
